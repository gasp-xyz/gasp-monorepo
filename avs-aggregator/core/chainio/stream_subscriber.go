// Package etherstream provides event streams from the blockchain.
package chainio

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"sort"
	"time"
	"math/big"

	// lots of poor naming in go-ethereum 👾
	"github.com/ethereum/go-ethereum"
	// "github.com/ethereum/go-ethereum/accounts/abi"
	// ether "github.com/ethereum/go-ethereum/common"
	chain "github.com/ethereum/go-ethereum/core/types"
)

// Reader configures the blockchain connectivity.
//
// Note that users need WebSockets when calling ethclient.DialContext, because
// subscriptions won't work with regular HTTP RPC.
type StreamReader struct {
	Backend interface{
		ethereum.LogFilterer
		ethereum.ChainReader
		ethereum.BlockNumberReader
	} // blockchain connection

	// limit for a subscription request (defaults to 2 s)
	SubscribeTimeout time.Duration
	// limit for history retreival (defaults to 7 s)
	FetchTimeout time.Duration

	// idle time on which no new content can be assumed
	// (defaults to 500 ms)
	ReceiveExpire time.Duration
	filterLimit uint64
}

// // EventsWithHistory resolves all logs matching eventType.
// // The history is sorted in ascending order. The first receive from stream
// // directly follows the last entry from history, if any.
// func (r Reader) EventsWithHistory(ctx context.Context, eventType *abi.Event) (stream <-chan chain.Log, _ ethereum.Subscription, history []chain.Log, _ error) {
// 	// first topic always is the signature hash of the respective event
// 	return r.QueryWithHistory(ctx, &ethereum.FilterQuery{Topics: [][]ether.Hash{{eventType.ID}}})
// }

// QueryWithHistory resolves all logs matching q.
// The history is sorted in ascending order. The first receive from stream
// directly follows the last entry from history, if any.
func (r StreamReader) QueryWithHistory(ctx context.Context, q *ethereum.FilterQuery) (stream <-chan chain.Log, _ ethereum.Subscription, history []chain.Log, _ error) {
	// limited retry on chain-reorganisation [errNoOverlap]
	const tryMax = 2
	for tryN := 1; tryN <= tryMax; tryN++ {
		// subscribe live stream
		timeout := r.SubscribeTimeout
		if timeout == 0 {
			timeout = 2 * time.Second
		}
		stream := make(chan chain.Log, 60)
		sub, err := r.Backend.SubscribeFilterLogs(ctx, *q, stream)
		if err != nil {
			return nil, nil, nil, fmt.Errorf("etherstream: no subscription on log events: %w", err)
		}
		// ⚠️ must Unsubscribe

		history, err := r.linkHistory(ctx, stream, q)
		switch {
		case errors.Is(err, errNoOverlap):
			sub.Unsubscribe()
			continue

		case err != nil:
			sub.Unsubscribe()
			return nil, nil, nil, err
		}

		return stream, sub, history, nil // OK
	}

	return nil, nil, nil, fmt.Errorf("etherstream: give up after %d tries: %w ", tryMax, errNoOverlap)
}

var errNoOverlap = errors.New("historic events don't match [overlap] with subscription reception—chain reorganisation assumed")

// LinkHistory returns the full history before the next entry from stream.
func (r *StreamReader) linkHistory(ctx context.Context, stream <-chan chain.Log, q *ethereum.FilterQuery) ([]chain.Log, error) {
	// fetch history
	timeout := r.FetchTimeout
	if timeout == 0 {
		timeout = 7 * time.Second
	}
	ctx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()
	history, err := r.GetLogs(ctx, q)
	if err != nil {
		return nil, fmt.Errorf("etherstream: historic logs unavailable: %w", err)
	}
	if len(history) == 0 {
		return nil, nil
	}
	// lookup not sorted
	sort.Slice(history, func(i, j int) bool { return Order(&history[i], &history[j]) > 0 })

	receiveExpire := r.ReceiveExpire
	if receiveExpire == 0 {
		receiveExpire = time.Second / 2
	}
	expireTimer := time.NewTimer(receiveExpire)
	defer expireTimer.Stop()

	// find overlap in history to ensure no gaps nor duplicates
	// workaround <https://github.com/ethereum/go-ethereum/issues/15063>
	select {
	case first := <-stream:
		cmp := func(i int) int { return Order(&history[i], &first) }
		i, ok := sort.Find(len(history), cmp)
		if ok {
			return history[:i+1], nil
		}
		return nil, errNoOverlap
	case <-expireTimer.C:
		// no reception after history retreival implies no overlap
		return history, nil
	}
}

// Order returns whether b follows a, with positive for yes, zero for equal, or
// negative for no.
func Order(a, b *chain.Log) int {
	diff := int(b.BlockNumber - a.BlockNumber)
	if diff == 0 {
		diff = int(b.TxIndex - a.TxIndex)
	}
	if diff == 0 && a.TxHash != b.TxHash {
		// a and b are on differd chains
		// use an arbitrary but consistent order
		diff = bytes.Compare(b.TxHash[:], a.TxHash[:])
	}
	return diff
}

func (r StreamReader) GetLogs(ctx context.Context, q *ethereum.FilterQuery) ([]chain.Log, error) {
	if r.filterLimit == 0{
		r.filterLimit = 5000
	}

	var c uint64
	var err error
	if q.ToBlock == nil {
		c, err = r.Backend.BlockNumber(ctx)
		if err != nil {
			return nil, fmt.Errorf("etherstream: BlockNumber failed: %w", err)
		}
	} else {
		if q.ToBlock.Sign() >= 0 {
			c = uint64(q.ToBlock.Int64())
		} else {
			h, err := r.Backend.HeaderByNumber(ctx, q.ToBlock)
			if err != nil {
				return nil, fmt.Errorf("etherstream: HeaderByNumber failed: %w", err)
			}
			c = uint64(h.Number.Int64())
		}
	}

	var s uint64
	if q.FromBlock == nil {
		s = 0
	} else {
		s = uint64(q.FromBlock.Int64())
	}

	var acc []chain.Log
	if c == 0{
		return acc, nil
	}
	if s>c{
		return acc, nil
	}

	iq := *q
	iq.FromBlock = big.NewInt(int64(s))
	lim := uint64(iq.FromBlock.Int64()) + r.filterLimit - 1
	if c <= lim {
		iq.ToBlock = big.NewInt(int64(c))
	} else {
		iq.ToBlock = big.NewInt(int64(lim))
	} 
	
	for {
		logs, err := r.Backend.FilterLogs(ctx, iq)
		if err != nil {
			return nil, fmt.Errorf("etherstream: GetLogs failed: %w", err)
		}
		acc = append(acc, logs...)
		if uint64(iq.ToBlock.Int64()) == c{
			break
		}
		f := uint64(iq.ToBlock.Int64()) + 1
		iq.FromBlock = big.NewInt(int64(f))
		lim := uint64(iq.FromBlock.Int64()) + r.filterLimit - 1
		if c <= lim {
			iq.ToBlock = big.NewInt(int64(c))
		} else {
			iq.ToBlock = big.NewInt(int64(lim))
		} 
	}
	return acc, nil
}

func (r StreamReader) StreamQueryWithHistory(ctx context.Context, q *ethereum.FilterQuery) (chan chain.Log, ethereum.Subscription, error) {
	stream, sub, history, err := r.QueryWithHistory(ctx, q)
	if err != nil {
		return nil, nil, fmt.Errorf("StreamQueryWithHistory at QueryWithHistory failed, err: %v", err)
	}

	sink := make(chan chain.Log)

	go func() {
		for _, historyEvent := range history{
			select{
			case sink <- historyEvent:
			case <-ctx.Done():
				return
			case <-sub.Err():
				return
			}

		}
		for {
			select{
			case sink <- <-stream:
			case <-ctx.Done():
				return
			case <-sub.Err():
				return
			}
		}
	}()
	return sink, sub, nil
}
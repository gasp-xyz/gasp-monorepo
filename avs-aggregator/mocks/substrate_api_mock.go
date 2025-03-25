package mocks

import (
	gsrpcrpcauthormocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/author/mocks"
	gsrpcrpcbeefymocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/beefy/mocks"
	gsrpcrpcchainmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/chain/mocks"
	gsrpcrpcmmrmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/mmr/mocks"
	gsrpcrpcoffchainmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/offchain/mocks"
	gsrpcrpcstatemocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/state/mocks"
	gsrpcrpcsystemmocks "github.com/centrifuge/go-substrate-rpc-client/v4/rpc/system/mocks"
	gsrpcrpc "github.com/centrifuge/go-substrate-rpc-client/v4/rpc"
	gsrpcclientmocks "github.com/centrifuge/go-substrate-rpc-client/v4/client/mocks"
	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	gsrpctypes "github.com/centrifuge/go-substrate-rpc-client/v4/types"
	gsrpctypescodec "github.com/centrifuge/go-substrate-rpc-client/v4/types/codec"
)

func GetFakeSubstrateMetadata() (gsrpctypes.Metadata, error) {
	// Decode the metadata
	var metadata gsrpctypes.Metadata
	err := gsrpctypescodec.DecodeFromHex(GaspMetadata, &metadata)
	return metadata, err
}

func NewMockSubstrateAPI() (*gsrpc.SubstrateAPI, error) {
	return &gsrpc.SubstrateAPI{
		RPC:     &gsrpcrpc.RPC{
			Author:   &gsrpcrpcauthormocks.Author{},
			Beefy:    &gsrpcrpcbeefymocks.Beefy{},
			Chain:    &gsrpcrpcchainmocks.Chain{},
			MMR:      &gsrpcrpcmmrmocks.MMR{},
			Offchain: &gsrpcrpcoffchainmocks.Offchain{},
			State:    &gsrpcrpcstatemocks.State{},
			System:   &gsrpcrpcsystemmocks.System{},
		},
		Client:  &gsrpcclientmocks.Client{},
	}, nil
}
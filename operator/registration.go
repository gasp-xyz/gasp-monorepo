package operator

// OUTDATED
// This file contains cli functions for registering an operator with the AVS and printing status
// However, all of this functionality has been moved to the plugin/ package
// we are just waiting for eigenlayer-cli to be open sourced so we can completely get rid of this registration functionality in the operator

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math/big"

	blspubkeyregistry "github.com/Layr-Labs/eigensdk-go/contracts/bindings/BLSPubkeyRegistry"
	regcoord "github.com/Layr-Labs/eigensdk-go/contracts/bindings/BLSRegistryCoordinatorWithIndices"
	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	eigenSdkTypes "github.com/Layr-Labs/eigensdk-go/types"
)

func (o *Operator) RegisterAtStartup() {
	err := o.RegisterOperatorWithEigen()
	if err != nil {
		o.logger.Error("Error while registering operator into eigen")
	}

	err = o.RegisterOperatorWithAvs()
	if err != nil {
		o.logger.Error("Error while registering operator into AVS")
	}
	o.logger.Info("Operator succesfully registered!")
}

func (o *Operator) RegisterOperatorWithEigen() error {
	op := eigenSdkTypes.Operator{
		Address:                 o.operatorAddr.String(),
		EarningsReceiverAddress: o.operatorAddr.String(),
	}

	status, err := o.ethRpc.ElReader.IsOperatorRegistered(context.Background(), op)
	if err != nil {
		return err
	}

	if !status {
		receipt, err := o.ethRpc.ElWriter.RegisterAsOperator(context.Background(), op)
		if err != nil {
			o.logger.Info("Error while registering operator")
			return err
		}
		o.logger.Infof(
			"Operator registration transaction at: %s",
			receipt.TxHash.String(),
		)

	} else {
		o.logger.Info("Operator is already registered on EigenLayer")
	}

	receipt, err := o.ethRpc.ElWriter.RegisterBLSPublicKey(context.Background(), o.config.BlsKeyPair, op)
	if err != nil {
		o.logger.Info("Error while registering BLS public key")
		return err
	}
	o.logger.Infof(
		"Operator bls key added transaction at: %s",
		receipt.TxHash.String(),
	)

	o.logger.Info("Operator is registered and bls key added successfully")
	return nil
}

func (o *Operator) RegisterOperatorWithAvs() error {
	quorumNumbers := []byte{0}
	socket := "Not Needed"

	pubkey := pubKeyG1ToBN254G1Point(o.config.BlsKeyPair.GetPubKeyG1())

	_, err := o.ethRpc.AvsWriter.RegisterOperatorWithAVSRegistryCoordinator(context.Background(), quorumNumbers, pubkey, socket)
	if err != nil {
		o.logger.Errorf("Unable to register operator with avs registry coordinator")
		return err
	}
	o.logger.Infof("Registered operator with avs registry coordinator.")

	return nil
}

func (o *Operator) DeregisterOperatorWithAvs() error {
	quorumNumbers := []byte{0}

	p := pubKeyG1ToBN254G1Point(o.config.BlsKeyPair.GetPubKeyG1())
	pubkey := blspubkeyregistry.BN254G1Point{
		X: p.X,
		Y: p.Y,
	}

	_, err := o.ethRpc.AvsWriter.DeregisterOperator(context.Background(), o.operatorAddr, quorumNumbers, pubkey)
	if err != nil {
		o.logger.Errorf("Unable to deregister operator with avs registry coordinator")
		return err
	}
	o.logger.Infof("Deregistered operator with avs registry coordinator.")

	return nil
}

// PRINTING STATUS OF OPERATOR: 1
// operator address: 0xa0ee7a142d267c1f36714e4a8f75612f20a79720
// dummy token balance: 0
// delegated shares in dummyTokenStrat: 200
// operator pubkey hash in AVS pubkey compendium (0 if not registered): 0x4b7b8243d970ff1c90a7c775c008baad825893ec6e806dfa5d3663dc093ed17f
// operator is opted in to eigenlayer: true
// operator is opted in to playgroundAVS (aka can be slashed): true
// operator status in AVS registry: REGISTERED
//
//	operatorId: 0x4b7b8243d970ff1c90a7c775c008baad825893ec6e806dfa5d3663dc093ed17f
//	middlewareTimesLen (# of stake updates): 0
//
// operator is frozen: false
type OperatorStatus struct {
	EcdsaAddress string
	// pubkey compendium related
	PubkeysRegistered bool
	G1Pubkey          string
	G2Pubkey          string
	// avs related
	OptedIntoSlashingByAvs bool
	RegisteredWithAvs      bool
	OperatorId             string
	Frozen                 bool
}

func (o *Operator) PrintOperatorStatus() error {
	fmt.Println("Printing operator status")
	pubkeyhash, err := o.ethRpc.ElReader.GetOperatorPubkeyHash(context.Background(), eigenSdkTypes.Operator{Address: o.operatorAddr.String()})
	if err != nil {
		return err
	}
	pubkeysRegistered := pubkeyhash != [32]byte{}
	serviceManagerCanSlashOperatorUntilBlock, err := o.ethRpc.ElReader.ServiceManagerCanSlashOperatorUntilBlock(
		context.Background(), o.operatorAddr, o.config.ServiceManagerAddr,
	)
	if err != nil {
		return err
	}
	curBlockNumber, err := o.ethRpc.Client.BlockNumber(context.Background())
	if err != nil {
		return err
	}
	optedIntoSlashingByAvs := curBlockNumber < uint64(serviceManagerCanSlashOperatorUntilBlock)
	registeredWithAvs := o.operatorId != [32]byte{}
	isFrozen, err := o.ethRpc.ElReader.OperatorIsFrozen(context.Background(), o.operatorAddr)
	if err != nil {
		return err
	}

	operatorStatus := OperatorStatus{
		EcdsaAddress:           o.operatorAddr.String(),
		PubkeysRegistered:      pubkeysRegistered,
		G1Pubkey:               o.config.BlsKeyPair.GetPubKeyG1().String(),
		G2Pubkey:               o.config.BlsKeyPair.GetPubKeyG2().String(),
		OptedIntoSlashingByAvs: optedIntoSlashingByAvs,
		RegisteredWithAvs:      registeredWithAvs,
		OperatorId:             hex.EncodeToString(o.operatorId[:]),
		Frozen:                 isFrozen,
	}
	operatorStatusJson, err := json.MarshalIndent(operatorStatus, "", " ")
	if err != nil {
		return err
	}
	fmt.Println(string(operatorStatusJson))
	return nil
}

func pubKeyG1ToBN254G1Point(p *bls.G1Point) regcoord.BN254G1Point {
	return regcoord.BN254G1Point{
		X: p.X.BigInt(new(big.Int)),
		Y: p.Y.BigInt(new(big.Int)),
	}
}

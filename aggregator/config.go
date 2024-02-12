package aggregator

import (
	"errors"
	"math/big"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signerv2"
	"github.com/ethereum/go-ethereum/accounts/keystore"
	"github.com/ethereum/go-ethereum/common"
	"github.com/mangata-finance/eigen-layer-monorepo/aggregator/core/config"
	"github.com/urfave/cli"
)

type Config struct {
	LogLevel          sdklogging.LogLevel
	EthRpcUrl         string
	EthWsUrl          string
	ChainId           *big.Int
	SubstrateWsRpcUrl string

	ServerAddressPort string
	BlockPeriod       int

	BlsOperatorStateRetrieverAddr common.Address
	BlsCompendiumAddr             common.Address
	ServiceManagerAddr            common.Address

	SignerFn signerv2.SignerFn
	Address  common.Address

	KickPeriod int
}

// NewConfig parses the Config from the provided flags or environment variables and
// returns a Config.
func NewConfig(ctx *cli.Context) (*Config, error) {
	var logLevel = sdklogging.Development
	if ctx.GlobalString(config.EnvironmentFlag.Name) == string(sdklogging.Production) {
		logLevel = sdklogging.Production
	}

	chainId := big.NewInt(int64(ctx.GlobalUint(config.ChainIdFlag.Name)))

	var signer signerv2.SignerFn
	var address common.Address
	keyStoreFile := ctx.GlobalString(config.EcdsaKeyFileFlag.Name)
	if keyStoreFile == "" {
		keyStoreContents := ctx.GlobalString(config.EcdsaKeyJsonFlag.Name)
		if keyStoreContents == "" {
			return nil, errors.New("one of --ecdsa-key-file or --ecdsa-key-json must be set")
		}
		sk, err := keystore.DecryptKey([]byte(keyStoreContents), ctx.GlobalString(config.EcdsaKeyPasswordFlag.Name))
		if err != nil {
			return nil, err
		}
		signer, address, err = signerv2.SignerFromConfig(signerv2.Config{
			PrivateKey: sk.PrivateKey,
		},
			chainId,
		)
		if err != nil {
			return nil, err
		}
	} else {
		var err error
		signer, address, err = signerv2.SignerFromConfig(signerv2.Config{
			KeystorePath: ctx.GlobalString(config.EcdsaKeyFileFlag.Name),
			Password:     ctx.GlobalString(config.EcdsaKeyPasswordFlag.Name),
		},
			chainId,
		)
		if err != nil {
			return nil, err
		}
	}

	return &Config{
		LogLevel:                      logLevel,
		ServerAddressPort:             ctx.GlobalString(config.AvsServerPortAddressFlag.Name),
		BlockPeriod:                   ctx.GlobalInt(config.AvsBlockValidationPeriodFlag.Name),
		KickPeriod:                    ctx.GlobalInt(config.AvsKickPeriodFlag.Name),
		EthRpcUrl:                     ctx.GlobalString(config.EthRpcFlag.Name),
		EthWsUrl:                      ctx.GlobalString(config.EthWsFlag.Name),
		ChainId:                       chainId,
		SubstrateWsRpcUrl:             ctx.GlobalString(config.SubstrateRpcFlag.Name),
		BlsOperatorStateRetrieverAddr: common.HexToAddress(ctx.GlobalString(config.BlsOperatorStateRetrieverFlag.Name)),
		BlsCompendiumAddr:             common.HexToAddress(ctx.GlobalString(config.BlsCompendiumFlag.Name)),
		ServiceManagerAddr:            common.HexToAddress(ctx.GlobalString(config.AvsServiceManagerFlag.Name)),
		SignerFn:                      signer,
		Address:                       address,
	}, nil
}

var Flags = []cli.Flag{
	config.EnvironmentFlag,
	config.EthRpcFlag,
	config.EthWsFlag,
	config.SubstrateRpcFlag,
	config.AvsServerPortAddressFlag,
	config.ChainIdFlag,
	config.BlsCompendiumFlag,
	config.BlsOperatorStateRetrieverFlag,
	config.AvsServiceManagerFlag,
	config.EcdsaKeyFileFlag,
	config.EcdsaKeyJsonFlag,
	config.EcdsaKeyPasswordFlag,
	config.AvsBlockValidationPeriodFlag,
	config.AvsKickPeriodFlag,
}

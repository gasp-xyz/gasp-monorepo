package aggregator

import (
	"math/big"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	"github.com/ethereum/go-ethereum/common"
	"github.com/mangata-finance/eigen-layer-monorepo/core/config"
	"github.com/urfave/cli"
)

type Config struct {
	LogLevel          sdklogging.LogLevel
	EthRpcUrl         string
	EthWsUrl          string
	ChainId           *big.Int
	SubstrateWsRpcUrl string
	
	ServerAddressPort string

	BlsOperatorStateRetrieverAddr common.Address
	BlsCompendiumAddr             common.Address
	ServiceManagerAddr            common.Address

	EcdsaSigner signer.Signer
}

// NewConfig parses the Config from the provided flags or environment variables and
// returns a Config.
func NewConfig(ctx *cli.Context) (*Config, error) {
	var logLevel = sdklogging.Development
	if ctx.GlobalString(config.EnvironmentFlag.Name) == string(sdklogging.Production) {
		logLevel = sdklogging.Production
	}

	chainId := big.NewInt(int64(ctx.GlobalUint(config.ChainIdFlag.Name)))

	signer, err := signer.NewPrivateKeyFromKeystoreSigner(
		ctx.GlobalString(config.EcdsaKeyFileFlag.Name),
		ctx.GlobalString(config.EcdsaKeyPasswordFlag.Name),
		chainId,
	)
	if err != nil {
		return nil, err
	}

	return &Config{
		LogLevel:                      logLevel,
		ServerAddressPort:             ctx.GlobalString(config.AvsServerPortAddressFlag.Name),
		EthRpcUrl:                     ctx.GlobalString(config.EthRpcFlag.Name),
		EthWsUrl:                      ctx.GlobalString(config.EthWsFlag.Name),
		ChainId:                       chainId,
		SubstrateWsRpcUrl:             ctx.GlobalString(config.SubstrateRpcFlag.Name),
		BlsOperatorStateRetrieverAddr: common.HexToAddress(ctx.GlobalString(config.BlsOperatorStateRetrieverFlag.Name)),
		BlsCompendiumAddr:             common.HexToAddress(ctx.GlobalString(config.BlsCompendiumFlag.Name)),
		ServiceManagerAddr:            common.HexToAddress(ctx.GlobalString(config.AvsServiceManagerFlag.Name)),
		EcdsaSigner:                   signer,
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
	config.EcdsaKeyPasswordFlag,
}

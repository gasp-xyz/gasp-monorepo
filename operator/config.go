package operator

import (
	"math/big"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signerv2"
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
	AggregatorUrl     string

	EigenMetricsIpPortAddress string
	NodeApiIpPortAddress      string
	EnableMetrics             bool
	EnableNodeApi             bool

	BlsOperatorStateRetrieverAddr common.Address
	BlsCompendiumAddr             common.Address
	ServiceManagerAddr            common.Address

	BlsKeyPair *bls.KeyPair      `json:"-"`
	SignerFn   signerv2.SignerFn `json:"-"`
	Address    common.Address

	RegisterAtStartup bool
}

// NewConfig parses the Config from the provided flags or environment variables and
// returns a Config.
func NewConfig(ctx *cli.Context) (*Config, error) {
	var logLevel = sdklogging.Development
	if ctx.GlobalString(config.EnvironmentFlag.Name) == string(sdklogging.Production) {
		logLevel = sdklogging.Production
	}

	chainId := big.NewInt(int64(ctx.GlobalUint(config.ChainIdFlag.Name)))

	blsKeypair, err := bls.ReadPrivateKeyFromFile(
		ctx.GlobalString(config.BlsKeyFileFlag.Name),
		ctx.GlobalString(config.BlsKeyPasswordFlag.Name),
	)
	if err != nil {
		return nil, err
	}

	signer, address, err := signerv2.SignerFromConfig(signerv2.Config{
		KeystorePath: ctx.GlobalString(config.EcdsaKeyFileFlag.Name),
		Password:     ctx.GlobalString(config.EcdsaKeyPasswordFlag.Name),
	},
		chainId,
	)
	if err != nil {
		return nil, err
	}

	return &Config{
		LogLevel:                      logLevel,
		EthRpcUrl:                     ctx.GlobalString(config.EthRpcFlag.Name),
		EthWsUrl:                      ctx.GlobalString(config.EthWsFlag.Name),
		ChainId:                       chainId,
		SubstrateWsRpcUrl:             ctx.GlobalString(config.SubstrateRpcFlag.Name),
		AggregatorUrl:                 ctx.GlobalString(config.AggregatorRpcFlag.Name),
		EigenMetricsIpPortAddress:     ctx.GlobalString(config.EigenMetricsIpPortAddress.Name),
		NodeApiIpPortAddress:          ctx.GlobalString(config.NodeApiIpPortAddress.Name),
		EnableMetrics:                 ctx.GlobalBool(config.EnableMetrics.Name),
		EnableNodeApi:                 ctx.GlobalBool(config.EnableNodeApi.Name),
		BlsOperatorStateRetrieverAddr: common.HexToAddress(ctx.GlobalString(config.BlsOperatorStateRetrieverFlag.Name)),
		BlsCompendiumAddr:             common.HexToAddress(ctx.GlobalString(config.BlsCompendiumFlag.Name)),
		ServiceManagerAddr:            common.HexToAddress(ctx.GlobalString(config.AvsServiceManagerFlag.Name)),
		BlsKeyPair:                    blsKeypair,
		SignerFn:                      signer,
		Address:                       address,
		RegisterAtStartup:             ctx.GlobalBool(config.RegisterAtStratupFlag.Name),
	}, nil
}

var Flags = []cli.Flag{
	config.EnvironmentFlag,
	config.EthRpcFlag,
	config.EthWsFlag,
	config.ChainIdFlag,
	config.SubstrateRpcFlag,
	config.AggregatorRpcFlag,
	config.BlsCompendiumFlag,
	config.BlsOperatorStateRetrieverFlag,
	config.AvsServiceManagerFlag,
	config.BlsKeyFileFlag,
	config.BlsKeyPasswordFlag,
	config.EcdsaKeyFileFlag,
	config.EcdsaKeyPasswordFlag,
	config.RegisterAtStratupFlag,
}

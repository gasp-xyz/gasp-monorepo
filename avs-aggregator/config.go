package aggregator

import (
	"errors"
	"math/big"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signerv2"
	"github.com/ethereum/go-ethereum/accounts/keystore"
	"github.com/ethereum/go-ethereum/common"
	"github.com/gasp-xyz/gasp-monorepo/avs-aggregator/core/config"
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
	BlockPeriodOpsTask       int
	Expiration        int
	UpdatePeriod      int
	DebounceRpc       int
	EnableKicker      bool

	AvsRegistryCoordinatorAddr common.Address
	AvsDeploymentBlock         uint64

	SignerFn signerv2.SignerFn `json:"-"`
	Address  common.Address

	KickPeriod int
	MinOpUpdateInterval int

	ReinitOpStateAtInit bool
	CheckTriggerOpStateUpdate bool
	CheckTriggerOpStateUpdateWindow bool

	AggIdleStart bool
	AggRunTriggerApiKey string

	EnableTraceLogs bool
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
		LogLevel:                   logLevel,
		ServerAddressPort:          ctx.GlobalString(config.AvsServerPortAddressFlag.Name),
		BlockPeriod:                ctx.GlobalInt(config.AvsBlockValidationPeriodFlag.Name),
		BlockPeriodOpsTask:                ctx.GlobalInt(config.AvsOpTaskPeriodFlag.Name),
		Expiration:                 ctx.GlobalInt(config.AvsTaskExpirationFlag.Name),
		KickPeriod:                 ctx.GlobalInt(config.AvsKickPeriodFlag.Name),
		MinOpUpdateInterval:            ctx.GlobalInt(config.AvsMinOpUpdateInterval.Name),
		UpdatePeriod:               ctx.GlobalInt(config.AvsUpdateStakePeriodFlag.Name),
		DebounceRpc:                ctx.GlobalInt(config.AvsDebounceRpcFlag.Name),
		EnableKicker:                ctx.GlobalBool(config.AvsEnableKickerFlag.Name),
		EthRpcUrl:                  ctx.GlobalString(config.EthRpcFlag.Name),
		EthWsUrl:                   ctx.GlobalString(config.EthWsFlag.Name),
		ChainId:                    chainId,
		SubstrateWsRpcUrl:          ctx.GlobalString(config.SubstrateRpcFlag.Name),
		AvsRegistryCoordinatorAddr: common.HexToAddress(ctx.GlobalString(config.AvsRegistryCoordinatorFlag.Name)),
		AvsDeploymentBlock:         uint64(ctx.GlobalInt(config.AvsDeploymentBlockFlag.Name)),
		SignerFn:                   signer,
		Address:                    address,
		ReinitOpStateAtInit:  ctx.GlobalBool(config.AggOsuReinitOpStateAtInit.Name),
		CheckTriggerOpStateUpdate:  ctx.GlobalBool(config.AggOsuCheckTriggerOpStateUpdate.Name),
		CheckTriggerOpStateUpdateWindow:  ctx.GlobalBool(config.AggOsuCheckTriggerOpStateUpdateWindow.Name),
		AggIdleStart: ctx.GlobalBool(config.AggIdleStart.Name),
		AggRunTriggerApiKey: ctx.GlobalString(config.AggRunTriggerApiKey.Name),
		EnableTraceLogs: ctx.GlobalBool(config.EnableTraceLogs.Name),
	}, nil
}

var Flags = []cli.Flag{
	config.EnvironmentFlag,
	config.EthRpcFlag,
	config.EthWsFlag,
	config.SubstrateRpcFlag,
	config.AvsServerPortAddressFlag,
	config.ChainIdFlag,
	config.AvsRegistryCoordinatorFlag,
	config.AvsDeploymentBlockFlag,
	config.EcdsaKeyFileFlag,
	config.EcdsaKeyJsonFlag,
	config.EcdsaKeyPasswordFlag,
	config.AvsBlockValidationPeriodFlag,
	config.AvsOpTaskPeriodFlag,
	config.AvsKickPeriodFlag,
	config.AvsMinOpUpdateInterval,
	config.AvsUpdateStakePeriodFlag,
	config.AvsTaskExpirationFlag,
	config.AvsDebounceRpcFlag,
	config.AvsEnableKickerFlag,
	config.AggOsuReinitOpStateAtInit,
	config.AggOsuCheckTriggerOpStateUpdate,
	config.AggOsuCheckTriggerOpStateUpdateWindow,
	config.AggIdleStart,
	config.AggRunTriggerApiKey,
	config.EnableTraceLogs,
}

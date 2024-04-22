package config

import (
	"github.com/urfave/cli"
)

var (
	EnvironmentFlag = cli.StringFlag{
		Name:     "environment",
		Usage:    "log level, 'production' only prints info and above. 'development' also prints debug",
		Required: false,
		Value:    "development",
		EnvVar:   "ENVIRONMEMT",
	}
	EthRpcFlag = cli.StringFlag{
		Name:     "eth-rpc-url",
		Usage:    "eth chain RPC url",
		Required: true,
		EnvVar:   "ETH_RPC_URL",
	}
	EthWsFlag = cli.StringFlag{
		Name:     "eth-ws-url",
		Usage:    "eth chain RPC websocket url",
		Required: true,
		EnvVar:   "ETH_WS_URL",
	}
	ChainIdFlag = cli.UintFlag{
		Name:     "chain-id",
		Usage:    "eth chain id",
		Required: true,
		EnvVar:   "CHAIN_ID",
	}
	SubstrateRpcFlag = cli.StringFlag{
		Name:     "substrate-rpc-url",
		Usage:    "substrate chain RPC url",
		Required: true,
		EnvVar:   "SUBSTRATE_RPC_URL",
	}

	// AVS
	AvsServerPortAddressFlag = cli.StringFlag{
		Name:     "avs-server-ip-port-address",
		Usage:    "Port at which avs server listens for operator calls",
		Required: true,
		EnvVar:   "AVS_SERVER_IP_PORT_ADDRESS",
	}
	AvsBlockValidationPeriodFlag = cli.IntFlag{
		Name:     "avs-block-validation-period",
		Usage:    "Period of block finalization per block produced on L2",
		Required: false,
		Value:    4,
		EnvVar:   "AVS_BLOCK_VALIDATION_PERIOD",
	}
	AvsTaskExpirationFlag = cli.IntFlag{
		Name:     "avs-task-expiration",
		Usage:    "Expiration of task in seconds",
		Required: false,
		Value:    30,
		EnvVar:   "AVS_TASK_EXPIRATION",
	}
	AvsKickPeriodFlag = cli.IntFlag{
		Name:     "avs-kick-period",
		Usage:    "Period of active OPs check per tasks created",
		Required: false,
		Value:    50,
		EnvVar:   "AVS_KICK_PERIOD",
	}
	AvsUpdateStakePeriodFlag = cli.IntFlag{
		Name:     "avs-update-stake-period",
		Usage:    "Period of running update stakes for operators",
		Required: false,
		Value:    50,
		EnvVar:   "AVS_UPDATE_STAKE_PERIOD",
	}

	// Deployment
	AvsRegistryCoordinatorFlag = cli.StringFlag{
		Name:     "avs-registry-coordinator-addr",
		Usage:    "Address of the AVS Registry Coordinator",
		Required: true,
		EnvVar:   "AVS_REGISTRY_COORDINATOR_ADDR",
	}
	AvsDeploymentBlockFlag = cli.IntFlag{
		Name:     "avs-deployment-block",
		Usage:    "block number at which AVS contracts were deployed, used for startBlock event filtering",
		Required: false,
		Value:    0,
		EnvVar:   "AVS_DEPLOYMENT_BLOCK",
	}

	// The files for encrypted private keys.
	EcdsaKeyFileFlag = cli.StringFlag{
		Name:     "ecdsa-key-file",
		Required: false,
		Usage:    "Path to the encrypted ecdsa private key",
		Value:    "",
		EnvVar:   "ECDSA_KEY_FILE",
	}
	EcdsaKeyJsonFlag = cli.StringFlag{
		Name:     "ecdsa-key-json",
		Required: false,
		Usage:    "Encrypted ecdsa private key json",
		Value:    "",
		EnvVar:   "ECDSA_KEY_JSON",
	}
	// Passwords to decrypt the private keys.
	EcdsaKeyPasswordFlag = cli.StringFlag{
		Name:     "ecdsa-key-password",
		Required: false,
		Value:    "",
		Usage:    "Password to decrypt ecdsa private key",
		EnvVar:   "ECDSA_KEY_PASSWORD",
	}
)

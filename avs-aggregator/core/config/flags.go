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
		Usage:    "Period of block finalization per block produced on mangata",
		Required: false,
		Value:    2,
		EnvVar:   "AVS_BLOCK_VALIDATION_PERIOD",
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
	BlsOperatorStateRetrieverFlag = cli.StringFlag{
		Name:     "bls-operator-state-retriever-addr",
		Usage:    "Address of the BLS Operator State Retriever",
		Required: true,
		EnvVar:   "BLS_OPERATOR_STATE_RETRIEVER_ADDR",
	}
	BlsCompendiumFlag = cli.StringFlag{
		Name:     "bls-compendium-addr",
		Usage:    "Address of the BLS Public Key Compendium",
		Required: true,
		EnvVar:   "BLS_COMPENDIUM_ADDR",
	}
	AvsServiceManagerFlag = cli.StringFlag{
		Name:     "avs-service-manager-addr",
		Usage:    "Address of the AVS Service Manager",
		Required: true,
		EnvVar:   "AVS_SERVICE_MANAGER_ADDR",
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

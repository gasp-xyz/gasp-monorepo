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
	AggregatorRpcFlag = cli.StringFlag{
		Name:     "aggregator-rpc-url",
		Usage:    "aggregator RPC url",
		Required: true,
		EnvVar:   "AGGREGATOR_RPC_URL",
	}

	// Operator
	RegisterAtStratupFlag = cli.StringFlag{
		Name:     "register-at-startup",
		Usage:    "run register plugin at operator startup",
		Required: false,
		EnvVar:   "REGISTER_AT_STARTUP",
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

	// metrics
	EigenMetricsIpPortAddress = cli.StringFlag{
		Name:     "eigen-metrics-ip-port-address",
		Usage:    "Port at which eigen-metrics listens for calls",
		Required: false,
		Value:    "",
		EnvVar:   "EIGEN_METRICS_IP_PORT_ADDRESS",
	}
	NodeApiIpPortAddress = cli.StringFlag{
		Name:     "node-api-ip-port-address",
		Usage:    "Port at which node-api- listens for calls",
		Required: false,
		Value:    "",
		EnvVar:   "NODE_API_IP_PORT_ADDRESS",
	}
	EnableMetrics = cli.BoolFlag{
		Name:     "enable-metrics",
		Usage:    "enable metrics",
		Required: false,
		EnvVar:   "ENABLE_METRICS",
	}
	EnableNodeApi = cli.BoolFlag{
		Name:     "enable-node-api",
		Usage:    "enable metrics",
		Required: false,
		EnvVar:   "ENABLE_NODE_API",
	}

	// Deployment
	BlsOperatorStateRetrieverFlag = cli.StringFlag{
		Name:     "bls-operator-state-retriever",
		Usage:    "Address of the BLS Operator State Retriever",
		Required: true,
		EnvVar:   "BLS_OPERATOR_STATE_RETRIEVER",
	}
	BlsCompendiumFlag = cli.StringFlag{
		Name:     "bls-public-key-compendium",
		Usage:    "Address of the BLS Public Key Compendium",
		Required: true,
		EnvVar:   "BLS_PUBLIC_KEY_COMPENDIUM",
	}
	AvsServiceManagerFlag = cli.StringFlag{
		Name:     "avs-service-manager",
		Usage:    "Address of the AVS Service Manager",
		Required: true,
		EnvVar:   "AVS_SERVICE_MANAGER",
	}

	// The files for encrypted private keys.
	BlsKeyFileFlag = cli.StringFlag{
		Name:     "bls-key-file",
		Required: false,
		Usage:    "Path to the encrypted bls private key",
		Value:    "/app/operator_keys/bls_key.json",
		EnvVar:   "BLS_KEY_FILE",
	}
	EcdsaKeyFileFlag = cli.StringFlag{
		Name:     "ecdsa-key-file",
		Required: false,
		Usage:    "Path to the encrypted ecdsa private key",
		Value:    "/app/operator_keys/ecdsa_key.json",
		EnvVar:   "ECDSA_KEY_FILE",
	}
	// Passwords to decrypt the private keys.
	BlsKeyPasswordFlag = cli.StringFlag{
		Name:     "bls-key-password",
		Required: false,
		Value:    "",
		Usage:    "Password to decrypt bls private key",
		EnvVar:   "BLS_KEY_PASSWORD",
	}
	EcdsaKeyPasswordFlag = cli.StringFlag{
		Name:     "ecdsa-key-password",
		Required: false,
		Value:    "",
		Usage:    "Password to decrypt ecdsa private key",
		EnvVar:   "ECDSA_KEY_PASSWORD",
	}
)

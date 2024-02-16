// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package contractFinalizerServiceManager

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// ISignatureUtilsSignatureWithSaltAndExpiry is an auto generated low-level Go binding around an user-defined struct.
type ISignatureUtilsSignatureWithSaltAndExpiry struct {
	Signature []byte
	Salt      [32]byte
	Expiry    *big.Int
}

// ContractFinalizerServiceManagerMetaData contains all meta data concerning the ContractFinalizerServiceManager contract.
var ContractFinalizerServiceManagerMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"internalType\":\"contractIDelegationManager\",\"name\":\"_delegation\",\"type\":\"address\"},{\"internalType\":\"contractIRegistryCoordinator\",\"name\":\"_registryCoordinator\",\"type\":\"address\"},{\"internalType\":\"contractIStakeRegistry\",\"name\":\"_stakeRegistry\",\"type\":\"address\"},{\"internalType\":\"contractIFinalizerTaskManager\",\"name\":\"_taskManager\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\"}],\"name\":\"Initialized\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"}],\"name\":\"deregisterOperatorFromAVS\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"}],\"name\":\"getOperatorRestakedStrategies\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getRestakeableStrategies\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"initialOwner\",\"type\":\"address\"}],\"name\":\"initialize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"}],\"internalType\":\"structISignatureUtils.SignatureWithSaltAndExpiry\",\"name\":\"operatorSignature\",\"type\":\"tuple\"}],\"name\":\"registerOperatorToAVS\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_metadataURI\",\"type\":\"string\"}],\"name\":\"setMetadataURI\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"taskManager\",\"outputs\":[{\"internalType\":\"contractIFinalizerTaskManager\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
	Bin: "0x6101006040523480156200001257600080fd5b506040516200160e3803806200160e83398101604081905262000035916200014f565b6001600160a01b0380851660a052808416608052821660c0528383836200005b62000074565b5050506001600160a01b031660e05250620001b7915050565b600054610100900460ff1615620000e15760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116101562000134576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146200014c57600080fd5b50565b600080600080608085870312156200016657600080fd5b8451620001738162000136565b6020860151909450620001868162000136565b6040860151909350620001998162000136565b6060860151909250620001ac8162000136565b939692955090935050565b60805160a05160c05160e0516113c26200024c6000396000610139015260008181610383015281816104df0152818161057601528181610a3c01528181610bc00152610c5f01526000818161068c0152818161075e01526108320152600081816101ae0152818161023d015281816102bd01528181610701015281816107d60152818161097a0152610b1b01526113c26000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c8063a364f4da11610066578063a364f4da14610121578063a50a640e14610134578063c4d66de81461015b578063e481af9d1461016e578063f2fde38b1461017657600080fd5b806333cfb7b7146100a3578063715018a6146100cc578063750521f5146100d65780638da5cb5b146100e95780639926ee7d1461010e575b600080fd5b6100b66100b1366004610ed5565b610189565b6040516100c39190610ef9565b60405180910390f35b6100d4610659565b005b6100d46100e4366004610ffb565b61066d565b6033546001600160a01b03165b6040516001600160a01b0390911681526020016100c3565b6100d461011c36600461104c565b6106f6565b6100d461012f366004610ed5565b6107cb565b6100f67f000000000000000000000000000000000000000000000000000000000000000081565b6100d4610169366004610ed5565b610861565b6100b6610974565b6100d4610184366004610ed5565b610d3e565b6040516309aa152760e11b81526001600160a01b0382811660048301526060916000917f000000000000000000000000000000000000000000000000000000000000000016906313542a4e90602401602060405180830381865afa1580156101f5573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061021991906110f7565b60405163871ef04960e01b8152600481018290529091506000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063871ef04990602401602060405180830381865afa158015610284573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102a89190611110565b90506001600160c01b038116158061034257507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610319573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061033d9190611139565b60ff16155b1561035e57505060408051600081526020810190915292915050565b6000610372826001600160c01b0316610db7565b90506000805b8251811015610448577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316633ca5a5f58483815181106103c2576103c261115c565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015610406573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061042a91906110f7565b6104349083611188565b915080610440816111a0565b915050610378565b5060008167ffffffffffffffff81111561046457610464610f46565b60405190808252806020026020018201604052801561048d578160200160208202803683370190505b5090506000805b845181101561064c5760008582815181106104b1576104b161115c565b0160200151604051633ca5a5f560e01b815260f89190911c6004820181905291506000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633ca5a5f590602401602060405180830381865afa158015610526573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061054a91906110f7565b905060005b81811015610636576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa1580156105c4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105e891906111bb565b600001518686815181106105fe576105fe61115c565b6001600160a01b039092166020928302919091019091015284610620816111a0565b955050808061062e906111a0565b91505061054f565b5050508080610644906111a0565b915050610494565b5090979650505050505050565b610661610e14565b61066b6000610e6e565b565b610675610e14565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb355906106c1908490600401611287565b600060405180830381600087803b1580156106db57600080fd5b505af11580156106ef573d6000803e3d6000fd5b5050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146107475760405162461bcd60e51b815260040161073e9061129a565b60405180910390fd5b604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d906107959085908590600401611312565b600060405180830381600087803b1580156107af57600080fd5b505af11580156107c3573d6000803e3d6000fd5b505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146108135760405162461bcd60e51b815260040161073e9061129a565b6040516351b27a6d60e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024016106c1565b600054610100900460ff16158080156108815750600054600160ff909116105b8061089b5750303b15801561089b575060005460ff166001145b6108fe5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840161073e565b6000805460ff191660011790558015610921576000805461ff0019166101001790555b61092a82610e6e565b8015610970576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050565b606060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109d6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109fa9190611139565b60ff16905080610a1857505060408051600081526020810190915290565b6000805b82811015610acd57604051633ca5a5f560e01b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015610a8b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610aaf91906110f7565b610ab99083611188565b915080610ac5816111a0565b915050610a1c565b5060008167ffffffffffffffff811115610ae957610ae9610f46565b604051908082528060200260200182016040528015610b12578160200160208202803683370190505b5090506000805b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b77573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b9b9190611139565b60ff16811015610d3457604051633ca5a5f560e01b815260ff821660048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015610c0f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c3391906110f7565b905060005b81811015610d1f576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015610cad573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cd191906111bb565b60000151858581518110610ce757610ce761115c565b6001600160a01b039092166020928302919091019091015283610d09816111a0565b9450508080610d17906111a0565b915050610c38565b50508080610d2c906111a0565b915050610b19565b5090949350505050565b610d46610e14565b6001600160a01b038116610dab5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161073e565b610db481610e6e565b50565b60606000805b610100811015610e0d576001811b915083821615610dfd57828160f81b604051602001610deb92919061135d565b60405160208183030381529060405292505b610e06816111a0565b9050610dbd565b5050919050565b6033546001600160a01b0316331461066b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161073e565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6001600160a01b0381168114610db457600080fd5b600060208284031215610ee757600080fd5b8135610ef281610ec0565b9392505050565b6020808252825182820181905260009190848201906040850190845b81811015610f3a5783516001600160a01b031683529284019291840191600101610f15565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b6040516060810167ffffffffffffffff81118282101715610f7f57610f7f610f46565b60405290565b600067ffffffffffffffff80841115610fa057610fa0610f46565b604051601f8501601f19908116603f01168101908282118183101715610fc857610fc8610f46565b81604052809350858152868686011115610fe157600080fd5b858560208301376000602087830101525050509392505050565b60006020828403121561100d57600080fd5b813567ffffffffffffffff81111561102457600080fd5b8201601f8101841361103557600080fd5b61104484823560208401610f85565b949350505050565b6000806040838503121561105f57600080fd5b823561106a81610ec0565b9150602083013567ffffffffffffffff8082111561108757600080fd5b908401906060828703121561109b57600080fd5b6110a3610f5c565b8235828111156110b257600080fd5b83019150601f820187136110c557600080fd5b6110d487833560208501610f85565b815260208301356020820152604083013560408201528093505050509250929050565b60006020828403121561110957600080fd5b5051919050565b60006020828403121561112257600080fd5b81516001600160c01b0381168114610ef257600080fd5b60006020828403121561114b57600080fd5b815160ff81168114610ef257600080fd5b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561119b5761119b611172565b500190565b60006000198214156111b4576111b4611172565b5060010190565b6000604082840312156111cd57600080fd5b6040516040810181811067ffffffffffffffff821117156111f0576111f0610f46565b60405282516111fe81610ec0565b815260208301516bffffffffffffffffffffffff8116811461121f57600080fd5b60208201529392505050565b60005b8381101561124657818101518382015260200161122e565b83811115611255576000848401525b50505050565b6000815180845261127381602086016020860161122b565b601f01601f19169290920160200192915050565b602081526000610ef2602083018461125b565b60208082526052908201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360408201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560608201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608082015260a00190565b60018060a01b038316815260406020820152600082516060604084015261133c60a084018261125b565b90506020840151606084015260408401516080840152809150509392505050565b6000835161136f81846020880161122b565b6001600160f81b031993909316919092019081526001019291505056fea2646970667358221220162f714a5a69ddac702997b4092dcbcb78db569d3a308e247085fdd103a8648064736f6c634300080c0033",
}

// ContractFinalizerServiceManagerABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractFinalizerServiceManagerMetaData.ABI instead.
var ContractFinalizerServiceManagerABI = ContractFinalizerServiceManagerMetaData.ABI

// ContractFinalizerServiceManagerBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractFinalizerServiceManagerMetaData.Bin instead.
var ContractFinalizerServiceManagerBin = ContractFinalizerServiceManagerMetaData.Bin

// DeployContractFinalizerServiceManager deploys a new Ethereum contract, binding an instance of ContractFinalizerServiceManager to it.
func DeployContractFinalizerServiceManager(auth *bind.TransactOpts, backend bind.ContractBackend, _delegation common.Address, _registryCoordinator common.Address, _stakeRegistry common.Address, _taskManager common.Address) (common.Address, *types.Transaction, *ContractFinalizerServiceManager, error) {
	parsed, err := ContractFinalizerServiceManagerMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractFinalizerServiceManagerBin), backend, _delegation, _registryCoordinator, _stakeRegistry, _taskManager)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &ContractFinalizerServiceManager{ContractFinalizerServiceManagerCaller: ContractFinalizerServiceManagerCaller{contract: contract}, ContractFinalizerServiceManagerTransactor: ContractFinalizerServiceManagerTransactor{contract: contract}, ContractFinalizerServiceManagerFilterer: ContractFinalizerServiceManagerFilterer{contract: contract}}, nil
}

// ContractFinalizerServiceManager is an auto generated Go binding around an Ethereum contract.
type ContractFinalizerServiceManager struct {
	ContractFinalizerServiceManagerCaller     // Read-only binding to the contract
	ContractFinalizerServiceManagerTransactor // Write-only binding to the contract
	ContractFinalizerServiceManagerFilterer   // Log filterer for contract events
}

// ContractFinalizerServiceManagerCaller is an auto generated read-only Go binding around an Ethereum contract.
type ContractFinalizerServiceManagerCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractFinalizerServiceManagerTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ContractFinalizerServiceManagerTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractFinalizerServiceManagerFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ContractFinalizerServiceManagerFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractFinalizerServiceManagerSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ContractFinalizerServiceManagerSession struct {
	Contract     *ContractFinalizerServiceManager // Generic contract binding to set the session for
	CallOpts     bind.CallOpts                    // Call options to use throughout this session
	TransactOpts bind.TransactOpts                // Transaction auth options to use throughout this session
}

// ContractFinalizerServiceManagerCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ContractFinalizerServiceManagerCallerSession struct {
	Contract *ContractFinalizerServiceManagerCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                          // Call options to use throughout this session
}

// ContractFinalizerServiceManagerTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ContractFinalizerServiceManagerTransactorSession struct {
	Contract     *ContractFinalizerServiceManagerTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                          // Transaction auth options to use throughout this session
}

// ContractFinalizerServiceManagerRaw is an auto generated low-level Go binding around an Ethereum contract.
type ContractFinalizerServiceManagerRaw struct {
	Contract *ContractFinalizerServiceManager // Generic contract binding to access the raw methods on
}

// ContractFinalizerServiceManagerCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ContractFinalizerServiceManagerCallerRaw struct {
	Contract *ContractFinalizerServiceManagerCaller // Generic read-only contract binding to access the raw methods on
}

// ContractFinalizerServiceManagerTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ContractFinalizerServiceManagerTransactorRaw struct {
	Contract *ContractFinalizerServiceManagerTransactor // Generic write-only contract binding to access the raw methods on
}

// NewContractFinalizerServiceManager creates a new instance of ContractFinalizerServiceManager, bound to a specific deployed contract.
func NewContractFinalizerServiceManager(address common.Address, backend bind.ContractBackend) (*ContractFinalizerServiceManager, error) {
	contract, err := bindContractFinalizerServiceManager(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerServiceManager{ContractFinalizerServiceManagerCaller: ContractFinalizerServiceManagerCaller{contract: contract}, ContractFinalizerServiceManagerTransactor: ContractFinalizerServiceManagerTransactor{contract: contract}, ContractFinalizerServiceManagerFilterer: ContractFinalizerServiceManagerFilterer{contract: contract}}, nil
}

// NewContractFinalizerServiceManagerCaller creates a new read-only instance of ContractFinalizerServiceManager, bound to a specific deployed contract.
func NewContractFinalizerServiceManagerCaller(address common.Address, caller bind.ContractCaller) (*ContractFinalizerServiceManagerCaller, error) {
	contract, err := bindContractFinalizerServiceManager(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerServiceManagerCaller{contract: contract}, nil
}

// NewContractFinalizerServiceManagerTransactor creates a new write-only instance of ContractFinalizerServiceManager, bound to a specific deployed contract.
func NewContractFinalizerServiceManagerTransactor(address common.Address, transactor bind.ContractTransactor) (*ContractFinalizerServiceManagerTransactor, error) {
	contract, err := bindContractFinalizerServiceManager(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerServiceManagerTransactor{contract: contract}, nil
}

// NewContractFinalizerServiceManagerFilterer creates a new log filterer instance of ContractFinalizerServiceManager, bound to a specific deployed contract.
func NewContractFinalizerServiceManagerFilterer(address common.Address, filterer bind.ContractFilterer) (*ContractFinalizerServiceManagerFilterer, error) {
	contract, err := bindContractFinalizerServiceManager(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerServiceManagerFilterer{contract: contract}, nil
}

// bindContractFinalizerServiceManager binds a generic wrapper to an already deployed contract.
func bindContractFinalizerServiceManager(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := ContractFinalizerServiceManagerMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractFinalizerServiceManager.Contract.ContractFinalizerServiceManagerCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.ContractFinalizerServiceManagerTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.ContractFinalizerServiceManagerTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractFinalizerServiceManager.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.contract.Transact(opts, method, params...)
}

// GetOperatorRestakedStrategies is a free data retrieval call binding the contract method 0x33cfb7b7.
//
// Solidity: function getOperatorRestakedStrategies(address operator) view returns(address[])
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCaller) GetOperatorRestakedStrategies(opts *bind.CallOpts, operator common.Address) ([]common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerServiceManager.contract.Call(opts, &out, "getOperatorRestakedStrategies", operator)

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetOperatorRestakedStrategies is a free data retrieval call binding the contract method 0x33cfb7b7.
//
// Solidity: function getOperatorRestakedStrategies(address operator) view returns(address[])
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) GetOperatorRestakedStrategies(operator common.Address) ([]common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.GetOperatorRestakedStrategies(&_ContractFinalizerServiceManager.CallOpts, operator)
}

// GetOperatorRestakedStrategies is a free data retrieval call binding the contract method 0x33cfb7b7.
//
// Solidity: function getOperatorRestakedStrategies(address operator) view returns(address[])
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCallerSession) GetOperatorRestakedStrategies(operator common.Address) ([]common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.GetOperatorRestakedStrategies(&_ContractFinalizerServiceManager.CallOpts, operator)
}

// GetRestakeableStrategies is a free data retrieval call binding the contract method 0xe481af9d.
//
// Solidity: function getRestakeableStrategies() view returns(address[])
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCaller) GetRestakeableStrategies(opts *bind.CallOpts) ([]common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerServiceManager.contract.Call(opts, &out, "getRestakeableStrategies")

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetRestakeableStrategies is a free data retrieval call binding the contract method 0xe481af9d.
//
// Solidity: function getRestakeableStrategies() view returns(address[])
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) GetRestakeableStrategies() ([]common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.GetRestakeableStrategies(&_ContractFinalizerServiceManager.CallOpts)
}

// GetRestakeableStrategies is a free data retrieval call binding the contract method 0xe481af9d.
//
// Solidity: function getRestakeableStrategies() view returns(address[])
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCallerSession) GetRestakeableStrategies() ([]common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.GetRestakeableStrategies(&_ContractFinalizerServiceManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerServiceManager.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) Owner() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.Owner(&_ContractFinalizerServiceManager.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCallerSession) Owner() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.Owner(&_ContractFinalizerServiceManager.CallOpts)
}

// TaskManager is a free data retrieval call binding the contract method 0xa50a640e.
//
// Solidity: function taskManager() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCaller) TaskManager(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerServiceManager.contract.Call(opts, &out, "taskManager")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// TaskManager is a free data retrieval call binding the contract method 0xa50a640e.
//
// Solidity: function taskManager() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) TaskManager() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.TaskManager(&_ContractFinalizerServiceManager.CallOpts)
}

// TaskManager is a free data retrieval call binding the contract method 0xa50a640e.
//
// Solidity: function taskManager() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCallerSession) TaskManager() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.TaskManager(&_ContractFinalizerServiceManager.CallOpts)
}

// DeregisterOperatorFromAVS is a paid mutator transaction binding the contract method 0xa364f4da.
//
// Solidity: function deregisterOperatorFromAVS(address operator) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) DeregisterOperatorFromAVS(opts *bind.TransactOpts, operator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "deregisterOperatorFromAVS", operator)
}

// DeregisterOperatorFromAVS is a paid mutator transaction binding the contract method 0xa364f4da.
//
// Solidity: function deregisterOperatorFromAVS(address operator) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) DeregisterOperatorFromAVS(operator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.DeregisterOperatorFromAVS(&_ContractFinalizerServiceManager.TransactOpts, operator)
}

// DeregisterOperatorFromAVS is a paid mutator transaction binding the contract method 0xa364f4da.
//
// Solidity: function deregisterOperatorFromAVS(address operator) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) DeregisterOperatorFromAVS(operator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.DeregisterOperatorFromAVS(&_ContractFinalizerServiceManager.TransactOpts, operator)
}

// Initialize is a paid mutator transaction binding the contract method 0xc4d66de8.
//
// Solidity: function initialize(address initialOwner) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) Initialize(opts *bind.TransactOpts, initialOwner common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "initialize", initialOwner)
}

// Initialize is a paid mutator transaction binding the contract method 0xc4d66de8.
//
// Solidity: function initialize(address initialOwner) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) Initialize(initialOwner common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.Initialize(&_ContractFinalizerServiceManager.TransactOpts, initialOwner)
}

// Initialize is a paid mutator transaction binding the contract method 0xc4d66de8.
//
// Solidity: function initialize(address initialOwner) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) Initialize(initialOwner common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.Initialize(&_ContractFinalizerServiceManager.TransactOpts, initialOwner)
}

// RegisterOperatorToAVS is a paid mutator transaction binding the contract method 0x9926ee7d.
//
// Solidity: function registerOperatorToAVS(address operator, (bytes,bytes32,uint256) operatorSignature) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) RegisterOperatorToAVS(opts *bind.TransactOpts, operator common.Address, operatorSignature ISignatureUtilsSignatureWithSaltAndExpiry) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "registerOperatorToAVS", operator, operatorSignature)
}

// RegisterOperatorToAVS is a paid mutator transaction binding the contract method 0x9926ee7d.
//
// Solidity: function registerOperatorToAVS(address operator, (bytes,bytes32,uint256) operatorSignature) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) RegisterOperatorToAVS(operator common.Address, operatorSignature ISignatureUtilsSignatureWithSaltAndExpiry) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.RegisterOperatorToAVS(&_ContractFinalizerServiceManager.TransactOpts, operator, operatorSignature)
}

// RegisterOperatorToAVS is a paid mutator transaction binding the contract method 0x9926ee7d.
//
// Solidity: function registerOperatorToAVS(address operator, (bytes,bytes32,uint256) operatorSignature) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) RegisterOperatorToAVS(operator common.Address, operatorSignature ISignatureUtilsSignatureWithSaltAndExpiry) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.RegisterOperatorToAVS(&_ContractFinalizerServiceManager.TransactOpts, operator, operatorSignature)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.RenounceOwnership(&_ContractFinalizerServiceManager.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.RenounceOwnership(&_ContractFinalizerServiceManager.TransactOpts)
}

// SetMetadataURI is a paid mutator transaction binding the contract method 0x750521f5.
//
// Solidity: function setMetadataURI(string _metadataURI) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) SetMetadataURI(opts *bind.TransactOpts, _metadataURI string) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "setMetadataURI", _metadataURI)
}

// SetMetadataURI is a paid mutator transaction binding the contract method 0x750521f5.
//
// Solidity: function setMetadataURI(string _metadataURI) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) SetMetadataURI(_metadataURI string) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.SetMetadataURI(&_ContractFinalizerServiceManager.TransactOpts, _metadataURI)
}

// SetMetadataURI is a paid mutator transaction binding the contract method 0x750521f5.
//
// Solidity: function setMetadataURI(string _metadataURI) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) SetMetadataURI(_metadataURI string) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.SetMetadataURI(&_ContractFinalizerServiceManager.TransactOpts, _metadataURI)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.TransferOwnership(&_ContractFinalizerServiceManager.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.TransferOwnership(&_ContractFinalizerServiceManager.TransactOpts, newOwner)
}

// ContractFinalizerServiceManagerInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the ContractFinalizerServiceManager contract.
type ContractFinalizerServiceManagerInitializedIterator struct {
	Event *ContractFinalizerServiceManagerInitialized // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerServiceManagerInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerServiceManagerInitialized)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerServiceManagerInitialized)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerServiceManagerInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerServiceManagerInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerServiceManagerInitialized represents a Initialized event raised by the ContractFinalizerServiceManager contract.
type ContractFinalizerServiceManagerInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) FilterInitialized(opts *bind.FilterOpts) (*ContractFinalizerServiceManagerInitializedIterator, error) {

	logs, sub, err := _ContractFinalizerServiceManager.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerServiceManagerInitializedIterator{contract: _ContractFinalizerServiceManager.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *ContractFinalizerServiceManagerInitialized) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerServiceManager.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerServiceManagerInitialized)
				if err := _ContractFinalizerServiceManager.contract.UnpackLog(event, "Initialized", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseInitialized is a log parse operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) ParseInitialized(log types.Log) (*ContractFinalizerServiceManagerInitialized, error) {
	event := new(ContractFinalizerServiceManagerInitialized)
	if err := _ContractFinalizerServiceManager.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractFinalizerServiceManagerOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the ContractFinalizerServiceManager contract.
type ContractFinalizerServiceManagerOwnershipTransferredIterator struct {
	Event *ContractFinalizerServiceManagerOwnershipTransferred // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractFinalizerServiceManagerOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerServiceManagerOwnershipTransferred)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractFinalizerServiceManagerOwnershipTransferred)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractFinalizerServiceManagerOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerServiceManagerOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerServiceManagerOwnershipTransferred represents a OwnershipTransferred event raised by the ContractFinalizerServiceManager contract.
type ContractFinalizerServiceManagerOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*ContractFinalizerServiceManagerOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractFinalizerServiceManager.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerServiceManagerOwnershipTransferredIterator{contract: _ContractFinalizerServiceManager.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *ContractFinalizerServiceManagerOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractFinalizerServiceManager.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerServiceManagerOwnershipTransferred)
				if err := _ContractFinalizerServiceManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferred is a log parse operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) ParseOwnershipTransferred(log types.Log) (*ContractFinalizerServiceManagerOwnershipTransferred, error) {
	event := new(ContractFinalizerServiceManagerOwnershipTransferred)
	if err := _ContractFinalizerServiceManager.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

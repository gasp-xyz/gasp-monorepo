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
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_avsDirectory\",\"type\":\"address\",\"internalType\":\"contractIAVSDirectory\"},{\"name\":\"_registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"_stakeRegistry\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"},{\"name\":\"_taskManager\",\"type\":\"address\",\"internalType\":\"contractIFinalizerTaskManager\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"avsDirectory\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"deregisterOperatorFromAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"getOperatorRestakedStrategies\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getRestakeableStrategies\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registerOperatorToAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorSignature\",\"type\":\"tuple\",\"internalType\":\"structISignatureUtils.SignatureWithSaltAndExpiry\",\"components\":[{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"expiry\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setMetadataURI\",\"inputs\":[{\"name\":\"_metadataURI\",\"type\":\"string\",\"internalType\":\"string\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"taskManager\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIFinalizerTaskManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false}]",
	Bin: "0x6101006040523480156200001257600080fd5b50604051620017423803806200174283398101604081905262000035916200014f565b6001600160a01b0380851660c052808416608052821660a0528383836200005b62000074565b5050506001600160a01b031660e05250620001b7915050565b600054610100900460ff1615620000e15760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116101562000134576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146200014c57600080fd5b50565b600080600080608085870312156200016657600080fd5b8451620001738162000136565b6020860151909450620001868162000136565b6040860151909350620001998162000136565b6060860151909250620001ac8162000136565b939692955090935050565b60805160a05160c05160e0516114f062000252600039600061016a01526000818160d9015281816106bd0152818161078f01526108630152600081816103b401528181610510015281816105a701528181610a6d01528181610bf10152610c900152600081816101df0152818161026e015281816102ee0152818161073201528181610807015281816109ab0152610b4c01526114f06000f3fe608060405234801561001057600080fd5b50600436106100a95760003560e01c80639926ee7d116100715780639926ee7d1461013f578063a364f4da14610152578063a50a640e14610165578063c4d66de81461018c578063e481af9d1461019f578063f2fde38b146101a757600080fd5b806333cfb7b7146100ae5780636b3aa72e146100d7578063715018a614610111578063750521f51461011b5780638da5cb5b1461012e575b600080fd5b6100c16100bc366004611008565b6101ba565b6040516100ce919061102c565b60405180910390f35b7f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160a01b0390911681526020016100ce565b61011961068a565b005b61011961012936600461112e565b61069e565b6033546001600160a01b03166100f9565b61011961014d36600461117f565b610727565b610119610160366004611008565b6107fc565b6100f97f000000000000000000000000000000000000000000000000000000000000000081565b61011961019a366004611008565b610892565b6100c16109a5565b6101196101b5366004611008565b610d6f565b6040516309aa152760e11b81526001600160a01b0382811660048301526060916000917f000000000000000000000000000000000000000000000000000000000000000016906313542a4e90602401602060405180830381865afa158015610226573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061024a919061122a565b60405163871ef04960e01b8152600481018290529091506000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063871ef04990602401602060405180830381865afa1580156102b5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102d99190611243565b90506001600160c01b038116158061037357507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561034a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061036e919061126c565b60ff16155b1561038f57505060408051600081526020810190915292915050565b60006103a3826001600160c01b0316610de8565b90506000805b8251811015610479577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316633ca5a5f58483815181106103f3576103f361128f565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015610437573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061045b919061122a565b61046590836112bb565b915080610471816112d3565b9150506103a9565b5060008167ffffffffffffffff81111561049557610495611079565b6040519080825280602002602001820160405280156104be578160200160208202803683370190505b5090506000805b845181101561067d5760008582815181106104e2576104e261128f565b0160200151604051633ca5a5f560e01b815260f89190911c6004820181905291506000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633ca5a5f590602401602060405180830381865afa158015610557573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061057b919061122a565b905060005b81811015610667576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa1580156105f5573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061061991906112ee565b6000015186868151811061062f5761062f61128f565b6001600160a01b039092166020928302919091019091015284610651816112d3565b955050808061065f906112d3565b915050610580565b5050508080610675906112d3565b9150506104c5565b5090979650505050505050565b610692610eab565b61069c6000610f05565b565b6106a6610eab565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb355906106f29084906004016113ab565b600060405180830381600087803b15801561070c57600080fd5b505af1158015610720573d6000803e3d6000fd5b5050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146107785760405162461bcd60e51b815260040161076f906113be565b60405180910390fd5b604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d906107c69085908590600401611436565b600060405180830381600087803b1580156107e057600080fd5b505af11580156107f4573d6000803e3d6000fd5b505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146108445760405162461bcd60e51b815260040161076f906113be565b6040516351b27a6d60e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024016106f2565b600054610100900460ff16158080156108b25750600054600160ff909116105b806108cc5750303b1580156108cc575060005460ff166001145b61092f5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840161076f565b6000805460ff191660011790558015610952576000805461ff0019166101001790555b61095b82610f57565b80156109a1576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050565b606060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a07573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a2b919061126c565b60ff16905080610a4957505060408051600081526020810190915290565b6000805b82811015610afe57604051633ca5a5f560e01b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015610abc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ae0919061122a565b610aea90836112bb565b915080610af6816112d3565b915050610a4d565b5060008167ffffffffffffffff811115610b1a57610b1a611079565b604051908082528060200260200182016040528015610b43578160200160208202803683370190505b5090506000805b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ba8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bcc919061126c565b60ff16811015610d6557604051633ca5a5f560e01b815260ff821660048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015610c40573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c64919061122a565b905060005b81811015610d50576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015610cde573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d0291906112ee565b60000151858581518110610d1857610d1861128f565b6001600160a01b039092166020928302919091019091015283610d3a816112d3565b9450508080610d48906112d3565b915050610c69565b50508080610d5d906112d3565b915050610b4a565b5090949350505050565b610d77610eab565b6001600160a01b038116610ddc5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161076f565b610de581610f05565b50565b6060600080610df684610fc2565b61ffff1667ffffffffffffffff811115610e1257610e12611079565b6040519080825280601f01601f191660200182016040528015610e3c576020820181803683370190505b5090506000805b825182108015610e54575061010081105b15610d65576001811b935085841615610e9b578060f81b838381518110610e7d57610e7d61128f565b60200101906001600160f81b031916908160001a9053508160010191505b610ea4816112d3565b9050610e43565b6033546001600160a01b0316331461069c5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161076f565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600054610100900460ff16610ddc5760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b606482015260840161076f565b6000805b8215610fed57610fd7600184611481565b9092169180610fe581611498565b915050610fc6565b92915050565b6001600160a01b0381168114610de557600080fd5b60006020828403121561101a57600080fd5b813561102581610ff3565b9392505050565b6020808252825182820181905260009190848201906040850190845b8181101561106d5783516001600160a01b031683529284019291840191600101611048565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b6040516060810167ffffffffffffffff811182821017156110b2576110b2611079565b60405290565b600067ffffffffffffffff808411156110d3576110d3611079565b604051601f8501601f19908116603f011681019082821181831017156110fb576110fb611079565b8160405280935085815286868601111561111457600080fd5b858560208301376000602087830101525050509392505050565b60006020828403121561114057600080fd5b813567ffffffffffffffff81111561115757600080fd5b8201601f8101841361116857600080fd5b611177848235602084016110b8565b949350505050565b6000806040838503121561119257600080fd5b823561119d81610ff3565b9150602083013567ffffffffffffffff808211156111ba57600080fd5b90840190606082870312156111ce57600080fd5b6111d661108f565b8235828111156111e557600080fd5b83019150601f820187136111f857600080fd5b611207878335602085016110b8565b815260208301356020820152604083013560408201528093505050509250929050565b60006020828403121561123c57600080fd5b5051919050565b60006020828403121561125557600080fd5b81516001600160c01b038116811461102557600080fd5b60006020828403121561127e57600080fd5b815160ff8116811461102557600080fd5b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600082198211156112ce576112ce6112a5565b500190565b60006000198214156112e7576112e76112a5565b5060010190565b60006040828403121561130057600080fd5b6040516040810181811067ffffffffffffffff8211171561132357611323611079565b604052825161133181610ff3565b815260208301516bffffffffffffffffffffffff8116811461135257600080fd5b60208201529392505050565b6000815180845260005b8181101561138457602081850181015186830182015201611368565b81811115611396576000602083870101525b50601f01601f19169290920160200192915050565b602081526000611025602083018461135e565b60208082526052908201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360408201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560608201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608082015260a00190565b60018060a01b038316815260406020820152600082516060604084015261146060a084018261135e565b90506020840151606084015260408401516080840152809150509392505050565b600082821015611493576114936112a5565b500390565b600061ffff808316818114156114b0576114b06112a5565b600101939250505056fea2646970667358221220e704ed8d91544c52ae2e3bec84f24428a9f17197071067ab5b5f87b01522c89564736f6c634300080c0033",
}

// ContractFinalizerServiceManagerABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractFinalizerServiceManagerMetaData.ABI instead.
var ContractFinalizerServiceManagerABI = ContractFinalizerServiceManagerMetaData.ABI

// ContractFinalizerServiceManagerBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractFinalizerServiceManagerMetaData.Bin instead.
var ContractFinalizerServiceManagerBin = ContractFinalizerServiceManagerMetaData.Bin

// DeployContractFinalizerServiceManager deploys a new Ethereum contract, binding an instance of ContractFinalizerServiceManager to it.
func DeployContractFinalizerServiceManager(auth *bind.TransactOpts, backend bind.ContractBackend, _avsDirectory common.Address, _registryCoordinator common.Address, _stakeRegistry common.Address, _taskManager common.Address) (common.Address, *types.Transaction, *ContractFinalizerServiceManager, error) {
	parsed, err := ContractFinalizerServiceManagerMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractFinalizerServiceManagerBin), backend, _avsDirectory, _registryCoordinator, _stakeRegistry, _taskManager)
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

// AvsDirectory is a free data retrieval call binding the contract method 0x6b3aa72e.
//
// Solidity: function avsDirectory() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCaller) AvsDirectory(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerServiceManager.contract.Call(opts, &out, "avsDirectory")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// AvsDirectory is a free data retrieval call binding the contract method 0x6b3aa72e.
//
// Solidity: function avsDirectory() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) AvsDirectory() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.AvsDirectory(&_ContractFinalizerServiceManager.CallOpts)
}

// AvsDirectory is a free data retrieval call binding the contract method 0x6b3aa72e.
//
// Solidity: function avsDirectory() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCallerSession) AvsDirectory() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.AvsDirectory(&_ContractFinalizerServiceManager.CallOpts)
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

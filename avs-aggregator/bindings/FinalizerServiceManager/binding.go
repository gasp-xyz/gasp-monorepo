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

// IRewardsCoordinatorRewardsSubmission is an auto generated low-level Go binding around an user-defined struct.
type IRewardsCoordinatorRewardsSubmission struct {
	StrategiesAndMultipliers []IRewardsCoordinatorStrategyAndMultiplier
	Token                    common.Address
	Amount                   *big.Int
	StartTimestamp           uint32
	Duration                 uint32
}

// IRewardsCoordinatorStrategyAndMultiplier is an auto generated low-level Go binding around an user-defined struct.
type IRewardsCoordinatorStrategyAndMultiplier struct {
	Strategy   common.Address
	Multiplier *big.Int
}

// ISignatureUtilsSignatureWithSaltAndExpiry is an auto generated low-level Go binding around an user-defined struct.
type ISignatureUtilsSignatureWithSaltAndExpiry struct {
	Signature []byte
	Salt      [32]byte
	Expiry    *big.Int
}

// ContractFinalizerServiceManagerMetaData contains all meta data concerning the ContractFinalizerServiceManager contract.
var ContractFinalizerServiceManagerMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_avsDirectory\",\"type\":\"address\",\"internalType\":\"contractIAVSDirectory\"},{\"name\":\"_rewardsCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRewardsCoordinator\"},{\"name\":\"_registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"_stakeRegistry\",\"type\":\"address\",\"internalType\":\"contractIStakeRegistry\"},{\"name\":\"_taskManager\",\"type\":\"address\",\"internalType\":\"contractIFinalizerTaskManager\"},{\"name\":\"_recurrentRegistrationBlocksLimit\",\"type\":\"uint64\",\"internalType\":\"uint64\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"avsDirectory\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"createAVSRewardsSubmission\",\"inputs\":[{\"name\":\"rewardsSubmissions\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinator.RewardsSubmission[]\",\"components\":[{\"name\":\"strategiesAndMultipliers\",\"type\":\"tuple[]\",\"internalType\":\"structIRewardsCoordinator.StrategyAndMultiplier[]\",\"components\":[{\"name\":\"strategy\",\"type\":\"address\",\"internalType\":\"contractIStrategy\"},{\"name\":\"multiplier\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"token\",\"type\":\"address\",\"internalType\":\"contractIERC20\"},{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"startTimestamp\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"duration\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"deregisterOperatorFromAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"ejectOperators\",\"inputs\":[{\"name\":\"operators\",\"type\":\"address[]\",\"internalType\":\"address[]\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes[]\",\"internalType\":\"bytes[]\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"ejector\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorRestakedStrategies\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getRestakeableStrategies\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_ejector\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"recurrentRegistrationBlocksLimit\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint64\",\"internalType\":\"uint64\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"registerOperatorToAVS\",\"inputs\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorSignature\",\"type\":\"tuple\",\"internalType\":\"structISignatureUtils.SignatureWithSaltAndExpiry\",\"components\":[{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"salt\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"expiry\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rewardsInitiator\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setEjector\",\"inputs\":[{\"name\":\"_ejector\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"setRewardsInitiator\",\"inputs\":[{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"taskManager\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIFinalizerTaskManager\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateAVSMetadataURI\",\"inputs\":[{\"name\":\"_metadataURI\",\"type\":\"string\",\"internalType\":\"string\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"EjectorUpdated\",\"inputs\":[{\"name\":\"prevEjector\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"newEjector\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RewardsInitiatorUpdated\",\"inputs\":[{\"name\":\"prevRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"newRewardsInitiator\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"}],\"anonymous\":false}]",
	Bin: "0x6101406040523480156200001257600080fd5b506040516200272038038062002720833981016040819052620000359162000168565b6001600160a01b0380871660805280861660a05280851660c052831660e05285858585620000626200008d565b5050506001600160a01b03909216610100526001600160401b03166101205250620002079350505050565b600054610100900460ff1615620000fa5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811610156200014d576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146200016557600080fd5b50565b60008060008060008060c087890312156200018257600080fd5b86516200018f816200014f565b6020880151909650620001a2816200014f565b6040880151909550620001b5816200014f565b6060880151909450620001c8816200014f565b6080880151909350620001db816200014f565b60a08801519092506001600160401b0381168114620001f957600080fd5b809150509295509295509295565b60805160a05160c05160e0516101005161012051612436620002ea600039600081816101ab0152610c230152600061024f0152600081816106f30152818161084e015281816108e501528181610b7a015281816110a40152818161122701526112c60152600081816104030152818161051e015281816105ad0152818161062d015281816109f801528181610a5701528181610acb01528181610d9f01528181610fdf015261118201526000818161156701528181611623015261170f0152600081816101e701528181610d2601528181610dfb0152610e7301526124366000f3fe608060405234801561001057600080fd5b50600436106101165760003560e01c80639926ee7d116100a2578063c0c53b8b11610071578063c0c53b8b14610284578063e481af9d14610297578063f2fde38b1461029f578063fc299dee146102b2578063fce36c7d146102c557600080fd5b80639926ee7d14610224578063a364f4da14610237578063a50a640e1461024a578063a98fb3551461027157600080fd5b80633bc28c8c116100e95780633bc28c8c14610193578063614cc144146101a65780636b3aa72e146101e5578063715018a61461020b5780638da5cb5b1461021357600080fd5b80631e25abfd1461011b57806328f61b31146101305780632cdd1e861461016057806333cfb7b714610173575b600080fd5b61012e610129366004611b4e565b6102d8565b005b609754610143906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b61012e61016e366004611bce565b6104e5565b610186610181366004611bce565b6104f9565b6040516101579190611bf2565b61012e6101a1366004611bce565b6109c8565b6101cd7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160401b039091168152602001610157565b7f0000000000000000000000000000000000000000000000000000000000000000610143565b61012e6109d9565b6033546001600160a01b0316610143565b61012e610232366004611cf2565b6109ed565b61012e610245366004611bce565b610d94565b6101437f000000000000000000000000000000000000000000000000000000000000000081565b61012e61027f366004611d9c565b610e54565b61012e610292366004611dec565b610ea8565b610186610fd9565b61012e6102ad366004611bce565b6113a5565b606554610143906001600160a01b031681565b61012e6102d3366004611e37565b61141b565b6097546001600160a01b03163314806102fb57506033546001600160a01b031633145b6103815760405162461bcd60e51b815260206004820152604660248201527f5265676973747279436f6f7264696e61746f722e6f6e6c79456a6563746f724f60448201527f724f776e65723a2063616c6c6572206973206e6f7420656a6563746f72206f726064820152651037bbb732b960d11b608482015260a4015b60405180910390fd5b8281146103f65760405162461bcd60e51b815260206004820152603e60248201527f5265676973747279436f6f7264696e61746f722e656a6563744f70657261746f60448201527f72733a2061726773206c656e67746820646f6573206e6f74206d6174636800006064820152608401610378565b60005b838110156104de577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636e3b17db86868481811061044257610442611e78565b90506020020160208101906104579190611bce565b85858581811061046957610469611e78565b905060200281019061047b9190611e8e565b6040518463ffffffff1660e01b815260040161049993929190611ed4565b600060405180830381600087803b1580156104b357600080fd5b505af11580156104c7573d6000803e3d6000fd5b5050505080806104d690611f2a565b9150506103f9565b5050505050565b6104ed61177c565b6104f6816117d6565b50565b6040516309aa152760e11b81526001600160a01b0382811660048301526060916000917f000000000000000000000000000000000000000000000000000000000000000016906313542a4e90602401602060405180830381865afa158015610565573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105899190611f43565b60405163871ef04960e01b8152600481018290529091506000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063871ef04990602401602060405180830381865afa1580156105f4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106189190611f5c565b90506001600160c01b03811615806106b257507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610689573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106ad9190611f85565b60ff16155b156106ce57505060408051600081526020810190915292915050565b60006106e2826001600160c01b031661183f565b90506000805b82518110156107b8577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316633ca5a5f584838151811061073257610732611e78565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015610776573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061079a9190611f43565b6107a49083611fa8565b9150806107b081611f2a565b9150506106e8565b506000816001600160401b038111156107d3576107d3611c3f565b6040519080825280602002602001820160405280156107fc578160200160208202803683370190505b5090506000805b84518110156109bb57600085828151811061082057610820611e78565b0160200151604051633ca5a5f560e01b815260f89190911c6004820181905291506000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633ca5a5f590602401602060405180830381865afa158015610895573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108b99190611f43565b905060005b818110156109a5576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015610933573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109579190611fd5565b6000015186868151811061096d5761096d611e78565b6001600160a01b03909216602092830291909101909101528461098f81611f2a565b955050808061099d90611f2a565b9150506108be565b50505080806109b390611f2a565b915050610803565b5090979650505050505050565b6109d061177c565b6104f681611901565b6109e161177c565b6109eb600061196a565b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a355760405162461bcd60e51b815260040161037890612033565b6040516309aa152760e11b81526001600160a01b0383811660048301526000917f0000000000000000000000000000000000000000000000000000000000000000909116906313542a4e90602401602060405180830381865afa158015610aa0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ac49190611f43565b905060005b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b27573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b4b9190611f85565b60ff168160ff161015610d0e57604051631f0a3c3360e31b81526004810183905260ff821660248201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063f851e19890604401606060405180830381865afa158015610bc9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bed91906120bd565b905080604001516001600160601b03166000148015610c125750805163ffffffff1615155b15610cfb5780516001600160401b037f00000000000000000000000000000000000000000000000000000000000000001690610c549063ffffffff164361212e565b11610cfb5760405162461bcd60e51b815260206004820152606560248201527f536572766963654d616e616765722e72656769737465724f70657261746f725460448201527f6f4156533a206d696e696d756d20626c6f636b7320656c6170736564206c696d60648201527f697420666f7220726563757272656e7420726567697374726174696f6e206e6f6084820152641d081b595d60da1b60a482015260c401610378565b5080610d0681612145565b915050610ac9565b50604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d90610d5d90869086906004016121b1565b600060405180830381600087803b158015610d7757600080fd5b505af1158015610d8b573d6000803e3d6000fd5b50505050505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610ddc5760405162461bcd60e51b815260040161037890612033565b6040516351b27a6d60e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024015b600060405180830381600087803b158015610e4057600080fd5b505af11580156104de573d6000803e3d6000fd5b610e5c61177c565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb35590610e269084906004016121fc565b600054610100900460ff1615808015610ec85750600054600160ff909116105b80610ee25750303b158015610ee2575060005460ff166001145b610f455760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610378565b6000805460ff191660011790558015610f68576000805461ff0019166101001790555b610f7284846119bc565b609780546001600160a01b0319166001600160a01b0384161790558015610fd3576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b606060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561103b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061105f9190611f85565b60ff1690508060000361108057505060408051600081526020810190915290565b6000805b8281101561113557604051633ca5a5f560e01b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa1580156110f3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906111179190611f43565b6111219083611fa8565b91508061112d81611f2a565b915050611084565b506000816001600160401b0381111561115057611150611c3f565b604051908082528060200260200182016040528015611179578160200160208202803683370190505b5090506000805b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156111de573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112029190611f85565b60ff1681101561139b57604051633ca5a5f560e01b815260ff821660048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015611276573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061129a9190611f43565b905060005b81811015611386576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa158015611314573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113389190611fd5565b6000015185858151811061134e5761134e611e78565b6001600160a01b03909216602092830291909101909101528361137081611f2a565b945050808061137e90611f2a565b91505061129f565b5050808061139390611f2a565b915050611180565b5090949350505050565b6113ad61177c565b6001600160a01b0381166114125760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610378565b6104f68161196a565b611423611a3d565b60005b818110156116f75782828281811061144057611440611e78565b9050602002810190611452919061220f565b611463906040810190602001611bce565b6001600160a01b03166323b872dd333086868681811061148557611485611e78565b9050602002810190611497919061220f565b604080516001600160e01b031960e087901b1681526001600160a01b039485166004820152939092166024840152013560448201526064016020604051808303816000875af11580156114ee573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611512919061223f565b50600083838381811061152757611527611e78565b9050602002810190611539919061220f565b61154a906040810190602001611bce565b604051636eb1769f60e11b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166024830152919091169063dd62ed3e90604401602060405180830381865afa1580156115b8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115dc9190611f43565b90508383838181106115f0576115f0611e78565b9050602002810190611602919061220f565b611613906040810190602001611bce565b6001600160a01b031663095ea7b37f00000000000000000000000000000000000000000000000000000000000000008387878781811061165557611655611e78565b9050602002810190611667919061220f565b604001356116759190611fa8565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af11580156116c0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116e4919061223f565b5050806116f090611f2a565b9050611426565b5060405163fce36c7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063fce36c7d9061174690859085906004016122d2565b600060405180830381600087803b15801561176057600080fd5b505af1158015611774573d6000803e3d6000fd5b505050505050565b6033546001600160a01b031633146109eb5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610378565b609754604080516001600160a01b03928316815291831660208301527f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9910160405180910390a1609780546001600160a01b0319166001600160a01b0392909216919091179055565b606060008061184d84611ad2565b61ffff166001600160401b0381111561186857611868611c3f565b6040519080825280601f01601f191660200182016040528015611892576020820181803683370190505b5090506000805b8251821080156118aa575061010081105b1561139b576001811b9350858416156118f1578060f81b8383815181106118d3576118d3611e78565b60200101906001600160f81b031916908160001a9053508160010191505b6118fa81611f2a565b9050611899565b606554604080516001600160a01b03928316815291831660208301527fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600054610100900460ff16611a275760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608401610378565b611a308261196a565b611a3981611901565b5050565b6065546001600160a01b031633146109eb5760405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a401610378565b6000805b8215611afd57611ae760018461212e565b9092169180611af5816123df565b915050611ad6565b92915050565b60008083601f840112611b1557600080fd5b5081356001600160401b03811115611b2c57600080fd5b6020830191508360208260051b8501011115611b4757600080fd5b9250929050565b60008060008060408587031215611b6457600080fd5b84356001600160401b0380821115611b7b57600080fd5b611b8788838901611b03565b90965094506020870135915080821115611ba057600080fd5b50611bad87828801611b03565b95989497509550505050565b6001600160a01b03811681146104f657600080fd5b600060208284031215611be057600080fd5b8135611beb81611bb9565b9392505050565b6020808252825182820181905260009190848201906040850190845b81811015611c335783516001600160a01b031683529284019291840191600101611c0e565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b604051606081016001600160401b0381118282101715611c7757611c77611c3f565b60405290565b60006001600160401b0380841115611c9757611c97611c3f565b604051601f8501601f19908116603f01168101908282118183101715611cbf57611cbf611c3f565b81604052809350858152868686011115611cd857600080fd5b858560208301376000602087830101525050509392505050565b60008060408385031215611d0557600080fd5b8235611d1081611bb9565b915060208301356001600160401b0380821115611d2c57600080fd5b9084019060608287031215611d4057600080fd5b611d48611c55565b823582811115611d5757600080fd5b83019150601f82018713611d6a57600080fd5b611d7987833560208501611c7d565b815260208301356020820152604083013560408201528093505050509250929050565b600060208284031215611dae57600080fd5b81356001600160401b03811115611dc457600080fd5b8201601f81018413611dd557600080fd5b611de484823560208401611c7d565b949350505050565b600080600060608486031215611e0157600080fd5b8335611e0c81611bb9565b92506020840135611e1c81611bb9565b91506040840135611e2c81611bb9565b809150509250925092565b60008060208385031215611e4a57600080fd5b82356001600160401b03811115611e6057600080fd5b611e6c85828601611b03565b90969095509350505050565b634e487b7160e01b600052603260045260246000fd5b6000808335601e19843603018112611ea557600080fd5b8301803591506001600160401b03821115611ebf57600080fd5b602001915036819003821315611b4757600080fd5b6001600160a01b03841681526040602082018190528101829052818360608301376000818301606090810191909152601f909201601f1916010192915050565b634e487b7160e01b600052601160045260246000fd5b600060018201611f3c57611f3c611f14565b5060010190565b600060208284031215611f5557600080fd5b5051919050565b600060208284031215611f6e57600080fd5b81516001600160c01b0381168114611beb57600080fd5b600060208284031215611f9757600080fd5b815160ff81168114611beb57600080fd5b60008219821115611fbb57611fbb611f14565b500190565b6001600160601b03811681146104f657600080fd5b600060408284031215611fe757600080fd5b604051604081018181106001600160401b038211171561200957612009611c3f565b604052825161201781611bb9565b8152602083015161202781611fc0565b60208201529392505050565b60208082526052908201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360408201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560608201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608082015260a00190565b63ffffffff811681146104f657600080fd5b6000606082840312156120cf57600080fd5b604051606081018181106001600160401b03821117156120f1576120f1611c3f565b60405282516120ff816120ab565b8152602083015161210f816120ab565b6020820152604083015161212281611fc0565b60408201529392505050565b60008282101561214057612140611f14565b500390565b600060ff821660ff810361215b5761215b611f14565b60010192915050565b6000815180845260005b8181101561218a5760208185018101518683018201520161216e565b8181111561219c576000602083870101525b50601f01601f19169290920160200192915050565b60018060a01b03831681526040602082015260008251606060408401526121db60a0840182612164565b90506020840151606084015260408401516080840152809150509392505050565b602081526000611beb6020830184612164565b60008235609e1983360301811261222557600080fd5b9190910192915050565b803561223a81611bb9565b919050565b60006020828403121561225157600080fd5b81518015158114611beb57600080fd5b8183526000602080850194508260005b858110156122bc57813561228481611bb9565b6001600160a01b031687528183013561229c81611fc0565b6001600160601b0316878401526040968701969190910190600101612271565b509495945050505050565b803561223a816120ab565b60208082528181018390526000906040808401600586901b8501820187855b888110156123d157878303603f190184528135368b9003609e1901811261231757600080fd5b8a0160a0813536839003601e1901811261233057600080fd5b820180356001600160401b0381111561234857600080fd5b8060061b360384131561235a57600080fd5b82875261236c838801828c8501612261565b9250505061237b88830161222f565b6001600160a01b0316888601528187013587860152606061239d8184016122c7565b63ffffffff169086015260806123b48382016122c7565b63ffffffff169501949094525092850192908501906001016122f1565b509098975050505050505050565b600061ffff8083168181036123f6576123f6611f14565b600101939250505056fea2646970667358221220fe7847586de1c3a1765ea908cb48dd2c9aeae31baa75013bc248b7b845180e0564736f6c634300080d0033",
}

// ContractFinalizerServiceManagerABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractFinalizerServiceManagerMetaData.ABI instead.
var ContractFinalizerServiceManagerABI = ContractFinalizerServiceManagerMetaData.ABI

// ContractFinalizerServiceManagerBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractFinalizerServiceManagerMetaData.Bin instead.
var ContractFinalizerServiceManagerBin = ContractFinalizerServiceManagerMetaData.Bin

// DeployContractFinalizerServiceManager deploys a new Ethereum contract, binding an instance of ContractFinalizerServiceManager to it.
func DeployContractFinalizerServiceManager(auth *bind.TransactOpts, backend bind.ContractBackend, _avsDirectory common.Address, _rewardsCoordinator common.Address, _registryCoordinator common.Address, _stakeRegistry common.Address, _taskManager common.Address, _recurrentRegistrationBlocksLimit uint64) (common.Address, *types.Transaction, *ContractFinalizerServiceManager, error) {
	parsed, err := ContractFinalizerServiceManagerMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractFinalizerServiceManagerBin), backend, _avsDirectory, _rewardsCoordinator, _registryCoordinator, _stakeRegistry, _taskManager, _recurrentRegistrationBlocksLimit)
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

// Ejector is a free data retrieval call binding the contract method 0x28f61b31.
//
// Solidity: function ejector() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCaller) Ejector(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerServiceManager.contract.Call(opts, &out, "ejector")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Ejector is a free data retrieval call binding the contract method 0x28f61b31.
//
// Solidity: function ejector() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) Ejector() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.Ejector(&_ContractFinalizerServiceManager.CallOpts)
}

// Ejector is a free data retrieval call binding the contract method 0x28f61b31.
//
// Solidity: function ejector() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCallerSession) Ejector() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.Ejector(&_ContractFinalizerServiceManager.CallOpts)
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

// RecurrentRegistrationBlocksLimit is a free data retrieval call binding the contract method 0x614cc144.
//
// Solidity: function recurrentRegistrationBlocksLimit() view returns(uint64)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCaller) RecurrentRegistrationBlocksLimit(opts *bind.CallOpts) (uint64, error) {
	var out []interface{}
	err := _ContractFinalizerServiceManager.contract.Call(opts, &out, "recurrentRegistrationBlocksLimit")

	if err != nil {
		return *new(uint64), err
	}

	out0 := *abi.ConvertType(out[0], new(uint64)).(*uint64)

	return out0, err

}

// RecurrentRegistrationBlocksLimit is a free data retrieval call binding the contract method 0x614cc144.
//
// Solidity: function recurrentRegistrationBlocksLimit() view returns(uint64)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) RecurrentRegistrationBlocksLimit() (uint64, error) {
	return _ContractFinalizerServiceManager.Contract.RecurrentRegistrationBlocksLimit(&_ContractFinalizerServiceManager.CallOpts)
}

// RecurrentRegistrationBlocksLimit is a free data retrieval call binding the contract method 0x614cc144.
//
// Solidity: function recurrentRegistrationBlocksLimit() view returns(uint64)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCallerSession) RecurrentRegistrationBlocksLimit() (uint64, error) {
	return _ContractFinalizerServiceManager.Contract.RecurrentRegistrationBlocksLimit(&_ContractFinalizerServiceManager.CallOpts)
}

// RewardsInitiator is a free data retrieval call binding the contract method 0xfc299dee.
//
// Solidity: function rewardsInitiator() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCaller) RewardsInitiator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractFinalizerServiceManager.contract.Call(opts, &out, "rewardsInitiator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// RewardsInitiator is a free data retrieval call binding the contract method 0xfc299dee.
//
// Solidity: function rewardsInitiator() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) RewardsInitiator() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.RewardsInitiator(&_ContractFinalizerServiceManager.CallOpts)
}

// RewardsInitiator is a free data retrieval call binding the contract method 0xfc299dee.
//
// Solidity: function rewardsInitiator() view returns(address)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerCallerSession) RewardsInitiator() (common.Address, error) {
	return _ContractFinalizerServiceManager.Contract.RewardsInitiator(&_ContractFinalizerServiceManager.CallOpts)
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

// CreateAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xfce36c7d.
//
// Solidity: function createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[] rewardsSubmissions) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) CreateAVSRewardsSubmission(opts *bind.TransactOpts, rewardsSubmissions []IRewardsCoordinatorRewardsSubmission) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "createAVSRewardsSubmission", rewardsSubmissions)
}

// CreateAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xfce36c7d.
//
// Solidity: function createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[] rewardsSubmissions) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) CreateAVSRewardsSubmission(rewardsSubmissions []IRewardsCoordinatorRewardsSubmission) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.CreateAVSRewardsSubmission(&_ContractFinalizerServiceManager.TransactOpts, rewardsSubmissions)
}

// CreateAVSRewardsSubmission is a paid mutator transaction binding the contract method 0xfce36c7d.
//
// Solidity: function createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[] rewardsSubmissions) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) CreateAVSRewardsSubmission(rewardsSubmissions []IRewardsCoordinatorRewardsSubmission) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.CreateAVSRewardsSubmission(&_ContractFinalizerServiceManager.TransactOpts, rewardsSubmissions)
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

// EjectOperators is a paid mutator transaction binding the contract method 0x1e25abfd.
//
// Solidity: function ejectOperators(address[] operators, bytes[] quorumNumbers) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) EjectOperators(opts *bind.TransactOpts, operators []common.Address, quorumNumbers [][]byte) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "ejectOperators", operators, quorumNumbers)
}

// EjectOperators is a paid mutator transaction binding the contract method 0x1e25abfd.
//
// Solidity: function ejectOperators(address[] operators, bytes[] quorumNumbers) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) EjectOperators(operators []common.Address, quorumNumbers [][]byte) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.EjectOperators(&_ContractFinalizerServiceManager.TransactOpts, operators, quorumNumbers)
}

// EjectOperators is a paid mutator transaction binding the contract method 0x1e25abfd.
//
// Solidity: function ejectOperators(address[] operators, bytes[] quorumNumbers) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) EjectOperators(operators []common.Address, quorumNumbers [][]byte) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.EjectOperators(&_ContractFinalizerServiceManager.TransactOpts, operators, quorumNumbers)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address initialOwner, address _rewardsInitiator, address _ejector) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) Initialize(opts *bind.TransactOpts, initialOwner common.Address, _rewardsInitiator common.Address, _ejector common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "initialize", initialOwner, _rewardsInitiator, _ejector)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address initialOwner, address _rewardsInitiator, address _ejector) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) Initialize(initialOwner common.Address, _rewardsInitiator common.Address, _ejector common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.Initialize(&_ContractFinalizerServiceManager.TransactOpts, initialOwner, _rewardsInitiator, _ejector)
}

// Initialize is a paid mutator transaction binding the contract method 0xc0c53b8b.
//
// Solidity: function initialize(address initialOwner, address _rewardsInitiator, address _ejector) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) Initialize(initialOwner common.Address, _rewardsInitiator common.Address, _ejector common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.Initialize(&_ContractFinalizerServiceManager.TransactOpts, initialOwner, _rewardsInitiator, _ejector)
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

// SetEjector is a paid mutator transaction binding the contract method 0x2cdd1e86.
//
// Solidity: function setEjector(address _ejector) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) SetEjector(opts *bind.TransactOpts, _ejector common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "setEjector", _ejector)
}

// SetEjector is a paid mutator transaction binding the contract method 0x2cdd1e86.
//
// Solidity: function setEjector(address _ejector) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) SetEjector(_ejector common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.SetEjector(&_ContractFinalizerServiceManager.TransactOpts, _ejector)
}

// SetEjector is a paid mutator transaction binding the contract method 0x2cdd1e86.
//
// Solidity: function setEjector(address _ejector) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) SetEjector(_ejector common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.SetEjector(&_ContractFinalizerServiceManager.TransactOpts, _ejector)
}

// SetRewardsInitiator is a paid mutator transaction binding the contract method 0x3bc28c8c.
//
// Solidity: function setRewardsInitiator(address newRewardsInitiator) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) SetRewardsInitiator(opts *bind.TransactOpts, newRewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "setRewardsInitiator", newRewardsInitiator)
}

// SetRewardsInitiator is a paid mutator transaction binding the contract method 0x3bc28c8c.
//
// Solidity: function setRewardsInitiator(address newRewardsInitiator) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) SetRewardsInitiator(newRewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.SetRewardsInitiator(&_ContractFinalizerServiceManager.TransactOpts, newRewardsInitiator)
}

// SetRewardsInitiator is a paid mutator transaction binding the contract method 0x3bc28c8c.
//
// Solidity: function setRewardsInitiator(address newRewardsInitiator) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) SetRewardsInitiator(newRewardsInitiator common.Address) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.SetRewardsInitiator(&_ContractFinalizerServiceManager.TransactOpts, newRewardsInitiator)
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

// UpdateAVSMetadataURI is a paid mutator transaction binding the contract method 0xa98fb355.
//
// Solidity: function updateAVSMetadataURI(string _metadataURI) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactor) UpdateAVSMetadataURI(opts *bind.TransactOpts, _metadataURI string) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.contract.Transact(opts, "updateAVSMetadataURI", _metadataURI)
}

// UpdateAVSMetadataURI is a paid mutator transaction binding the contract method 0xa98fb355.
//
// Solidity: function updateAVSMetadataURI(string _metadataURI) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerSession) UpdateAVSMetadataURI(_metadataURI string) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.UpdateAVSMetadataURI(&_ContractFinalizerServiceManager.TransactOpts, _metadataURI)
}

// UpdateAVSMetadataURI is a paid mutator transaction binding the contract method 0xa98fb355.
//
// Solidity: function updateAVSMetadataURI(string _metadataURI) returns()
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerTransactorSession) UpdateAVSMetadataURI(_metadataURI string) (*types.Transaction, error) {
	return _ContractFinalizerServiceManager.Contract.UpdateAVSMetadataURI(&_ContractFinalizerServiceManager.TransactOpts, _metadataURI)
}

// ContractFinalizerServiceManagerEjectorUpdatedIterator is returned from FilterEjectorUpdated and is used to iterate over the raw logs and unpacked data for EjectorUpdated events raised by the ContractFinalizerServiceManager contract.
type ContractFinalizerServiceManagerEjectorUpdatedIterator struct {
	Event *ContractFinalizerServiceManagerEjectorUpdated // Event containing the contract specifics and raw log

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
func (it *ContractFinalizerServiceManagerEjectorUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerServiceManagerEjectorUpdated)
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
		it.Event = new(ContractFinalizerServiceManagerEjectorUpdated)
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
func (it *ContractFinalizerServiceManagerEjectorUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerServiceManagerEjectorUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerServiceManagerEjectorUpdated represents a EjectorUpdated event raised by the ContractFinalizerServiceManager contract.
type ContractFinalizerServiceManagerEjectorUpdated struct {
	PrevEjector common.Address
	NewEjector  common.Address
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterEjectorUpdated is a free log retrieval operation binding the contract event 0x8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9.
//
// Solidity: event EjectorUpdated(address prevEjector, address newEjector)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) FilterEjectorUpdated(opts *bind.FilterOpts) (*ContractFinalizerServiceManagerEjectorUpdatedIterator, error) {

	logs, sub, err := _ContractFinalizerServiceManager.contract.FilterLogs(opts, "EjectorUpdated")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerServiceManagerEjectorUpdatedIterator{contract: _ContractFinalizerServiceManager.contract, event: "EjectorUpdated", logs: logs, sub: sub}, nil
}

// WatchEjectorUpdated is a free log subscription operation binding the contract event 0x8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9.
//
// Solidity: event EjectorUpdated(address prevEjector, address newEjector)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) WatchEjectorUpdated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerServiceManagerEjectorUpdated) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerServiceManager.contract.WatchLogs(opts, "EjectorUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerServiceManagerEjectorUpdated)
				if err := _ContractFinalizerServiceManager.contract.UnpackLog(event, "EjectorUpdated", log); err != nil {
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

// ParseEjectorUpdated is a log parse operation binding the contract event 0x8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9.
//
// Solidity: event EjectorUpdated(address prevEjector, address newEjector)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) ParseEjectorUpdated(log types.Log) (*ContractFinalizerServiceManagerEjectorUpdated, error) {
	event := new(ContractFinalizerServiceManagerEjectorUpdated)
	if err := _ContractFinalizerServiceManager.contract.UnpackLog(event, "EjectorUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
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

// ContractFinalizerServiceManagerRewardsInitiatorUpdatedIterator is returned from FilterRewardsInitiatorUpdated and is used to iterate over the raw logs and unpacked data for RewardsInitiatorUpdated events raised by the ContractFinalizerServiceManager contract.
type ContractFinalizerServiceManagerRewardsInitiatorUpdatedIterator struct {
	Event *ContractFinalizerServiceManagerRewardsInitiatorUpdated // Event containing the contract specifics and raw log

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
func (it *ContractFinalizerServiceManagerRewardsInitiatorUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractFinalizerServiceManagerRewardsInitiatorUpdated)
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
		it.Event = new(ContractFinalizerServiceManagerRewardsInitiatorUpdated)
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
func (it *ContractFinalizerServiceManagerRewardsInitiatorUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractFinalizerServiceManagerRewardsInitiatorUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractFinalizerServiceManagerRewardsInitiatorUpdated represents a RewardsInitiatorUpdated event raised by the ContractFinalizerServiceManager contract.
type ContractFinalizerServiceManagerRewardsInitiatorUpdated struct {
	PrevRewardsInitiator common.Address
	NewRewardsInitiator  common.Address
	Raw                  types.Log // Blockchain specific contextual infos
}

// FilterRewardsInitiatorUpdated is a free log retrieval operation binding the contract event 0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3.
//
// Solidity: event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) FilterRewardsInitiatorUpdated(opts *bind.FilterOpts) (*ContractFinalizerServiceManagerRewardsInitiatorUpdatedIterator, error) {

	logs, sub, err := _ContractFinalizerServiceManager.contract.FilterLogs(opts, "RewardsInitiatorUpdated")
	if err != nil {
		return nil, err
	}
	return &ContractFinalizerServiceManagerRewardsInitiatorUpdatedIterator{contract: _ContractFinalizerServiceManager.contract, event: "RewardsInitiatorUpdated", logs: logs, sub: sub}, nil
}

// WatchRewardsInitiatorUpdated is a free log subscription operation binding the contract event 0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3.
//
// Solidity: event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) WatchRewardsInitiatorUpdated(opts *bind.WatchOpts, sink chan<- *ContractFinalizerServiceManagerRewardsInitiatorUpdated) (event.Subscription, error) {

	logs, sub, err := _ContractFinalizerServiceManager.contract.WatchLogs(opts, "RewardsInitiatorUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractFinalizerServiceManagerRewardsInitiatorUpdated)
				if err := _ContractFinalizerServiceManager.contract.UnpackLog(event, "RewardsInitiatorUpdated", log); err != nil {
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

// ParseRewardsInitiatorUpdated is a log parse operation binding the contract event 0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3.
//
// Solidity: event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator)
func (_ContractFinalizerServiceManager *ContractFinalizerServiceManagerFilterer) ParseRewardsInitiatorUpdated(log types.Log) (*ContractFinalizerServiceManagerRewardsInitiatorUpdated, error) {
	event := new(ContractFinalizerServiceManagerRewardsInitiatorUpdated)
	if err := _ContractFinalizerServiceManager.contract.UnpackLog(event, "RewardsInitiatorUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

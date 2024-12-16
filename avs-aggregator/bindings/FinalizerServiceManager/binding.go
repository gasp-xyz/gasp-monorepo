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
	Bin: "0x6101406040523480156200001257600080fd5b50604051620026fc380380620026fc833981016040819052620000359162000168565b6001600160a01b0380871660805280861660a05280851660c052831660e05285858585620000626200008d565b5050506001600160a01b03909216610100526001600160401b03166101205250620002079350505050565b600054610100900460ff1615620000fa5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811610156200014d576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146200016557600080fd5b50565b60008060008060008060c087890312156200018257600080fd5b86516200018f816200014f565b6020880151909650620001a2816200014f565b6040880151909550620001b5816200014f565b6060880151909450620001c8816200014f565b6080880151909350620001db816200014f565b60a08801519092506001600160401b0381168114620001f957600080fd5b809150509295509295509295565b60805160a05160c05160e0516101005161012051612412620002ea600039600081816101ab0152610bff0152600061024f0152600081816106cf0152818161082a015281816108c101528181610b56015281816110800152818161120301526112a20152600081816103df015281816104fa0152818161058901528181610609015281816109d401528181610a3301528181610aa701528181610d7b01528181610fbb015261115e015260008181611543015281816115ff01526116eb0152600081816101e701528181610d0201528181610dd70152610e4f01526124126000f3fe608060405234801561001057600080fd5b50600436106101165760003560e01c80639926ee7d116100a2578063c0c53b8b11610071578063c0c53b8b14610284578063e481af9d14610297578063f2fde38b1461029f578063fc299dee146102b2578063fce36c7d146102c557600080fd5b80639926ee7d14610224578063a364f4da14610237578063a50a640e1461024a578063a98fb3551461027157600080fd5b80633bc28c8c116100e95780633bc28c8c14610193578063614cc144146101a65780636b3aa72e146101e5578063715018a61461020b5780638da5cb5b1461021357600080fd5b80631e25abfd1461011b57806328f61b31146101305780632cdd1e861461016057806333cfb7b714610173575b600080fd5b61012e610129366004611b2a565b6102d8565b005b609754610143906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b61012e61016e366004611baa565b6104c1565b610186610181366004611baa565b6104d5565b6040516101579190611bce565b61012e6101a1366004611baa565b6109a4565b6101cd7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160401b039091168152602001610157565b7f0000000000000000000000000000000000000000000000000000000000000000610143565b61012e6109b5565b6033546001600160a01b0316610143565b61012e610232366004611cce565b6109c9565b61012e610245366004611baa565b610d70565b6101437f000000000000000000000000000000000000000000000000000000000000000081565b61012e61027f366004611d78565b610e30565b61012e610292366004611dc8565b610e84565b610186610fb5565b61012e6102ad366004611baa565b611381565b606554610143906001600160a01b031681565b61012e6102d3366004611e13565b6113f7565b6097546001600160a01b0316331461035d5760405162461bcd60e51b815260206004820152603a60248201527f5265676973747279436f6f7264696e61746f722e6f6e6c79456a6563746f723a60448201527f2063616c6c6572206973206e6f742074686520656a6563746f7200000000000060648201526084015b60405180910390fd5b8281146103d25760405162461bcd60e51b815260206004820152603e60248201527f5265676973747279436f6f7264696e61746f722e656a6563744f70657261746f60448201527f72733a2061726773206c656e67746820646f6573206e6f74206d6174636800006064820152608401610354565b60005b838110156104ba577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636e3b17db86868481811061041e5761041e611e54565b90506020020160208101906104339190611baa565b85858581811061044557610445611e54565b90506020028101906104579190611e6a565b6040518463ffffffff1660e01b815260040161047593929190611eb0565b600060405180830381600087803b15801561048f57600080fd5b505af11580156104a3573d6000803e3d6000fd5b5050505080806104b290611f06565b9150506103d5565b5050505050565b6104c9611758565b6104d2816117b2565b50565b6040516309aa152760e11b81526001600160a01b0382811660048301526060916000917f000000000000000000000000000000000000000000000000000000000000000016906313542a4e90602401602060405180830381865afa158015610541573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105659190611f1f565b60405163871ef04960e01b8152600481018290529091506000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063871ef04990602401602060405180830381865afa1580156105d0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105f49190611f38565b90506001600160c01b038116158061068e57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610665573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106899190611f61565b60ff16155b156106aa57505060408051600081526020810190915292915050565b60006106be826001600160c01b031661181b565b90506000805b8251811015610794577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316633ca5a5f584838151811061070e5761070e611e54565b01602001516040516001600160e01b031960e084901b16815260f89190911c6004820152602401602060405180830381865afa158015610752573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107769190611f1f565b6107809083611f84565b91508061078c81611f06565b9150506106c4565b506000816001600160401b038111156107af576107af611c1b565b6040519080825280602002602001820160405280156107d8578160200160208202803683370190505b5090506000805b84518110156109975760008582815181106107fc576107fc611e54565b0160200151604051633ca5a5f560e01b815260f89190911c6004820181905291506000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633ca5a5f590602401602060405180830381865afa158015610871573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108959190611f1f565b905060005b81811015610981576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa15801561090f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109339190611fb1565b6000015186868151811061094957610949611e54565b6001600160a01b03909216602092830291909101909101528461096b81611f06565b955050808061097990611f06565b91505061089a565b505050808061098f90611f06565b9150506107df565b5090979650505050505050565b6109ac611758565b6104d2816118dd565b6109bd611758565b6109c76000611946565b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a115760405162461bcd60e51b81526004016103549061200f565b6040516309aa152760e11b81526001600160a01b0383811660048301526000917f0000000000000000000000000000000000000000000000000000000000000000909116906313542a4e90602401602060405180830381865afa158015610a7c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610aa09190611f1f565b905060005b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b03573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b279190611f61565b60ff168160ff161015610cea57604051631f0a3c3360e31b81526004810183905260ff821660248201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063f851e19890604401606060405180830381865afa158015610ba5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bc99190612099565b905080604001516001600160601b03166000148015610bee5750805163ffffffff1615155b15610cd75780516001600160401b037f00000000000000000000000000000000000000000000000000000000000000001690610c309063ffffffff164361210a565b11610cd75760405162461bcd60e51b815260206004820152606560248201527f536572766963654d616e616765722e72656769737465724f70657261746f725460448201527f6f4156533a206d696e696d756d20626c6f636b7320656c6170736564206c696d60648201527f697420666f7220726563757272656e7420726567697374726174696f6e206e6f6084820152641d081b595d60da1b60a482015260c401610354565b5080610ce281612121565b915050610aa5565b50604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d90610d39908690869060040161218d565b600060405180830381600087803b158015610d5357600080fd5b505af1158015610d67573d6000803e3d6000fd5b50505050505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610db85760405162461bcd60e51b81526004016103549061200f565b6040516351b27a6d60e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da906024015b600060405180830381600087803b158015610e1c57600080fd5b505af11580156104ba573d6000803e3d6000fd5b610e38611758565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb35590610e029084906004016121d8565b600054610100900460ff1615808015610ea45750600054600160ff909116105b80610ebe5750303b158015610ebe575060005460ff166001145b610f215760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610354565b6000805460ff191660011790558015610f44576000805461ff0019166101001790555b610f4e8484611998565b609780546001600160a01b0319166001600160a01b0384161790558015610faf576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b606060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611017573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061103b9190611f61565b60ff1690508060000361105c57505060408051600081526020810190915290565b6000805b8281101561111157604051633ca5a5f560e01b815260ff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa1580156110cf573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906110f39190611f1f565b6110fd9083611f84565b91508061110981611f06565b915050611060565b506000816001600160401b0381111561112c5761112c611c1b565b604051908082528060200260200182016040528015611155578160200160208202803683370190505b5090506000805b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639aa1653d6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156111ba573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906111de9190611f61565b60ff1681101561137757604051633ca5a5f560e01b815260ff821660048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633ca5a5f590602401602060405180830381865afa158015611252573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112769190611f1f565b905060005b81811015611362576040516356e4026d60e11b815260ff84166004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063adc804da906044016040805180830381865afa1580156112f0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113149190611fb1565b6000015185858151811061132a5761132a611e54565b6001600160a01b03909216602092830291909101909101528361134c81611f06565b945050808061135a90611f06565b91505061127b565b5050808061136f90611f06565b91505061115c565b5090949350505050565b611389611758565b6001600160a01b0381166113ee5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610354565b6104d281611946565b6113ff611a19565b60005b818110156116d35782828281811061141c5761141c611e54565b905060200281019061142e91906121eb565b61143f906040810190602001611baa565b6001600160a01b03166323b872dd333086868681811061146157611461611e54565b905060200281019061147391906121eb565b604080516001600160e01b031960e087901b1681526001600160a01b039485166004820152939092166024840152013560448201526064016020604051808303816000875af11580156114ca573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114ee919061221b565b50600083838381811061150357611503611e54565b905060200281019061151591906121eb565b611526906040810190602001611baa565b604051636eb1769f60e11b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166024830152919091169063dd62ed3e90604401602060405180830381865afa158015611594573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115b89190611f1f565b90508383838181106115cc576115cc611e54565b90506020028101906115de91906121eb565b6115ef906040810190602001611baa565b6001600160a01b031663095ea7b37f00000000000000000000000000000000000000000000000000000000000000008387878781811061163157611631611e54565b905060200281019061164391906121eb565b604001356116519190611f84565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af115801561169c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116c0919061221b565b5050806116cc90611f06565b9050611402565b5060405163fce36c7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063fce36c7d9061172290859085906004016122ae565b600060405180830381600087803b15801561173c57600080fd5b505af1158015611750573d6000803e3d6000fd5b505050505050565b6033546001600160a01b031633146109c75760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610354565b609754604080516001600160a01b03928316815291831660208301527f8f30ab09f43a6c157d7fce7e0a13c003042c1c95e8a72e7a146a21c0caa24dc9910160405180910390a1609780546001600160a01b0319166001600160a01b0392909216919091179055565b606060008061182984611aae565b61ffff166001600160401b0381111561184457611844611c1b565b6040519080825280601f01601f19166020018201604052801561186e576020820181803683370190505b5090506000805b825182108015611886575061010081105b15611377576001811b9350858416156118cd578060f81b8383815181106118af576118af611e54565b60200101906001600160f81b031916908160001a9053508160010191505b6118d681611f06565b9050611875565b606554604080516001600160a01b03928316815291831660208301527fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b600054610100900460ff16611a035760405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608401610354565b611a0c82611946565b611a15816118dd565b5050565b6065546001600160a01b031633146109c75760405162461bcd60e51b815260206004820152604c60248201527f536572766963654d616e61676572426173652e6f6e6c7952657761726473496e60448201527f69746961746f723a2063616c6c6572206973206e6f742074686520726577617260648201526b32399034b734ba34b0ba37b960a11b608482015260a401610354565b6000805b8215611ad957611ac360018461210a565b9092169180611ad1816123bb565b915050611ab2565b92915050565b60008083601f840112611af157600080fd5b5081356001600160401b03811115611b0857600080fd5b6020830191508360208260051b8501011115611b2357600080fd5b9250929050565b60008060008060408587031215611b4057600080fd5b84356001600160401b0380821115611b5757600080fd5b611b6388838901611adf565b90965094506020870135915080821115611b7c57600080fd5b50611b8987828801611adf565b95989497509550505050565b6001600160a01b03811681146104d257600080fd5b600060208284031215611bbc57600080fd5b8135611bc781611b95565b9392505050565b6020808252825182820181905260009190848201906040850190845b81811015611c0f5783516001600160a01b031683529284019291840191600101611bea565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b604051606081016001600160401b0381118282101715611c5357611c53611c1b565b60405290565b60006001600160401b0380841115611c7357611c73611c1b565b604051601f8501601f19908116603f01168101908282118183101715611c9b57611c9b611c1b565b81604052809350858152868686011115611cb457600080fd5b858560208301376000602087830101525050509392505050565b60008060408385031215611ce157600080fd5b8235611cec81611b95565b915060208301356001600160401b0380821115611d0857600080fd5b9084019060608287031215611d1c57600080fd5b611d24611c31565b823582811115611d3357600080fd5b83019150601f82018713611d4657600080fd5b611d5587833560208501611c59565b815260208301356020820152604083013560408201528093505050509250929050565b600060208284031215611d8a57600080fd5b81356001600160401b03811115611da057600080fd5b8201601f81018413611db157600080fd5b611dc084823560208401611c59565b949350505050565b600080600060608486031215611ddd57600080fd5b8335611de881611b95565b92506020840135611df881611b95565b91506040840135611e0881611b95565b809150509250925092565b60008060208385031215611e2657600080fd5b82356001600160401b03811115611e3c57600080fd5b611e4885828601611adf565b90969095509350505050565b634e487b7160e01b600052603260045260246000fd5b6000808335601e19843603018112611e8157600080fd5b8301803591506001600160401b03821115611e9b57600080fd5b602001915036819003821315611b2357600080fd5b6001600160a01b03841681526040602082018190528101829052818360608301376000818301606090810191909152601f909201601f1916010192915050565b634e487b7160e01b600052601160045260246000fd5b600060018201611f1857611f18611ef0565b5060010190565b600060208284031215611f3157600080fd5b5051919050565b600060208284031215611f4a57600080fd5b81516001600160c01b0381168114611bc757600080fd5b600060208284031215611f7357600080fd5b815160ff81168114611bc757600080fd5b60008219821115611f9757611f97611ef0565b500190565b6001600160601b03811681146104d257600080fd5b600060408284031215611fc357600080fd5b604051604081018181106001600160401b0382111715611fe557611fe5611c1b565b6040528251611ff381611b95565b8152602083015161200381611f9c565b60208201529392505050565b60208082526052908201527f536572766963654d616e61676572426173652e6f6e6c7952656769737472794360408201527f6f6f7264696e61746f723a2063616c6c6572206973206e6f742074686520726560608201527133b4b9ba393c9031b7b7b93234b730ba37b960711b608082015260a00190565b63ffffffff811681146104d257600080fd5b6000606082840312156120ab57600080fd5b604051606081018181106001600160401b03821117156120cd576120cd611c1b565b60405282516120db81612087565b815260208301516120eb81612087565b602082015260408301516120fe81611f9c565b60408201529392505050565b60008282101561211c5761211c611ef0565b500390565b600060ff821660ff810361213757612137611ef0565b60010192915050565b6000815180845260005b818110156121665760208185018101518683018201520161214a565b81811115612178576000602083870101525b50601f01601f19169290920160200192915050565b60018060a01b03831681526040602082015260008251606060408401526121b760a0840182612140565b90506020840151606084015260408401516080840152809150509392505050565b602081526000611bc76020830184612140565b60008235609e1983360301811261220157600080fd5b9190910192915050565b803561221681611b95565b919050565b60006020828403121561222d57600080fd5b81518015158114611bc757600080fd5b8183526000602080850194508260005b8581101561229857813561226081611b95565b6001600160a01b031687528183013561227881611f9c565b6001600160601b031687840152604096870196919091019060010161224d565b509495945050505050565b803561221681612087565b60208082528181018390526000906040808401600586901b8501820187855b888110156123ad57878303603f190184528135368b9003609e190181126122f357600080fd5b8a0160a0813536839003601e1901811261230c57600080fd5b820180356001600160401b0381111561232457600080fd5b8060061b360384131561233657600080fd5b828752612348838801828c850161223d565b9250505061235788830161220b565b6001600160a01b031688860152818701358786015260606123798184016122a3565b63ffffffff169086015260806123908382016122a3565b63ffffffff169501949094525092850192908501906001016122cd565b509098975050505050505050565b600061ffff8083168181036123d2576123d2611ef0565b600101939250505056fea26469706673582212207c33c7fae0f9498cea676be6c2583448dff789e6de5726ee3f03c75ccac1625a64736f6c634300080d0033",
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

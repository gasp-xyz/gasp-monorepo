// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package contractOperatorStateRetrieverExtended

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

// OperatorStateRetrieverCheckSignaturesIndices is an auto generated low-level Go binding around an user-defined struct.
type OperatorStateRetrieverCheckSignaturesIndices struct {
	NonSignerQuorumBitmapIndices []uint32
	QuorumApkIndices             []uint32
	TotalStakeIndices            []uint32
	NonSignerStakeIndices        [][]uint32
}

// OperatorStateRetrieverOperator is an auto generated low-level Go binding around an user-defined struct.
type OperatorStateRetrieverOperator struct {
	Operator   common.Address
	OperatorId [32]byte
	Stake      *big.Int
}

// ContractOperatorStateRetrieverExtendedMetaData contains all meta data concerning the ContractOperatorStateRetrieverExtended contract.
var ContractOperatorStateRetrieverExtendedMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"function\",\"name\":\"getCheckSignaturesIndices\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"referenceBlockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"nonSignerOperatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structOperatorStateRetriever.CheckSignaturesIndices\",\"components\":[{\"name\":\"nonSignerQuorumBitmapIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"quorumApkIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"totalStakeIndices\",\"type\":\"uint32[]\",\"internalType\":\"uint32[]\"},{\"name\":\"nonSignerStakeIndices\",\"type\":\"uint32[][]\",\"internalType\":\"uint32[][]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorIdQuorums\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorState\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorsFromIds\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getOperatorsStakesForQuorum\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"operatorsAddr\",\"type\":\"address[]\",\"internalType\":\"address[]\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple[][]\",\"internalType\":\"structOperatorStateRetriever.Operator[][]\",\"components\":[{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"stake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getQuorumBitmapsAtBlockNumber\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"operatorIds\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"blockNumber\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256[]\",\"internalType\":\"uint256[]\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorStakeForQuorum\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorStakeForQuorumClipped\",\"inputs\":[{\"name\":\"registryCoordinator\",\"type\":\"address\",\"internalType\":\"contractIRegistryCoordinator\"},{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"operator\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"}],\"stateMutability\":\"view\"}]",
	Bin: "0x608060405234801561001057600080fd5b50612635806100206000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c80634f739f74116100665780634f739f74146101145780635c15566214610134578063ae7b27bb14610154578063c6a867ed1461017f578063cefdc1d4146101b157600080fd5b8063060564321461009857806315a9bd85146100c157806322094e1c146100e15780633563b0d114610101575b600080fd5b6100ab6100a6366004611b1c565b6101d2565b6040516100b89190611b48565b60405180910390f35b6100d46100cf366004611be8565b610260565b6040516100b89190611c3c565b6100f46100ef366004611cca565b61037f565b6040516100b89190611df0565b6100f461010f366004611e72565b6107cb565b610127610122366004611f29565b610c61565b6040516100b89190611ffe565b6101476101423660046120dc565b61138c565b6040516100b8919061218d565b6101676101623660046121c5565b611554565b6040516001600160601b0390911681526020016100b8565b61019261018d3660046121c5565b611579565b604080516001600160601b0390931683529015156020830152016100b8565b6101c46101bf366004612216565b6116e8565b6040516100b892919061224d565b60405163871ef04960e01b8152600481018290526060906000906001600160a01b0385169063871ef04990602401602060405180830381865afa15801561021d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610241919061226e565b90506000610257826001600160c01b031661187a565b95945050505050565b60606000826001600160401b0381111561027c5761027c611e0a565b6040519080825280602002602001820160405280156102a5578160200160208202803683370190505b50905060005b8381101561037657856001600160a01b031663296bb0648686848181106102d4576102d4612297565b905060200201356040518263ffffffff1660e01b81526004016102f991815260200190565b602060405180830381865afa158015610316573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061033a91906122ad565b82828151811061034c5761034c612297565b6001600160a01b03909216602092830291909101909101528061036e816122e0565b9150506102ab565b50949350505050565b60606000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156103c1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103e591906122ad565b90506000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610427573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061044b91906122ad565b90506000866001600160401b0381111561046757610467611e0a565b60405190808252806020026020018201604052801561049a57816020015b60608152602001906001900390816104855790505b50905060005b878110156107be5760008989838181106104bc576104bc612297565b919091013560f81c9150606090508a8a848181106104dc576104dc612297565b9050013560f81c60f81b816000815181106104f9576104f9612297565b60200101906001600160f81b031916908160001a905350876001600160401b0381111561052857610528611e0a565b60405190808252806020026020018201604052801561057357816020015b60408051606081018252600080825260208083018290529282015282526000199092019101816105465790505b5084848151811061058657610586612297565b602002602001018190525060005b888110156107a8576000876001600160a01b03166313542a4e8c8c858181106105bf576105bf612297565b90506020020160208101906105d491906122fb565b6040516001600160e01b031960e084901b1681526001600160a01b039091166004820152602401602060405180830381865afa158015610618573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061063c9190612318565b9050600061066c8f868e8e8781811061065757610657612297565b905060200201602081019061016291906122fb565b905081610677575060005b6107028f6001600160a01b031663871ef049846040518263ffffffff1660e01b81526004016106a891815260200190565b602060405180830381865afa1580156106c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106e9919061226e565b6001600160c01b03166106fb86611946565b9081161490565b61070a575060005b60405180606001604052808d8d8681811061072757610727612297565b905060200201602081019061073c91906122fb565b6001600160a01b03168152602001838152602001826001600160601b031681525087878151811061076f5761076f612297565b6020026020010151848151811061078857610788612297565b6020026020010181905250505080806107a0906122e0565b915050610594565b50505080806107b6906122e0565b9150506104a0565b5098975050505050505050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa15801561080d573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061083191906122ad565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa158015610873573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089791906122ad565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156108d9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108fd91906122ad565b9050600086516001600160401b0381111561091a5761091a611e0a565b60405190808252806020026020018201604052801561094d57816020015b60608152602001906001900390816109385790505b50905060005b8751811015610c5557600088828151811061097057610970612297565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa1580156109d1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526109f99190810190612331565b905080516001600160401b03811115610a1457610a14611e0a565b604051908082528060200260200182016040528015610a5f57816020015b6040805160608101825260008082526020808301829052928201528252600019909201910181610a325790505b50848481518110610a7257610a72612297565b602002602001018190525060005b8151811015610c3f576040518060600160405280876001600160a01b03166347b314e8858581518110610ab557610ab5612297565b60200260200101516040518263ffffffff1660e01b8152600401610adb91815260200190565b602060405180830381865afa158015610af8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b1c91906122ad565b6001600160a01b03168152602001838381518110610b3c57610b3c612297565b60200260200101518152602001896001600160a01b031663fa28c627858581518110610b6a57610b6a612297565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa158015610bc6573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bea91906123c1565b6001600160601b0316815250858581518110610c0857610c08612297565b60200260200101518281518110610c2157610c21612297565b60200260200101819052508080610c37906122e0565b915050610a80565b5050508080610c4d906122e0565b915050610953565b50979650505050505050565b610c8c6040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ccc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cf091906122ad565b9050610d1d6040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e90610d4d908b90899089906004016123ea565b600060405180830381865afa158015610d6a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610d929190810190612434565b81526040516340e03a8160e11b81526001600160a01b038316906381c0750290610dc4908b908b908b906004016124eb565b600060405180830381865afa158015610de1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e099190810190612434565b6040820152856001600160401b03811115610e2657610e26611e0a565b604051908082528060200260200182016040528015610e5957816020015b6060815260200190600190039081610e445790505b50606082015260005b60ff811687111561129d576000856001600160401b03811115610e8757610e87611e0a565b604051908082528060200260200182016040528015610eb0578160200160208202803683370190505b5083606001518360ff1681518110610eca57610eca612297565b602002602001018190525060005b8681101561119d5760008c6001600160a01b03166304ec63518a8a85818110610f0357610f03612297565b905060200201358e88600001518681518110610f2157610f21612297565b60200260200101516040518463ffffffff1660e01b8152600401610f5e9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa158015610f7b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f9f919061226e565b90506001600160c01b0381166110485760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a4015b60405180910390fd5b8a8a8560ff1681811061105d5761105d612297565b6001600160c01b03841692013560f81c9190911c60019081161415905061118a57856001600160a01b031663dd9846b98a8a8581811061109f5761109f612297565b905060200201358d8d8860ff168181106110bb576110bb612297565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa158015611111573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611135919061250b565b85606001518560ff168151811061114e5761114e612297565b6020026020010151848151811061116757611167612297565b63ffffffff9092166020928302919091019091015282611186816122e0565b9350505b5080611195816122e0565b915050610ed8565b506000816001600160401b038111156111b8576111b8611e0a565b6040519080825280602002602001820160405280156111e1578160200160208202803683370190505b50905060005b828110156112625784606001518460ff168151811061120857611208612297565b6020026020010151818151811061122157611221612297565b602002602001015182828151811061123b5761123b612297565b63ffffffff909216602092830291909101909101528061125a816122e0565b9150506111e7565b508084606001518460ff168151811061127d5761127d612297565b60200260200101819052505050808061129590612528565b915050610e62565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156112de573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061130291906122ad565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c90611335908b908b908e90600401612548565b600060405180830381865afa158015611352573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261137a9190810190612434565b60208301525098975050505050505050565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b81526004016113be929190612572565b600060405180830381865afa1580156113db573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526114039190810190612434565b9050600084516001600160401b0381111561142057611420611e0a565b604051908082528060200260200182016040528015611449578160200160208202803683370190505b50905060005b855181101561154a57866001600160a01b03166304ec635187838151811061147957611479612297565b60200260200101518786858151811061149457611494612297565b60200260200101516040518463ffffffff1660e01b81526004016114d19392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa1580156114ee573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611512919061226e565b6001600160c01b031682828151811061152d5761152d612297565b602090810291909101015280611542816122e0565b91505061144f565b5095945050505050565b6000806000611564868686611579565b91509150806103765750600095945050505050565b6000806000856001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115bc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115e091906122ad565b60405162fcdba760e51b815260ff871660048201526001600160a01b038681166024830152919250600091831690631f9b74e090604401602060405180830381865afa158015611634573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061165891906123c1565b60405163c46778a560e01b815260ff881660048201529091506000906001600160a01b0384169063c46778a590602401602060405180830381865afa1580156116a5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116c991906123c1565b919450506001600160601b039081169084161015915050935093915050565b604080516001808252818301909252600091606091839160208083019080368337019050509050848160008151811061172357611723612297565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e9061175f9088908690600401612572565b600060405180830381865afa15801561177c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117a49190810190612434565b6000815181106117b6576117b6612297565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa158015611822573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611846919061226e565b6001600160c01b03169050600061185c8261187a565b90508161186a8a838a6107cb565b9550955050505050935093915050565b606060008061188884611ad3565b61ffff166001600160401b038111156118a3576118a3611e0a565b6040519080825280601f01601f1916602001820160405280156118cd576020820181803683370190505b5090506000805b8251821080156118e5575061010081105b1561193c576001811b93508584161561192c578060f81b83838151811061190e5761190e612297565b60200101906001600160f81b031916908160001a9053508160010191505b611935816122e0565b90506118d4565b5090949350505050565b6000610100825111156119cf5760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a40161103f565b81516119dd57506000919050565b600080836000815181106119f3576119f3612297565b0160200151600160f89190911c81901b92505b8451811015611aca57848181518110611a2157611a21612297565b0160200151600160f89190911c1b9150828211611ab65760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a40161103f565b91811791611ac3816122e0565b9050611a06565b50909392505050565b6000805b8215611afe57611ae86001846125c6565b9092169180611af6816125dd565b915050611ad7565b92915050565b6001600160a01b0381168114611b1957600080fd5b50565b60008060408385031215611b2f57600080fd5b8235611b3a81611b04565b946020939093013593505050565b600060208083528351808285015260005b81811015611b7557858101830151858201604001528201611b59565b81811115611b87576000604083870101525b50601f01601f1916929092016040019392505050565b60008083601f840112611baf57600080fd5b5081356001600160401b03811115611bc657600080fd5b6020830191508360208260051b8501011115611be157600080fd5b9250929050565b600080600060408486031215611bfd57600080fd5b8335611c0881611b04565b925060208401356001600160401b03811115611c2357600080fd5b611c2f86828701611b9d565b9497909650939450505050565b6020808252825182820181905260009190848201906040850190845b81811015611c7d5783516001600160a01b031683529284019291840191600101611c58565b50909695505050505050565b60008083601f840112611c9b57600080fd5b5081356001600160401b03811115611cb257600080fd5b602083019150836020828501011115611be157600080fd5b600080600080600060608688031215611ce257600080fd5b8535611ced81611b04565b945060208601356001600160401b0380821115611d0957600080fd5b611d1589838a01611c89565b90965094506040880135915080821115611d2e57600080fd5b50611d3b88828901611b9d565b969995985093965092949392505050565b600081518084526020808501808196508360051b810191508286016000805b86811015611de2578385038a52825180518087529087019087870190845b81811015611dcd57835180516001600160a01b031684528a8101518b8501526040908101516001600160601b03169084015292890192606090920191600101611d89565b50509a87019a95505091850191600101611d6b565b509298975050505050505050565b602081526000611e036020830184611d4c565b9392505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b0381118282101715611e4857611e48611e0a565b604052919050565b63ffffffff81168114611b1957600080fd5b8035611e6d81611e50565b919050565b600080600060608486031215611e8757600080fd5b8335611e9281611b04565b92506020848101356001600160401b0380821115611eaf57600080fd5b818701915087601f830112611ec357600080fd5b813581811115611ed557611ed5611e0a565b611ee7601f8201601f19168501611e20565b91508082528884828501011115611efd57600080fd5b8084840185840137600084828401015250809450505050611f2060408501611e62565b90509250925092565b60008060008060008060808789031215611f4257600080fd5b8635611f4d81611b04565b95506020870135611f5d81611e50565b945060408701356001600160401b0380821115611f7957600080fd5b611f858a838b01611c89565b90965094506060890135915080821115611f9e57600080fd5b50611fab89828a01611b9d565b979a9699509497509295939492505050565b600081518084526020808501945080840160005b83811015611ff357815163ffffffff1687529582019590820190600101611fd1565b509495945050505050565b60006020808352835160808285015261201a60a0850182611fbd565b905081850151601f19808684030160408701526120378383611fbd565b925060408701519150808684030160608701526120548383611fbd565b60608801518782038301608089015280518083529194508501925084840190600581901b8501860160005b828110156120ab5784878303018452612099828751611fbd565b9588019593880193915060010161207f565b509998505050505050505050565b60006001600160401b038211156120d2576120d2611e0a565b5060051b60200190565b6000806000606084860312156120f157600080fd5b83356120fc81611b04565b92506020848101356001600160401b0381111561211857600080fd5b8501601f8101871361212957600080fd5b803561213c612137826120b9565b611e20565b81815260059190911b8201830190838101908983111561215b57600080fd5b928401925b8284101561217957833582529284019290840190612160565b8096505050505050611f2060408501611e62565b6020808252825182820181905260009190848201906040850190845b81811015611c7d578351835292840192918401916001016121a9565b6000806000606084860312156121da57600080fd5b83356121e581611b04565b9250602084013560ff811681146121fb57600080fd5b9150604084013561220b81611b04565b809150509250925092565b60008060006060848603121561222b57600080fd5b833561223681611b04565b925060208401359150604084013561220b81611e50565b8281526040602082015260006122666040830184611d4c565b949350505050565b60006020828403121561228057600080fd5b81516001600160c01b0381168114611e0357600080fd5b634e487b7160e01b600052603260045260246000fd5b6000602082840312156122bf57600080fd5b8151611e0381611b04565b634e487b7160e01b600052601160045260246000fd5b60006000198214156122f4576122f46122ca565b5060010190565b60006020828403121561230d57600080fd5b8135611e0381611b04565b60006020828403121561232a57600080fd5b5051919050565b6000602080838503121561234457600080fd5b82516001600160401b0381111561235a57600080fd5b8301601f8101851361236b57600080fd5b8051612379612137826120b9565b81815260059190911b8201830190838101908783111561239857600080fd5b928401925b828410156123b65783518252928401929084019061239d565b979650505050505050565b6000602082840312156123d357600080fd5b81516001600160601b0381168114611e0357600080fd5b63ffffffff84168152604060208201819052810182905260006001600160fb1b0383111561241757600080fd5b8260051b8085606085013760009201606001918252509392505050565b6000602080838503121561244757600080fd5b82516001600160401b0381111561245d57600080fd5b8301601f8101851361246e57600080fd5b805161247c612137826120b9565b81815260059190911b8201830190838101908783111561249b57600080fd5b928401925b828410156123b65783516124b381611e50565b825292840192908401906124a0565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b63ffffffff841681526040602082015260006102576040830184866124c2565b60006020828403121561251d57600080fd5b8151611e0381611e50565b600060ff821660ff81141561253f5761253f6122ca565b60010192915050565b60408152600061255c6040830185876124c2565b905063ffffffff83166020830152949350505050565b60006040820163ffffffff851683526020604081850152818551808452606086019150828701935060005b818110156125b95784518352938301939183019160010161259d565b5090979650505050505050565b6000828210156125d8576125d86122ca565b500390565b600061ffff808316818114156125f5576125f56122ca565b600101939250505056fea26469706673582212200d044af7655409f5e1094b2d33c2e59f5310823f0e57dcde531498957f1ad39b64736f6c634300080c0033",
}

// ContractOperatorStateRetrieverExtendedABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractOperatorStateRetrieverExtendedMetaData.ABI instead.
var ContractOperatorStateRetrieverExtendedABI = ContractOperatorStateRetrieverExtendedMetaData.ABI

// ContractOperatorStateRetrieverExtendedBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractOperatorStateRetrieverExtendedMetaData.Bin instead.
var ContractOperatorStateRetrieverExtendedBin = ContractOperatorStateRetrieverExtendedMetaData.Bin

// DeployContractOperatorStateRetrieverExtended deploys a new Ethereum contract, binding an instance of ContractOperatorStateRetrieverExtended to it.
func DeployContractOperatorStateRetrieverExtended(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *ContractOperatorStateRetrieverExtended, error) {
	parsed, err := ContractOperatorStateRetrieverExtendedMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractOperatorStateRetrieverExtendedBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &ContractOperatorStateRetrieverExtended{ContractOperatorStateRetrieverExtendedCaller: ContractOperatorStateRetrieverExtendedCaller{contract: contract}, ContractOperatorStateRetrieverExtendedTransactor: ContractOperatorStateRetrieverExtendedTransactor{contract: contract}, ContractOperatorStateRetrieverExtendedFilterer: ContractOperatorStateRetrieverExtendedFilterer{contract: contract}}, nil
}

// ContractOperatorStateRetrieverExtended is an auto generated Go binding around an Ethereum contract.
type ContractOperatorStateRetrieverExtended struct {
	ContractOperatorStateRetrieverExtendedCaller     // Read-only binding to the contract
	ContractOperatorStateRetrieverExtendedTransactor // Write-only binding to the contract
	ContractOperatorStateRetrieverExtendedFilterer   // Log filterer for contract events
}

// ContractOperatorStateRetrieverExtendedCaller is an auto generated read-only Go binding around an Ethereum contract.
type ContractOperatorStateRetrieverExtendedCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractOperatorStateRetrieverExtendedTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ContractOperatorStateRetrieverExtendedTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractOperatorStateRetrieverExtendedFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ContractOperatorStateRetrieverExtendedFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractOperatorStateRetrieverExtendedSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ContractOperatorStateRetrieverExtendedSession struct {
	Contract     *ContractOperatorStateRetrieverExtended // Generic contract binding to set the session for
	CallOpts     bind.CallOpts                           // Call options to use throughout this session
	TransactOpts bind.TransactOpts                       // Transaction auth options to use throughout this session
}

// ContractOperatorStateRetrieverExtendedCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ContractOperatorStateRetrieverExtendedCallerSession struct {
	Contract *ContractOperatorStateRetrieverExtendedCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                                 // Call options to use throughout this session
}

// ContractOperatorStateRetrieverExtendedTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ContractOperatorStateRetrieverExtendedTransactorSession struct {
	Contract     *ContractOperatorStateRetrieverExtendedTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                                 // Transaction auth options to use throughout this session
}

// ContractOperatorStateRetrieverExtendedRaw is an auto generated low-level Go binding around an Ethereum contract.
type ContractOperatorStateRetrieverExtendedRaw struct {
	Contract *ContractOperatorStateRetrieverExtended // Generic contract binding to access the raw methods on
}

// ContractOperatorStateRetrieverExtendedCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ContractOperatorStateRetrieverExtendedCallerRaw struct {
	Contract *ContractOperatorStateRetrieverExtendedCaller // Generic read-only contract binding to access the raw methods on
}

// ContractOperatorStateRetrieverExtendedTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ContractOperatorStateRetrieverExtendedTransactorRaw struct {
	Contract *ContractOperatorStateRetrieverExtendedTransactor // Generic write-only contract binding to access the raw methods on
}

// NewContractOperatorStateRetrieverExtended creates a new instance of ContractOperatorStateRetrieverExtended, bound to a specific deployed contract.
func NewContractOperatorStateRetrieverExtended(address common.Address, backend bind.ContractBackend) (*ContractOperatorStateRetrieverExtended, error) {
	contract, err := bindContractOperatorStateRetrieverExtended(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ContractOperatorStateRetrieverExtended{ContractOperatorStateRetrieverExtendedCaller: ContractOperatorStateRetrieverExtendedCaller{contract: contract}, ContractOperatorStateRetrieverExtendedTransactor: ContractOperatorStateRetrieverExtendedTransactor{contract: contract}, ContractOperatorStateRetrieverExtendedFilterer: ContractOperatorStateRetrieverExtendedFilterer{contract: contract}}, nil
}

// NewContractOperatorStateRetrieverExtendedCaller creates a new read-only instance of ContractOperatorStateRetrieverExtended, bound to a specific deployed contract.
func NewContractOperatorStateRetrieverExtendedCaller(address common.Address, caller bind.ContractCaller) (*ContractOperatorStateRetrieverExtendedCaller, error) {
	contract, err := bindContractOperatorStateRetrieverExtended(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ContractOperatorStateRetrieverExtendedCaller{contract: contract}, nil
}

// NewContractOperatorStateRetrieverExtendedTransactor creates a new write-only instance of ContractOperatorStateRetrieverExtended, bound to a specific deployed contract.
func NewContractOperatorStateRetrieverExtendedTransactor(address common.Address, transactor bind.ContractTransactor) (*ContractOperatorStateRetrieverExtendedTransactor, error) {
	contract, err := bindContractOperatorStateRetrieverExtended(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ContractOperatorStateRetrieverExtendedTransactor{contract: contract}, nil
}

// NewContractOperatorStateRetrieverExtendedFilterer creates a new log filterer instance of ContractOperatorStateRetrieverExtended, bound to a specific deployed contract.
func NewContractOperatorStateRetrieverExtendedFilterer(address common.Address, filterer bind.ContractFilterer) (*ContractOperatorStateRetrieverExtendedFilterer, error) {
	contract, err := bindContractOperatorStateRetrieverExtended(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ContractOperatorStateRetrieverExtendedFilterer{contract: contract}, nil
}

// bindContractOperatorStateRetrieverExtended binds a generic wrapper to an already deployed contract.
func bindContractOperatorStateRetrieverExtended(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := ContractOperatorStateRetrieverExtendedMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractOperatorStateRetrieverExtended.Contract.ContractOperatorStateRetrieverExtendedCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.ContractOperatorStateRetrieverExtendedTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.ContractOperatorStateRetrieverExtendedTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractOperatorStateRetrieverExtended.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.contract.Transact(opts, method, params...)
}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCaller) GetCheckSignaturesIndices(opts *bind.CallOpts, registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (OperatorStateRetrieverCheckSignaturesIndices, error) {
	var out []interface{}
	err := _ContractOperatorStateRetrieverExtended.contract.Call(opts, &out, "getCheckSignaturesIndices", registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)

	if err != nil {
		return *new(OperatorStateRetrieverCheckSignaturesIndices), err
	}

	out0 := *abi.ConvertType(out[0], new(OperatorStateRetrieverCheckSignaturesIndices)).(*OperatorStateRetrieverCheckSignaturesIndices)

	return out0, err

}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedSession) GetCheckSignaturesIndices(registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (OperatorStateRetrieverCheckSignaturesIndices, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetCheckSignaturesIndices(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)
}

// GetCheckSignaturesIndices is a free data retrieval call binding the contract method 0x4f739f74.
//
// Solidity: function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes quorumNumbers, bytes32[] nonSignerOperatorIds) view returns((uint32[],uint32[],uint32[],uint32[][]))
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerSession) GetCheckSignaturesIndices(registryCoordinator common.Address, referenceBlockNumber uint32, quorumNumbers []byte, nonSignerOperatorIds [][32]byte) (OperatorStateRetrieverCheckSignaturesIndices, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetCheckSignaturesIndices(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, referenceBlockNumber, quorumNumbers, nonSignerOperatorIds)
}

// GetOperatorIdQuorums is a free data retrieval call binding the contract method 0x06056432.
//
// Solidity: function getOperatorIdQuorums(address registryCoordinator, bytes32 operatorId) view returns(bytes)
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCaller) GetOperatorIdQuorums(opts *bind.CallOpts, registryCoordinator common.Address, operatorId [32]byte) ([]byte, error) {
	var out []interface{}
	err := _ContractOperatorStateRetrieverExtended.contract.Call(opts, &out, "getOperatorIdQuorums", registryCoordinator, operatorId)

	if err != nil {
		return *new([]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([]byte)).(*[]byte)

	return out0, err

}

// GetOperatorIdQuorums is a free data retrieval call binding the contract method 0x06056432.
//
// Solidity: function getOperatorIdQuorums(address registryCoordinator, bytes32 operatorId) view returns(bytes)
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedSession) GetOperatorIdQuorums(registryCoordinator common.Address, operatorId [32]byte) ([]byte, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorIdQuorums(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, operatorId)
}

// GetOperatorIdQuorums is a free data retrieval call binding the contract method 0x06056432.
//
// Solidity: function getOperatorIdQuorums(address registryCoordinator, bytes32 operatorId) view returns(bytes)
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerSession) GetOperatorIdQuorums(registryCoordinator common.Address, operatorId [32]byte) ([]byte, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorIdQuorums(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, operatorId)
}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((address,bytes32,uint96)[][])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCaller) GetOperatorState(opts *bind.CallOpts, registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]OperatorStateRetrieverOperator, error) {
	var out []interface{}
	err := _ContractOperatorStateRetrieverExtended.contract.Call(opts, &out, "getOperatorState", registryCoordinator, quorumNumbers, blockNumber)

	if err != nil {
		return *new([][]OperatorStateRetrieverOperator), err
	}

	out0 := *abi.ConvertType(out[0], new([][]OperatorStateRetrieverOperator)).(*[][]OperatorStateRetrieverOperator)

	return out0, err

}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((address,bytes32,uint96)[][])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedSession) GetOperatorState(registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]OperatorStateRetrieverOperator, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorState(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, quorumNumbers, blockNumber)
}

// GetOperatorState is a free data retrieval call binding the contract method 0x3563b0d1.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes quorumNumbers, uint32 blockNumber) view returns((address,bytes32,uint96)[][])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerSession) GetOperatorState(registryCoordinator common.Address, quorumNumbers []byte, blockNumber uint32) ([][]OperatorStateRetrieverOperator, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorState(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, quorumNumbers, blockNumber)
}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (address,bytes32,uint96)[][])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCaller) GetOperatorState0(opts *bind.CallOpts, registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]OperatorStateRetrieverOperator, error) {
	var out []interface{}
	err := _ContractOperatorStateRetrieverExtended.contract.Call(opts, &out, "getOperatorState0", registryCoordinator, operatorId, blockNumber)

	if err != nil {
		return *new(*big.Int), *new([][]OperatorStateRetrieverOperator), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)
	out1 := *abi.ConvertType(out[1], new([][]OperatorStateRetrieverOperator)).(*[][]OperatorStateRetrieverOperator)

	return out0, out1, err

}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (address,bytes32,uint96)[][])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedSession) GetOperatorState0(registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]OperatorStateRetrieverOperator, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorState0(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, operatorId, blockNumber)
}

// GetOperatorState0 is a free data retrieval call binding the contract method 0xcefdc1d4.
//
// Solidity: function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) view returns(uint256, (address,bytes32,uint96)[][])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerSession) GetOperatorState0(registryCoordinator common.Address, operatorId [32]byte, blockNumber uint32) (*big.Int, [][]OperatorStateRetrieverOperator, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorState0(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, operatorId, blockNumber)
}

// GetOperatorsFromIds is a free data retrieval call binding the contract method 0x15a9bd85.
//
// Solidity: function getOperatorsFromIds(address registryCoordinator, bytes32[] operatorIds) view returns(address[])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCaller) GetOperatorsFromIds(opts *bind.CallOpts, registryCoordinator common.Address, operatorIds [][32]byte) ([]common.Address, error) {
	var out []interface{}
	err := _ContractOperatorStateRetrieverExtended.contract.Call(opts, &out, "getOperatorsFromIds", registryCoordinator, operatorIds)

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetOperatorsFromIds is a free data retrieval call binding the contract method 0x15a9bd85.
//
// Solidity: function getOperatorsFromIds(address registryCoordinator, bytes32[] operatorIds) view returns(address[])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedSession) GetOperatorsFromIds(registryCoordinator common.Address, operatorIds [][32]byte) ([]common.Address, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorsFromIds(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, operatorIds)
}

// GetOperatorsFromIds is a free data retrieval call binding the contract method 0x15a9bd85.
//
// Solidity: function getOperatorsFromIds(address registryCoordinator, bytes32[] operatorIds) view returns(address[])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerSession) GetOperatorsFromIds(registryCoordinator common.Address, operatorIds [][32]byte) ([]common.Address, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorsFromIds(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, operatorIds)
}

// GetOperatorsStakesForQuorum is a free data retrieval call binding the contract method 0x22094e1c.
//
// Solidity: function getOperatorsStakesForQuorum(address registryCoordinator, bytes quorumNumbers, address[] operatorsAddr) view returns((address,bytes32,uint96)[][])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCaller) GetOperatorsStakesForQuorum(opts *bind.CallOpts, registryCoordinator common.Address, quorumNumbers []byte, operatorsAddr []common.Address) ([][]OperatorStateRetrieverOperator, error) {
	var out []interface{}
	err := _ContractOperatorStateRetrieverExtended.contract.Call(opts, &out, "getOperatorsStakesForQuorum", registryCoordinator, quorumNumbers, operatorsAddr)

	if err != nil {
		return *new([][]OperatorStateRetrieverOperator), err
	}

	out0 := *abi.ConvertType(out[0], new([][]OperatorStateRetrieverOperator)).(*[][]OperatorStateRetrieverOperator)

	return out0, err

}

// GetOperatorsStakesForQuorum is a free data retrieval call binding the contract method 0x22094e1c.
//
// Solidity: function getOperatorsStakesForQuorum(address registryCoordinator, bytes quorumNumbers, address[] operatorsAddr) view returns((address,bytes32,uint96)[][])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedSession) GetOperatorsStakesForQuorum(registryCoordinator common.Address, quorumNumbers []byte, operatorsAddr []common.Address) ([][]OperatorStateRetrieverOperator, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorsStakesForQuorum(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, quorumNumbers, operatorsAddr)
}

// GetOperatorsStakesForQuorum is a free data retrieval call binding the contract method 0x22094e1c.
//
// Solidity: function getOperatorsStakesForQuorum(address registryCoordinator, bytes quorumNumbers, address[] operatorsAddr) view returns((address,bytes32,uint96)[][])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerSession) GetOperatorsStakesForQuorum(registryCoordinator common.Address, quorumNumbers []byte, operatorsAddr []common.Address) ([][]OperatorStateRetrieverOperator, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetOperatorsStakesForQuorum(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, quorumNumbers, operatorsAddr)
}

// GetQuorumBitmapsAtBlockNumber is a free data retrieval call binding the contract method 0x5c155662.
//
// Solidity: function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] operatorIds, uint32 blockNumber) view returns(uint256[])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCaller) GetQuorumBitmapsAtBlockNumber(opts *bind.CallOpts, registryCoordinator common.Address, operatorIds [][32]byte, blockNumber uint32) ([]*big.Int, error) {
	var out []interface{}
	err := _ContractOperatorStateRetrieverExtended.contract.Call(opts, &out, "getQuorumBitmapsAtBlockNumber", registryCoordinator, operatorIds, blockNumber)

	if err != nil {
		return *new([]*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new([]*big.Int)).(*[]*big.Int)

	return out0, err

}

// GetQuorumBitmapsAtBlockNumber is a free data retrieval call binding the contract method 0x5c155662.
//
// Solidity: function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] operatorIds, uint32 blockNumber) view returns(uint256[])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedSession) GetQuorumBitmapsAtBlockNumber(registryCoordinator common.Address, operatorIds [][32]byte, blockNumber uint32) ([]*big.Int, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetQuorumBitmapsAtBlockNumber(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, operatorIds, blockNumber)
}

// GetQuorumBitmapsAtBlockNumber is a free data retrieval call binding the contract method 0x5c155662.
//
// Solidity: function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] operatorIds, uint32 blockNumber) view returns(uint256[])
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerSession) GetQuorumBitmapsAtBlockNumber(registryCoordinator common.Address, operatorIds [][32]byte, blockNumber uint32) ([]*big.Int, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.GetQuorumBitmapsAtBlockNumber(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, operatorIds, blockNumber)
}

// OperatorStakeForQuorum is a free data retrieval call binding the contract method 0xc6a867ed.
//
// Solidity: function operatorStakeForQuorum(address registryCoordinator, uint8 quorumNumber, address operator) view returns(uint96, bool)
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCaller) OperatorStakeForQuorum(opts *bind.CallOpts, registryCoordinator common.Address, quorumNumber uint8, operator common.Address) (*big.Int, bool, error) {
	var out []interface{}
	err := _ContractOperatorStateRetrieverExtended.contract.Call(opts, &out, "operatorStakeForQuorum", registryCoordinator, quorumNumber, operator)

	if err != nil {
		return *new(*big.Int), *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)
	out1 := *abi.ConvertType(out[1], new(bool)).(*bool)

	return out0, out1, err

}

// OperatorStakeForQuorum is a free data retrieval call binding the contract method 0xc6a867ed.
//
// Solidity: function operatorStakeForQuorum(address registryCoordinator, uint8 quorumNumber, address operator) view returns(uint96, bool)
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedSession) OperatorStakeForQuorum(registryCoordinator common.Address, quorumNumber uint8, operator common.Address) (*big.Int, bool, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.OperatorStakeForQuorum(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, quorumNumber, operator)
}

// OperatorStakeForQuorum is a free data retrieval call binding the contract method 0xc6a867ed.
//
// Solidity: function operatorStakeForQuorum(address registryCoordinator, uint8 quorumNumber, address operator) view returns(uint96, bool)
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerSession) OperatorStakeForQuorum(registryCoordinator common.Address, quorumNumber uint8, operator common.Address) (*big.Int, bool, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.OperatorStakeForQuorum(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, quorumNumber, operator)
}

// OperatorStakeForQuorumClipped is a free data retrieval call binding the contract method 0xae7b27bb.
//
// Solidity: function operatorStakeForQuorumClipped(address registryCoordinator, uint8 quorumNumber, address operator) view returns(uint96)
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCaller) OperatorStakeForQuorumClipped(opts *bind.CallOpts, registryCoordinator common.Address, quorumNumber uint8, operator common.Address) (*big.Int, error) {
	var out []interface{}
	err := _ContractOperatorStateRetrieverExtended.contract.Call(opts, &out, "operatorStakeForQuorumClipped", registryCoordinator, quorumNumber, operator)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// OperatorStakeForQuorumClipped is a free data retrieval call binding the contract method 0xae7b27bb.
//
// Solidity: function operatorStakeForQuorumClipped(address registryCoordinator, uint8 quorumNumber, address operator) view returns(uint96)
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedSession) OperatorStakeForQuorumClipped(registryCoordinator common.Address, quorumNumber uint8, operator common.Address) (*big.Int, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.OperatorStakeForQuorumClipped(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, quorumNumber, operator)
}

// OperatorStakeForQuorumClipped is a free data retrieval call binding the contract method 0xae7b27bb.
//
// Solidity: function operatorStakeForQuorumClipped(address registryCoordinator, uint8 quorumNumber, address operator) view returns(uint96)
func (_ContractOperatorStateRetrieverExtended *ContractOperatorStateRetrieverExtendedCallerSession) OperatorStakeForQuorumClipped(registryCoordinator common.Address, quorumNumber uint8, operator common.Address) (*big.Int, error) {
	return _ContractOperatorStateRetrieverExtended.Contract.OperatorStakeForQuorumClipped(&_ContractOperatorStateRetrieverExtended.CallOpts, registryCoordinator, quorumNumber, operator)
}

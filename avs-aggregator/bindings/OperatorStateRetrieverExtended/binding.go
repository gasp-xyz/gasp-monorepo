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
	Bin: "0x608060405234801561001057600080fd5b50612654806100206000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c80634f739f74116100665780634f739f74146101145780635c15566214610134578063ae7b27bb14610154578063c6a867ed1461017f578063cefdc1d4146101b157600080fd5b8063060564321461009857806315a9bd85146100c157806322094e1c146100e15780633563b0d114610101575b600080fd5b6100ab6100a6366004611b3b565b6101d2565b6040516100b89190611b67565b60405180910390f35b6100d46100cf366004611c07565b610260565b6040516100b89190611c5b565b6100f46100ef366004611ce9565b61037f565b6040516100b89190611e0f565b6100f461010f366004611e91565b6107ea565b610127610122366004611f48565b610c80565b6040516100b8919061201d565b6101476101423660046120fb565b6113ab565b6040516100b891906121ac565b6101676101623660046121e4565b611573565b6040516001600160601b0390911681526020016100b8565b61019261018d3660046121e4565b611598565b604080516001600160601b0390931683529015156020830152016100b8565b6101c46101bf366004612235565b611707565b6040516100b892919061226c565b60405163871ef04960e01b8152600481018290526060906000906001600160a01b0385169063871ef04990602401602060405180830381865afa15801561021d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610241919061228d565b90506000610257826001600160c01b0316611899565b95945050505050565b60606000826001600160401b0381111561027c5761027c611e29565b6040519080825280602002602001820160405280156102a5578160200160208202803683370190505b50905060005b8381101561037657856001600160a01b031663296bb0648686848181106102d4576102d46122b6565b905060200201356040518263ffffffff1660e01b81526004016102f991815260200190565b602060405180830381865afa158015610316573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061033a91906122cc565b82828151811061034c5761034c6122b6565b6001600160a01b03909216602092830291909101909101528061036e816122ff565b9150506102ab565b50949350505050565b60606000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156103c1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103e591906122cc565b90506000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610427573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061044b91906122cc565b90506000866001600160401b0381111561046757610467611e29565b60405190808252806020026020018201604052801561049a57816020015b60608152602001906001900390816104855790505b50905060005b878110156107dd5760008989838181106104bc576104bc6122b6565b604080516001808252818301909252939091013560f81c935060009291506020820181803683370190505090508a8a848181106104fb576104fb6122b6565b9050013560f81c60f81b81600081518110610518576105186122b6565b60200101906001600160f81b031916908160001a905350876001600160401b0381111561054757610547611e29565b60405190808252806020026020018201604052801561059257816020015b60408051606081018252600080825260208083018290529282015282526000199092019101816105655790505b508484815181106105a5576105a56122b6565b602002602001018190525060005b888110156107c7576000876001600160a01b03166313542a4e8c8c858181106105de576105de6122b6565b90506020020160208101906105f3919061231a565b6040516001600160e01b031960e084901b1681526001600160a01b039091166004820152602401602060405180830381865afa158015610637573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061065b9190612337565b9050600061068b8f868e8e87818110610676576106766122b6565b9050602002016020810190610162919061231a565b905081610696575060005b6107218f6001600160a01b031663871ef049846040518263ffffffff1660e01b81526004016106c791815260200190565b602060405180830381865afa1580156106e4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610708919061228d565b6001600160c01b031661071a86611965565b9081161490565b610729575060005b60405180606001604052808d8d86818110610746576107466122b6565b905060200201602081019061075b919061231a565b6001600160a01b03168152602001838152602001826001600160601b031681525087878151811061078e5761078e6122b6565b602002602001015184815181106107a7576107a76122b6565b6020026020010181905250505080806107bf906122ff565b9150506105b3565b50505080806107d5906122ff565b9150506104a0565b5098975050505050505050565b60606000846001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa15801561082c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061085091906122cc565b90506000856001600160a01b0316639e9923c26040518163ffffffff1660e01b8152600401602060405180830381865afa158015610892573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108b691906122cc565b90506000866001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156108f8573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061091c91906122cc565b9050600086516001600160401b0381111561093957610939611e29565b60405190808252806020026020018201604052801561096c57816020015b60608152602001906001900390816109575790505b50905060005b8751811015610c7457600088828151811061098f5761098f6122b6565b0160200151604051638902624560e01b815260f89190911c6004820181905263ffffffff8a16602483015291506000906001600160a01b03871690638902624590604401600060405180830381865afa1580156109f0573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610a189190810190612350565b905080516001600160401b03811115610a3357610a33611e29565b604051908082528060200260200182016040528015610a7e57816020015b6040805160608101825260008082526020808301829052928201528252600019909201910181610a515790505b50848481518110610a9157610a916122b6565b602002602001018190525060005b8151811015610c5e576040518060600160405280876001600160a01b03166347b314e8858581518110610ad457610ad46122b6565b60200260200101516040518263ffffffff1660e01b8152600401610afa91815260200190565b602060405180830381865afa158015610b17573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b3b91906122cc565b6001600160a01b03168152602001838381518110610b5b57610b5b6122b6565b60200260200101518152602001896001600160a01b031663fa28c627858581518110610b8957610b896122b6565b60209081029190910101516040516001600160e01b031960e084901b168152600481019190915260ff8816602482015263ffffffff8f166044820152606401602060405180830381865afa158015610be5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c0991906123e0565b6001600160601b0316815250858581518110610c2757610c276122b6565b60200260200101518281518110610c4057610c406122b6565b60200260200101819052508080610c56906122ff565b915050610a9f565b5050508080610c6c906122ff565b915050610972565b50979650505050505050565b610cab6040518060800160405280606081526020016060815260200160608152602001606081525090565b6000876001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ceb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d0f91906122cc565b9050610d3c6040518060800160405280606081526020016060815260200160608152602001606081525090565b6040516361c8a12f60e11b81526001600160a01b038a169063c391425e90610d6c908b9089908990600401612409565b600060405180830381865afa158015610d89573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610db19190810190612453565b81526040516340e03a8160e11b81526001600160a01b038316906381c0750290610de3908b908b908b9060040161250a565b600060405180830381865afa158015610e00573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e289190810190612453565b6040820152856001600160401b03811115610e4557610e45611e29565b604051908082528060200260200182016040528015610e7857816020015b6060815260200190600190039081610e635790505b50606082015260005b60ff81168711156112bc576000856001600160401b03811115610ea657610ea6611e29565b604051908082528060200260200182016040528015610ecf578160200160208202803683370190505b5083606001518360ff1681518110610ee957610ee96122b6565b602002602001018190525060005b868110156111bc5760008c6001600160a01b03166304ec63518a8a85818110610f2257610f226122b6565b905060200201358e88600001518681518110610f4057610f406122b6565b60200260200101516040518463ffffffff1660e01b8152600401610f7d9392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa158015610f9a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fbe919061228d565b90506001600160c01b0381166110675760405162461bcd60e51b815260206004820152605c60248201527f4f70657261746f7253746174655265747269657665722e676574436865636b5360448201527f69676e617475726573496e64696365733a206f70657261746f72206d7573742060648201527f6265207265676973746572656420617420626c6f636b6e756d62657200000000608482015260a4015b60405180910390fd5b8a8a8560ff1681811061107c5761107c6122b6565b6001600160c01b03841692013560f81c9190911c6001908116141590506111a957856001600160a01b031663dd9846b98a8a858181106110be576110be6122b6565b905060200201358d8d8860ff168181106110da576110da6122b6565b6040516001600160e01b031960e087901b1681526004810194909452919091013560f81c60248301525063ffffffff8f166044820152606401602060405180830381865afa158015611130573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611154919061252a565b85606001518560ff168151811061116d5761116d6122b6565b60200260200101518481518110611186576111866122b6565b63ffffffff90921660209283029190910190910152826111a5816122ff565b9350505b50806111b4816122ff565b915050610ef7565b506000816001600160401b038111156111d7576111d7611e29565b604051908082528060200260200182016040528015611200578160200160208202803683370190505b50905060005b828110156112815784606001518460ff1681518110611227576112276122b6565b60200260200101518181518110611240576112406122b6565b602002602001015182828151811061125a5761125a6122b6565b63ffffffff9092166020928302919091019091015280611279816122ff565b915050611206565b508084606001518460ff168151811061129c5761129c6122b6565b6020026020010181905250505080806112b490612547565b915050610e81565b506000896001600160a01b0316635df459466040518163ffffffff1660e01b8152600401602060405180830381865afa1580156112fd573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061132191906122cc565b60405163354952a360e21b81529091506001600160a01b0382169063d5254a8c90611354908b908b908e90600401612567565b600060405180830381865afa158015611371573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526113999190810190612453565b60208301525098975050505050505050565b60606000846001600160a01b031663c391425e84866040518363ffffffff1660e01b81526004016113dd929190612591565b600060405180830381865afa1580156113fa573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526114229190810190612453565b9050600084516001600160401b0381111561143f5761143f611e29565b604051908082528060200260200182016040528015611468578160200160208202803683370190505b50905060005b855181101561156957866001600160a01b03166304ec6351878381518110611498576114986122b6565b6020026020010151878685815181106114b3576114b36122b6565b60200260200101516040518463ffffffff1660e01b81526004016114f09392919092835263ffffffff918216602084015216604082015260600190565b602060405180830381865afa15801561150d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611531919061228d565b6001600160c01b031682828151811061154c5761154c6122b6565b602090810291909101015280611561816122ff565b91505061146e565b5095945050505050565b6000806000611583868686611598565b91509150806103765750600095945050505050565b6000806000856001600160a01b031663683048356040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115db573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115ff91906122cc565b60405162fcdba760e51b815260ff871660048201526001600160a01b038681166024830152919250600091831690631f9b74e090604401602060405180830381865afa158015611653573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061167791906123e0565b60405163c46778a560e01b815260ff881660048201529091506000906001600160a01b0384169063c46778a590602401602060405180830381865afa1580156116c4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116e891906123e0565b919450506001600160601b039081169084161015915050935093915050565b6040805160018082528183019092526000916060918391602080830190803683370190505090508481600081518110611742576117426122b6565b60209081029190910101526040516361c8a12f60e11b81526000906001600160a01b0388169063c391425e9061177e9088908690600401612591565b600060405180830381865afa15801561179b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117c39190810190612453565b6000815181106117d5576117d56122b6565b60209081029190910101516040516304ec635160e01b81526004810188905263ffffffff87811660248301529091166044820181905291506000906001600160a01b038916906304ec635190606401602060405180830381865afa158015611841573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611865919061228d565b6001600160c01b03169050600061187b82611899565b9050816118898a838a6107ea565b9550955050505050935093915050565b60606000806118a784611af2565b61ffff166001600160401b038111156118c2576118c2611e29565b6040519080825280601f01601f1916602001820160405280156118ec576020820181803683370190505b5090506000805b825182108015611904575061010081105b1561195b576001811b93508584161561194b578060f81b83838151811061192d5761192d6122b6565b60200101906001600160f81b031916908160001a9053508160010191505b611954816122ff565b90506118f3565b5090949350505050565b6000610100825111156119ee5760405162461bcd60e51b8152602060048201526044602482018190527f4269746d61705574696c732e6f72646572656442797465734172726179546f42908201527f69746d61703a206f7264657265644279746573417272617920697320746f6f206064820152636c6f6e6760e01b608482015260a40161105e565b81516119fc57506000919050565b60008083600081518110611a1257611a126122b6565b0160200151600160f89190911c81901b92505b8451811015611ae957848181518110611a4057611a406122b6565b0160200151600160f89190911c1b9150828211611ad55760405162461bcd60e51b815260206004820152604760248201527f4269746d61705574696c732e6f72646572656442797465734172726179546f4260448201527f69746d61703a206f72646572656442797465734172726179206973206e6f74206064820152661bdc99195c995960ca1b608482015260a40161105e565b91811791611ae2816122ff565b9050611a25565b50909392505050565b6000805b8215611b1d57611b076001846125e5565b9092169180611b15816125fc565b915050611af6565b92915050565b6001600160a01b0381168114611b3857600080fd5b50565b60008060408385031215611b4e57600080fd5b8235611b5981611b23565b946020939093013593505050565b600060208083528351808285015260005b81811015611b9457858101830151858201604001528201611b78565b81811115611ba6576000604083870101525b50601f01601f1916929092016040019392505050565b60008083601f840112611bce57600080fd5b5081356001600160401b03811115611be557600080fd5b6020830191508360208260051b8501011115611c0057600080fd5b9250929050565b600080600060408486031215611c1c57600080fd5b8335611c2781611b23565b925060208401356001600160401b03811115611c4257600080fd5b611c4e86828701611bbc565b9497909650939450505050565b6020808252825182820181905260009190848201906040850190845b81811015611c9c5783516001600160a01b031683529284019291840191600101611c77565b50909695505050505050565b60008083601f840112611cba57600080fd5b5081356001600160401b03811115611cd157600080fd5b602083019150836020828501011115611c0057600080fd5b600080600080600060608688031215611d0157600080fd5b8535611d0c81611b23565b945060208601356001600160401b0380821115611d2857600080fd5b611d3489838a01611ca8565b90965094506040880135915080821115611d4d57600080fd5b50611d5a88828901611bbc565b969995985093965092949392505050565b600081518084526020808501808196508360051b810191508286016000805b86811015611e01578385038a52825180518087529087019087870190845b81811015611dec57835180516001600160a01b031684528a8101518b8501526040908101516001600160601b03169084015292890192606090920191600101611da8565b50509a87019a95505091850191600101611d8a565b509298975050505050505050565b602081526000611e226020830184611d6b565b9392505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b0381118282101715611e6757611e67611e29565b604052919050565b63ffffffff81168114611b3857600080fd5b8035611e8c81611e6f565b919050565b600080600060608486031215611ea657600080fd5b8335611eb181611b23565b92506020848101356001600160401b0380821115611ece57600080fd5b818701915087601f830112611ee257600080fd5b813581811115611ef457611ef4611e29565b611f06601f8201601f19168501611e3f565b91508082528884828501011115611f1c57600080fd5b8084840185840137600084828401015250809450505050611f3f60408501611e81565b90509250925092565b60008060008060008060808789031215611f6157600080fd5b8635611f6c81611b23565b95506020870135611f7c81611e6f565b945060408701356001600160401b0380821115611f9857600080fd5b611fa48a838b01611ca8565b90965094506060890135915080821115611fbd57600080fd5b50611fca89828a01611bbc565b979a9699509497509295939492505050565b600081518084526020808501945080840160005b8381101561201257815163ffffffff1687529582019590820190600101611ff0565b509495945050505050565b60006020808352835160808285015261203960a0850182611fdc565b905081850151601f19808684030160408701526120568383611fdc565b925060408701519150808684030160608701526120738383611fdc565b60608801518782038301608089015280518083529194508501925084840190600581901b8501860160005b828110156120ca57848783030184526120b8828751611fdc565b9588019593880193915060010161209e565b509998505050505050505050565b60006001600160401b038211156120f1576120f1611e29565b5060051b60200190565b60008060006060848603121561211057600080fd5b833561211b81611b23565b92506020848101356001600160401b0381111561213757600080fd5b8501601f8101871361214857600080fd5b803561215b612156826120d8565b611e3f565b81815260059190911b8201830190838101908983111561217a57600080fd5b928401925b828410156121985783358252928401929084019061217f565b8096505050505050611f3f60408501611e81565b6020808252825182820181905260009190848201906040850190845b81811015611c9c578351835292840192918401916001016121c8565b6000806000606084860312156121f957600080fd5b833561220481611b23565b9250602084013560ff8116811461221a57600080fd5b9150604084013561222a81611b23565b809150509250925092565b60008060006060848603121561224a57600080fd5b833561225581611b23565b925060208401359150604084013561222a81611e6f565b8281526040602082015260006122856040830184611d6b565b949350505050565b60006020828403121561229f57600080fd5b81516001600160c01b0381168114611e2257600080fd5b634e487b7160e01b600052603260045260246000fd5b6000602082840312156122de57600080fd5b8151611e2281611b23565b634e487b7160e01b600052601160045260246000fd5b6000600019821415612313576123136122e9565b5060010190565b60006020828403121561232c57600080fd5b8135611e2281611b23565b60006020828403121561234957600080fd5b5051919050565b6000602080838503121561236357600080fd5b82516001600160401b0381111561237957600080fd5b8301601f8101851361238a57600080fd5b8051612398612156826120d8565b81815260059190911b820183019083810190878311156123b757600080fd5b928401925b828410156123d5578351825292840192908401906123bc565b979650505050505050565b6000602082840312156123f257600080fd5b81516001600160601b0381168114611e2257600080fd5b63ffffffff84168152604060208201819052810182905260006001600160fb1b0383111561243657600080fd5b8260051b8085606085013760009201606001918252509392505050565b6000602080838503121561246657600080fd5b82516001600160401b0381111561247c57600080fd5b8301601f8101851361248d57600080fd5b805161249b612156826120d8565b81815260059190911b820183019083810190878311156124ba57600080fd5b928401925b828410156123d55783516124d281611e6f565b825292840192908401906124bf565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b63ffffffff841681526040602082015260006102576040830184866124e1565b60006020828403121561253c57600080fd5b8151611e2281611e6f565b600060ff821660ff81141561255e5761255e6122e9565b60010192915050565b60408152600061257b6040830185876124e1565b905063ffffffff83166020830152949350505050565b60006040820163ffffffff851683526020604081850152818551808452606086019150828701935060005b818110156125d8578451835293830193918301916001016125bc565b5090979650505050505050565b6000828210156125f7576125f76122e9565b500390565b600061ffff80831681811415612614576126146122e9565b600101939250505056fea2646970667358221220237229fbbe7beac223fb6db1be110bdfd0fc53d48bacdc3e1b777d9b38f7f9a464736f6c634300080c0033",
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

export const mangataContractAbi = [
	{ inputs: [], stateMutability: "nonpayable", type: "constructor" },
	{
		inputs: [
			{ internalType: "uint256", name: "value", type: "uint256" },
			{ internalType: "uint256", name: "length", type: "uint256" },
		],
		name: "StringsInsufficientHexLength",
		type: "error",
	},
	{
		anonymous: false,
		inputs: [
			{
				indexed: false,
				internalType: "address",
				name: "depositRecipient",
				type: "address",
			},
			{
				indexed: false,
				internalType: "address",
				name: "tokenAddress",
				type: "address",
			},
			{
				indexed: false,
				internalType: "uint256",
				name: "amount",
				type: "uint256",
			},
		],
		name: "DepositAcceptedIntoQueue",
		type: "event",
	},
	{
		anonymous: false,
		inputs: [
			{
				indexed: false,
				internalType: "uint256",
				name: "requestId",
				type: "uint256",
			},
			{
				indexed: false,
				internalType: "uint256",
				name: "originalRequestId",
				type: "uint256",
			},
			{
				indexed: false,
				internalType: "bool",
				name: "cancelJustified",
				type: "bool",
			},
		],
		name: "DisputeResolutionAcceptedIntoQueue",
		type: "event",
	},
	{
		anonymous: false,
		inputs: [
			{
				indexed: false,
				internalType: "address",
				name: "withdrawRecipient",
				type: "address",
			},
			{
				indexed: false,
				internalType: "address",
				name: "tokenAddress",
				type: "address",
			},
			{
				indexed: false,
				internalType: "uint256",
				name: "amount",
				type: "uint256",
			},
		],
		name: "FundsWithdrawn",
		type: "event",
	},
	{
		anonymous: false,
		inputs: [
			{
				indexed: false,
				internalType: "uint256[]",
				name: "l2UpdatesToRemove",
				type: "uint256[]",
			},
		],
		name: "L2UpdatesToRemovedAcceptedIntoQueue",
		type: "event",
	},
	{
		anonymous: false,
		inputs: [
			{
				indexed: false,
				internalType: "address",
				name: "withdrawRecipient",
				type: "address",
			},
			{
				indexed: false,
				internalType: "address",
				name: "tokenAddress",
				type: "address",
			},
			{
				indexed: false,
				internalType: "uint256",
				name: "amount",
				type: "uint256",
			},
		],
		name: "WithdrawAcceptedIntoQueue",
		type: "event",
	},
	{
		anonymous: false,
		inputs: [
			{
				indexed: false,
				internalType: "bytes32",
				name: "cancelHash",
				type: "bytes32",
			},
			{
				indexed: false,
				internalType: "bytes32",
				name: "calculatedHash",
				type: "bytes32",
			},
		],
		name: "cancelAndCalculatedHash",
		type: "event",
	},
	{
		inputs: [{ internalType: "uint256", name: "", type: "uint256" }],
		name: "cancelResolutions",
		outputs: [
			{ internalType: "uint256", name: "l2RequestId", type: "uint256" },
			{ internalType: "bool", name: "cancelJustified", type: "bool" },
		],
		stateMutability: "view",
		type: "function",
	},
	{
		inputs: [],
		name: "counter",
		outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
		stateMutability: "view",
		type: "function",
	},
	{
		inputs: [
			{ internalType: "address", name: "tokenAddress", type: "address" },
			{ internalType: "uint256", name: "amount", type: "uint256" },
		],
		name: "deposit",
		outputs: [],
		stateMutability: "nonpayable",
		type: "function",
	},
	{
		inputs: [],
		name: "getUpdateForL2",
		outputs: [{ internalType: "string", name: "", type: "string" }],
		stateMutability: "view",
		type: "function",
	},
	{
		inputs: [],
		name: "lastProcessedUpdate_origin_l1",
		outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
		stateMutability: "view",
		type: "function",
	},
	{
		inputs: [],
		name: "lastProcessedUpdate_origin_l2",
		outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
		stateMutability: "view",
		type: "function",
	},
	{
		inputs: [
			{ internalType: "address", name: "tokenAddress", type: "address" },
			{ internalType: "address", name: "accountAddress", type: "address" },
		],
		name: "test_getTokenBalance",
		outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
		stateMutability: "view",
		type: "function",
	},
	{
		inputs: [],
		name: "test_hashOfPending",
		outputs: [{ internalType: "bytes32", name: "", type: "bytes32" }],
		stateMutability: "view",
		type: "function",
	},
	{
		inputs: [
			{ internalType: "uint256", name: "start", type: "uint256" },
			{ internalType: "uint256", name: "end", type: "uint256" },
		],
		name: "test_hashOfPending_range",
		outputs: [{ internalType: "bytes32", name: "", type: "bytes32" }],
		stateMutability: "view",
		type: "function",
	},
	{
		inputs: [
			{ internalType: "uint256[]", name: "inputArray", type: "uint256[]" },
		],
		name: "update_l1_from_l2",
		outputs: [],
		stateMutability: "nonpayable",
		type: "function",
	},
	{
		inputs: [
			{ internalType: "address", name: "tokenAddress", type: "address" },
			{ internalType: "uint256", name: "amount", type: "uint256" },
		],
		name: "withdraw",
		outputs: [],
		stateMutability: "nonpayable",
		type: "function",
	},
] as const;

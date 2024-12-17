use bindings::finalizer_task_manager::FinalizerTaskManagerCalls;
use ethers::{
    abi::{Abi, AbiDecode, Token},
    types::{Address, Bytes},
};
use eyre::eyre;
use serde_json::from_str;

#[derive(Debug)]
pub struct CallDecoder {
    taskman_addr: Address,
    abi: Abi,
}

impl CallDecoder {
    pub fn new(task_manager_addr: Address) -> CallDecoder {
        let abi: Abi = from_str(ABI).expect("ABI copypaste is valid");
        CallDecoder {
            taskman_addr: task_manager_addr,
            abi,
        }
    }

    pub fn parse_call_data(&self, call_data: Bytes) -> eyre::Result<FinalizerTaskManagerCalls> {
        if let Ok(call) = FinalizerTaskManagerCalls::decode(call_data.clone()) {
            return Ok(call);
        }

        let fun = self.abi.function("execTransaction").unwrap();
        if let Ok(call) = fun.decode_input(&call_data) {
            match call.as_slice() {
                // might have a batch here also, make sure we don't use it at safe.wallet for now
                // Function: execTransaction(address to,uint256 value,bytes data,uint8 operation,uint256 safeTxGas,uint256 baseGas,uint256 gasPrice,address gasToken,address refundReceiver,bytes signatures)
                [Token::Address(addr), _, Token::Bytes(bytes), _, _, _, _, _, _, _] => {
                    if let Ok(call) = FinalizerTaskManagerCalls::decode(bytes.clone()) {
                        if *addr == self.taskman_addr {
                            return Ok(call);
                        }
                    }
                }
                _ => (),
            }
        }

        Err(eyre!("cannot decode trx call data {:x}", call_data))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_safe_execute() {
        let call_data = "0x000000000000000000000000150fe8dbb943c372f3e8c31d9c89f1e6a13cbbfd000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000380000000000000000000000000000000000000000000000000000000000000020409ec190600000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000f9a220e545b062671cd65ac82712b6c349d132ed05cb7f79bba13a18fb4b5d2f6fa759ff9ca954a9934099df33a3d04e1daa2fa5cbb6e503e3a1e37ec1200d3a0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000146cac90000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000146cac90000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000004200000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000042000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008203b96af9e7822fa84a41943a3ddad9c64f7491fbaf5d105324e1a142e5c658fb0f56b6eeaf0ba50f95dbd8d2687cf6efbbcb57d227c50baf72cab9b1167766f31cc9b1d8f3e748ef9cdf19b40c3ef067527ed72f36b0ddf6558a9771cac7bc0a7c39053c91880185276038933df2bb877209823f7a272ab5c14c54181f9dc433de1c000000000000000000000000000000000000000000000000000000000000";
        let bytes = Bytes::from_str(&call_data).unwrap();
        let decoder = CallDecoder::new(
            Address::from_str("0x150FE8DBB943c372F3e8C31d9c89f1E6A13cBBFd").unwrap(),
        );
        assert!(decoder.parse_call_data(bytes).is_ok());
    }
}

const ABI: &str = r#"[
  { "inputs": [], "stateMutability": "nonpayable", "type": "constructor" },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "owner",
        "type": "address"
      }
    ],
    "name": "AddedOwner",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "approvedHash",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "owner",
        "type": "address"
      }
    ],
    "name": "ApproveHash",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "handler",
        "type": "address"
      }
    ],
    "name": "ChangedFallbackHandler",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "guard",
        "type": "address"
      }
    ],
    "name": "ChangedGuard",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "threshold",
        "type": "uint256"
      }
    ],
    "name": "ChangedThreshold",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "module",
        "type": "address"
      }
    ],
    "name": "DisabledModule",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "module",
        "type": "address"
      }
    ],
    "name": "EnabledModule",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "txHash",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "payment",
        "type": "uint256"
      }
    ],
    "name": "ExecutionFailure",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "module",
        "type": "address"
      }
    ],
    "name": "ExecutionFromModuleFailure",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "module",
        "type": "address"
      }
    ],
    "name": "ExecutionFromModuleSuccess",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "txHash",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "payment",
        "type": "uint256"
      }
    ],
    "name": "ExecutionSuccess",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "owner",
        "type": "address"
      }
    ],
    "name": "RemovedOwner",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "sender",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "value",
        "type": "uint256"
      }
    ],
    "name": "SafeReceived",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "initiator",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "address[]",
        "name": "owners",
        "type": "address[]"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "threshold",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "initializer",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "fallbackHandler",
        "type": "address"
      }
    ],
    "name": "SafeSetup",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "msgHash",
        "type": "bytes32"
      }
    ],
    "name": "SignMsg",
    "type": "event"
  },
  { "stateMutability": "nonpayable", "type": "fallback" },
  {
    "inputs": [],
    "name": "VERSION",
    "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "owner", "type": "address" },
      { "internalType": "uint256", "name": "_threshold", "type": "uint256" }
    ],
    "name": "addOwnerWithThreshold",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "hashToApprove", "type": "bytes32" }
    ],
    "name": "approveHash",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "", "type": "address" },
      { "internalType": "bytes32", "name": "", "type": "bytes32" }
    ],
    "name": "approvedHashes",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "_threshold", "type": "uint256" }
    ],
    "name": "changeThreshold",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "dataHash", "type": "bytes32" },
      { "internalType": "bytes", "name": "data", "type": "bytes" },
      { "internalType": "bytes", "name": "signatures", "type": "bytes" },
      {
        "internalType": "uint256",
        "name": "requiredSignatures",
        "type": "uint256"
      }
    ],
    "name": "checkNSignatures",
    "outputs": [],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "dataHash", "type": "bytes32" },
      { "internalType": "bytes", "name": "data", "type": "bytes" },
      { "internalType": "bytes", "name": "signatures", "type": "bytes" }
    ],
    "name": "checkSignatures",
    "outputs": [],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "prevModule", "type": "address" },
      { "internalType": "address", "name": "module", "type": "address" }
    ],
    "name": "disableModule",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "domainSeparator",
    "outputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "module", "type": "address" }
    ],
    "name": "enableModule",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "to", "type": "address" },
      { "internalType": "uint256", "name": "value", "type": "uint256" },
      { "internalType": "bytes", "name": "data", "type": "bytes" },
      {
        "internalType": "enum Enum.Operation",
        "name": "operation",
        "type": "uint8"
      },
      { "internalType": "uint256", "name": "safeTxGas", "type": "uint256" },
      { "internalType": "uint256", "name": "baseGas", "type": "uint256" },
      { "internalType": "uint256", "name": "gasPrice", "type": "uint256" },
      { "internalType": "address", "name": "gasToken", "type": "address" },
      {
        "internalType": "address",
        "name": "refundReceiver",
        "type": "address"
      },
      { "internalType": "uint256", "name": "_nonce", "type": "uint256" }
    ],
    "name": "encodeTransactionData",
    "outputs": [{ "internalType": "bytes", "name": "", "type": "bytes" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "to", "type": "address" },
      { "internalType": "uint256", "name": "value", "type": "uint256" },
      { "internalType": "bytes", "name": "data", "type": "bytes" },
      {
        "internalType": "enum Enum.Operation",
        "name": "operation",
        "type": "uint8"
      },
      { "internalType": "uint256", "name": "safeTxGas", "type": "uint256" },
      { "internalType": "uint256", "name": "baseGas", "type": "uint256" },
      { "internalType": "uint256", "name": "gasPrice", "type": "uint256" },
      { "internalType": "address", "name": "gasToken", "type": "address" },
      {
        "internalType": "address payable",
        "name": "refundReceiver",
        "type": "address"
      },
      { "internalType": "bytes", "name": "signatures", "type": "bytes" }
    ],
    "name": "execTransaction",
    "outputs": [{ "internalType": "bool", "name": "success", "type": "bool" }],
    "stateMutability": "payable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "to", "type": "address" },
      { "internalType": "uint256", "name": "value", "type": "uint256" },
      { "internalType": "bytes", "name": "data", "type": "bytes" },
      {
        "internalType": "enum Enum.Operation",
        "name": "operation",
        "type": "uint8"
      }
    ],
    "name": "execTransactionFromModule",
    "outputs": [{ "internalType": "bool", "name": "success", "type": "bool" }],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "to", "type": "address" },
      { "internalType": "uint256", "name": "value", "type": "uint256" },
      { "internalType": "bytes", "name": "data", "type": "bytes" },
      {
        "internalType": "enum Enum.Operation",
        "name": "operation",
        "type": "uint8"
      }
    ],
    "name": "execTransactionFromModuleReturnData",
    "outputs": [
      { "internalType": "bool", "name": "success", "type": "bool" },
      { "internalType": "bytes", "name": "returnData", "type": "bytes" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "getChainId",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "start", "type": "address" },
      { "internalType": "uint256", "name": "pageSize", "type": "uint256" }
    ],
    "name": "getModulesPaginated",
    "outputs": [
      { "internalType": "address[]", "name": "array", "type": "address[]" },
      { "internalType": "address", "name": "next", "type": "address" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "getOwners",
    "outputs": [
      { "internalType": "address[]", "name": "", "type": "address[]" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "offset", "type": "uint256" },
      { "internalType": "uint256", "name": "length", "type": "uint256" }
    ],
    "name": "getStorageAt",
    "outputs": [{ "internalType": "bytes", "name": "", "type": "bytes" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "getThreshold",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "to", "type": "address" },
      { "internalType": "uint256", "name": "value", "type": "uint256" },
      { "internalType": "bytes", "name": "data", "type": "bytes" },
      {
        "internalType": "enum Enum.Operation",
        "name": "operation",
        "type": "uint8"
      },
      { "internalType": "uint256", "name": "safeTxGas", "type": "uint256" },
      { "internalType": "uint256", "name": "baseGas", "type": "uint256" },
      { "internalType": "uint256", "name": "gasPrice", "type": "uint256" },
      { "internalType": "address", "name": "gasToken", "type": "address" },
      {
        "internalType": "address",
        "name": "refundReceiver",
        "type": "address"
      },
      { "internalType": "uint256", "name": "_nonce", "type": "uint256" }
    ],
    "name": "getTransactionHash",
    "outputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "module", "type": "address" }
    ],
    "name": "isModuleEnabled",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "owner", "type": "address" }
    ],
    "name": "isOwner",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "nonce",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "prevOwner", "type": "address" },
      { "internalType": "address", "name": "owner", "type": "address" },
      { "internalType": "uint256", "name": "_threshold", "type": "uint256" }
    ],
    "name": "removeOwner",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "handler", "type": "address" }
    ],
    "name": "setFallbackHandler",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "guard", "type": "address" }
    ],
    "name": "setGuard",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address[]", "name": "_owners", "type": "address[]" },
      { "internalType": "uint256", "name": "_threshold", "type": "uint256" },
      { "internalType": "address", "name": "to", "type": "address" },
      { "internalType": "bytes", "name": "data", "type": "bytes" },
      {
        "internalType": "address",
        "name": "fallbackHandler",
        "type": "address"
      },
      { "internalType": "address", "name": "paymentToken", "type": "address" },
      { "internalType": "uint256", "name": "payment", "type": "uint256" },
      {
        "internalType": "address payable",
        "name": "paymentReceiver",
        "type": "address"
      }
    ],
    "name": "setup",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
    "name": "signedMessages",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "targetContract",
        "type": "address"
      },
      { "internalType": "bytes", "name": "calldataPayload", "type": "bytes" }
    ],
    "name": "simulateAndRevert",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "prevOwner", "type": "address" },
      { "internalType": "address", "name": "oldOwner", "type": "address" },
      { "internalType": "address", "name": "newOwner", "type": "address" }
    ],
    "name": "swapOwner",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  { "stateMutability": "payable", "type": "receive" }
]"#;

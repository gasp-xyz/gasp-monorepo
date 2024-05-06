import eigenContractAbi from "../FinalizerTaskManager.json" assert {type: "json"};
import rolldownAbi from "../RollDown.json" assert { type: "json" };
import {configuration} from "../config/config.js"



export const ROLLDOWN_ABI = rolldownAbi.abi;
export const EIGEN_ABI = eigenContractAbi.abi;
export const MANGATA_NODE_URL = configuration.MANGATA_NODE_URL
export const ETH_CHAIN_URL = configuration.ETH_CHAIN_URL
export const WALLET_PRIVATE_KEY = configuration.WALLET_PRIVATE_KEY
export const EIGEN_CONTRACT_ADDRESS = configuration.EIGEN_CONTRACT_ADDRESS! as `0x${string}`;
export const MANGATA_CONTRACT_ADDRESS = configuration.MANGATA_CONTRACT_ADDRESS as `0x${string}`;
export const FINALIZATION_SOURCE = configuration.FINALIZATION_SOURCE;
export const VERBOSE = Boolean(configuration.VERBOSE);
export const LIMIT = parseInt(configuration.LIMIT)
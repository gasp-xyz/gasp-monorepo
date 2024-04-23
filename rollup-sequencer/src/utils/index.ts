import {configuration} from "../config";
import {Keyring} from "@polkadot/api";
import {hexToU8a} from "@polkadot/util";
import {KeypairType} from "@polkadot/util-crypto/types";
import rolldownAbi from "../RollDown.json" assert { type: "json" };


/**
 * CONSTANTS
 */
const MANGATA_CONTRACT_ADDRESS = configuration.MANGATA_CONTRACT_ADDRESS as `0x${string}`

const BLOCK_NUMBER_DELAY = configuration.BLOCK_NUMBER_DELAY;

const ETH_CHAIN_URL = configuration.ETH_CHAIN_URL

const MNEMONIC = configuration.MNEMONIC

const ABI = rolldownAbi.abi;

/**
 * CUSTOM METHODS
 */
function sleep(timeInMilliseconds: number) {
    return new Promise((resolve) => setTimeout(resolve, timeInMilliseconds));
}

function getKeyring(type: KeypairType) {
    return new Keyring({ type });
}

function getCollatorAddress(type: KeypairType, mnemonic: string) {
    const keyring = getKeyring(type)
    return keyring.addFromSeed(hexToU8a(mnemonic));
}

export {
    MANGATA_CONTRACT_ADDRESS,
    BLOCK_NUMBER_DELAY,
    ETH_CHAIN_URL,
    ABI,
    sleep,
    getCollatorAddress
}
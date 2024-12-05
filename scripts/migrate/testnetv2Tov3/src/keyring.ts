
import type { KeyringPair } from "@polkadot/keyring/types";
import type { KeypairType } from "@polkadot/util-crypto/types";
import {Keyring } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";

function getKeyring(type: KeypairType): Keyring {
	return new Keyring({ type });
}

export function getKeyPair(type: KeypairType, mnemonic: string): KeyringPair {
	const keyring = getKeyring(type);
	return keyring.addFromSeed(hexToU8a(mnemonic));
}

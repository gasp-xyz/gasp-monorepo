import { getSourceApi, getTargetApi } from "./api.js";
import { xxhashAsHex } from "@polkadot/util-crypto";
import { Codec, ITuple } from "@polkadot/types/types";
import { SubmittableExtrinsic } from "@polkadot/api/types";
import { Bytes, StorageKey } from "@polkadot/types";
import { Call } from "@polkadot/types/interfaces";
import {getKeyPair} from "./keyring.js";
import { TARGET_SUDO_MNEMONIC } from "./Config.js";
import { signTx } from "gasp-sdk-target";
import type { BlockHash } from '@polkadot/types/interfaces/chain';
import type { StorageChangeSet, KeyValueOption } from "@polkadot/types/interfaces/state";


export type Extrinsic = SubmittableExtrinsic<"promise">;

// TODO
// Not doing this right now because only one storag item has changed and it is just a StorageVal
// Hopefully there is a way to get the concrete type passed into SourceType so that we can avoid sourceType string (same for TargetType)
// export async function migrateWithTransform<SourceType, TargetType>(source_data: string, sourceType: string, targetType: string, transform: (arg: SourceType) => TargetType ) {
    
//     const sourceApi = await getSourceApi();
//     const targetApi = await getTargetApi();
//     let sourceVal = sourceApi.createType(sourceType, source_data);
//     let targetVal = transform(sourceVal);

// }

export async function migrateWithoutTransform( sourceBlockHashAt: BlockHash) {
    const sourceApi = await getSourceApi();
    const targetApi = await getTargetApi();

    console.log("::: Starting migration :::");

    const sudo_signer = getKeyPair("ethereum", TARGET_SUDO_MNEMONIC);

    // toMigrateWithoutTransform
    const storageToMigrate = [
        // ["Xyk", "Pools"],
        // ["Xyk", "LiquidityAssets"],
        // ["Xyk", "LiquidityPools"],
        // ["AssetRegistry", "Metadata"],
        // ["AssetRegistry", "IdToL1Asset"],
        // ["AssetRegistry", "L1AssetToId"],
        // ["Tokens", "TotalIssuance"],
        // ["Tokens", "NextCurrencyId"],
        // ["Tokens", "Locks"],
        ["Tokens", "Accounts"],
        // ["Tokens", "Reserves"],
    ]

    for (let dataId = 0; dataId < storageToMigrate.length; dataId++) {
      const key = getStorageKey(
        storageToMigrate[dataId][0],
        storageToMigrate[dataId][1],
      );
      const pageItemCount = 400;
      console.warn(
        "::: starting with :::" + JSON.stringify(storageToMigrate[dataId]),
      );
      let allKeys = [];
      let cont = true;
      let keys = await sourceApi.rpc.state.getKeysPaged(key, pageItemCount, undefined, sourceBlockHashAt);
      let loop: number = 0;
      while (cont) {
        const storage  = await sourceApi.rpc.state.queryStorage(keys, sourceBlockHashAt, sourceBlockHashAt);
        for (let index = 0; index < keys.length; index++) {
          allKeys.push([keys[index], (storage[0][1] as KeyValueOption[])[index].toHex()]);
        }
        
        console.info("Found:" + JSON.stringify(allKeys.length));
        const nextkeys = await sourceApi.rpc.state.getKeysPaged(
          key,
          pageItemCount,
          keys[keys.length - 1],
          sourceBlockHashAt
        );
        if (loop % 8 === 0) {
          const txs: Extrinsic[] = [];
          allKeys.forEach((x) => {
            const storageKey = targetApi.createType("StorageKey", x[0]);
            const storageData = targetApi.createType("StorageData", x[1]);
            const tx = targetApi.tx.system.setStorage([
              [storageKey, storageData] as ITuple<[StorageKey, Bytes]>,
            ]);
            txs.push(targetApi.tx.sudo.sudo(tx as any as Call));
          });
          await signTx(
            targetApi,
            targetApi.tx.utility.batchAll(txs as any as Call[]),
            sudo_signer,
          );
          allKeys = [];
          console.log("lastKeyMigrated", keys[keys.length - 1]);
        }
        if (nextkeys.includes(keys[keys.length - 1]) || nextkeys.length === 0) {
          cont = false;
        } else {
          keys = nextkeys;
        }
        loop++;
      }
      const txs: Extrinsic[] = [];
      allKeys.forEach((x) => {
        const storageKey = targetApi.createType("StorageKey", x[0]);
        const storageData = targetApi.createType("StorageData", x[1]);
        const tx = targetApi.tx.system.setStorage([
          [storageKey, storageData] as ITuple<[StorageKey, Bytes]>,
        ]);
        txs.push(targetApi.tx.sudo.sudo(tx as any as Call));
      });
  
      await signTx(
        targetApi,
        targetApi.tx.utility.batchAll(txs as any as Call[]),
        sudo_signer,
      );
    }
  }

  export function getStorageKey(
    moduleName: string,
    storageItemName: string,
  ): string {
    return (
      xxhashAsHex(moduleName, 128) +
      stripHexPrefix(xxhashAsHex(storageItemName, 128))
    );
  }
  export function stripHexPrefix(str: string): string {
    return isHexPrefixed(str) ? str.slice(2) : str;
  }
  function isHexPrefixed(str: string): boolean {
    return str.slice(0, 2) === "0x";
  }
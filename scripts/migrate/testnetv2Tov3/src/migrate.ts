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
import type { KeyValueOption } from "@polkadot/types/interfaces/state";


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
        // ["Tokens", "Reserves"],
        ["Tokens", "Accounts"],
    ]

    for (let dataId = 0; dataId < storageToMigrate.length; dataId++) {
      const key = getStorageKey(
        storageToMigrate[dataId][0],
        storageToMigrate[dataId][1],
      );
      const pageItemCount = 500;
      console.warn(
        "::: starting with :::" + JSON.stringify(storageToMigrate[dataId]),
      );
      let allKeys = [];
      let cont = true;
      let startKey = undefined;

      // Uncomment and edit this to start from a specific key and blockHash
      let sourceBlockHashAt: BlockHash = sourceApi.createType('BlockHash', '0xb1b909f7eb6c4dd59ecf08643940eeaae9c6d9db05dea9635e9154d5fa9efd4a');
      // 176500 done at 0x99971b5749ac43e0235e41b0d37869188ee7418a6531173d60d1f6a82d8f4d51172ee63239e33e39502de6dfa6f5a5ae7db676f79bbe73c870dad45b053a5b2b238d5dedd9c986912843223804000000
      // Another 652500 done at 0x99971b5749ac43e0235e41b0d37869188ee7418a6531173d60d1f6a82d8f4d516bff0d30eb5a349f2666469fea4a1d515b6ed2fb85fc681786d2c636749ef2979bb2dd21b4def25cfda6ef3a00000000
      // Another 736500 done at 0x99971b5749ac43e0235e41b0d37869188ee7418a6531173d60d1f6a82d8f4d51cb76f13fd77bab283f9d65271bc72ab94308a9f4cbd9420fb1a0df332f911a8b279c6c1e5153cb1f00942ff401000000
      // Another 404500 done at 0x99971b5749ac43e0235e41b0d37869188ee7418a6531173d60d1f6a82d8f4d51fff886bd2b46f082ecbcbaa9631f139c771683cd5b0a017ac1220a931e16f1a551c23c5cb4def25cfda6ef3a00000000
      // Another 220 at tail (missing logs)
      // let startKey = "0x99971b5749ac43e0235e41b0d37869188ee7418a6531173d60d1f6a82d8f4d51fff886bd2b46f082ecbcbaa9631f139c771683cd5b0a017ac1220a931e16f1a551c23c5cb4def25cfda6ef3a00000000";

      let keys = await sourceApi.rpc.state.getKeysPaged(key, pageItemCount, startKey, sourceBlockHashAt);
      let loop: number = 0;
      let totalNumberOfKeys: number = keys.length;
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
          console.log("using sourceBlockHashAt", sourceBlockHashAt.toHex());
          if (keys.length !== 0) {
            console.log("lastKeyMigrated", keys[keys.length - 1].toHex());
          }
          console.log("totalNumberOfKeys", totalNumberOfKeys);
        }
        if (nextkeys.includes(keys[keys.length - 1]) || nextkeys.length === 0) {
          cont = false;
        } else {
          keys = nextkeys;
        }
        // TODO
        // this should probably be `... + nextKeys.length` instead of `... + keys.length`
        totalNumberOfKeys = totalNumberOfKeys + keys.length;
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
      // TODO
      // Add tail logs here
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
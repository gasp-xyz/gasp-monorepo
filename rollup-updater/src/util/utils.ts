import * as util from "node:util";
import type {
    FrameSystemEventRecord,
    PalletRolldownMessagesL1Update,
} from "@polkadot/types/lookup"
import type { Option } from "@polkadot/types-codec";


export function getMinRequestId(l2Update: any) {
    let minId = Math.min.apply(null,
        [
            l2Update[0].withdrawals,
            l2Update[0].cancels,
            l2Update[0].results
        ].flat()
            .map(function(item: any) {
                return Number(item.requestId.id);
            }))

    if (minId === Infinity) {
        return null;
    } else {
        return minId;
    }
}

function print(data: any) {
    console.log(util.inspect(data, { depth: null }));
}

export {
    print
}
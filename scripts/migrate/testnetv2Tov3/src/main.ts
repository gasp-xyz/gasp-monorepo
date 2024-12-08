import {migrateWithoutTransform} from "./migrate.js";
import { getSourceApi } from "./api.js";

export async function main() {
    console.log("Starting main...");
    const sourceApi = await getSourceApi();
    let sourceBlockHashAt = await sourceApi.rpc.chain.getBlockHash();
    await migrateWithoutTransform(sourceBlockHashAt);
}

main().catch(console.error).finally(() => process.exit());
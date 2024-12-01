import {migrateWithoutTransform} from "./migrate.js";

export async function main() {
    console.log("Starting main...");
    await migrateWithoutTransform();
}

main().catch(console.error).finally(() => process.exit());
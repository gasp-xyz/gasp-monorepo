import { Mangata } from "@mangata-finance/sdk"


const main = async (): Promise<void> => {
    const api = await Mangata.instance(["wss://collator-01-ws-rollup-dev.gasp.xyz"]).api()
    console.log(api.isConnected)
}

main().then(() => console.log("done"))
import type { Option} from '@polkadot/types-codec';
import { Mangata as SourceMangata } from "gasp-sdk-source";
import { Mangata as TargetMangata } from "gasp-sdk-target";
import { type ApiPromise } from "@polkadot/api";
import { SOURCE_URL, TARGET_URL } from "./Config.js";

async function getSourceApi(nodeUrl: string = SOURCE_URL): Promise<ApiPromise> {
	const api = await SourceMangata.instance([nodeUrl]).api();
	await api.isReady;
	return api;
}

async function getTargetApi(nodeUrl: string = TARGET_URL): Promise<ApiPromise> {
	const api = await TargetMangata.instance([nodeUrl]).api();
	await api.isReady;
	return api;
}

export {getSourceApi, getTargetApi };

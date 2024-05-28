import {getEntryFromBlsApkRegistry} from "./operatorUtilities";
import {expect} from "@jest/globals";

export async function validateBLSApkRegistry(publicClient: any, operatorAddress: string, operatorId: string) {
    const response = await getEntryFromBlsApkRegistry(publicClient, "getRegisteredPubkey", [operatorAddress] );
    const g1Point = response[0];
    expect(response[1]).toBe(operatorId);
    const pubKey = await getEntryFromBlsApkRegistry(publicClient, "operatorToPubkey", [operatorAddress] );
    expect(pubKey[0].toString()).toBe(JSON.parse(JSON.stringify(g1Point)).X);
    expect(pubKey[1].toString()).toBe(JSON.parse(JSON.stringify(g1Point)).Y);
    const pubKeyHash = await getEntryFromBlsApkRegistry(publicClient, "operatorToPubkeyHash", [operatorAddress] );
    const operatorAddr = await getEntryFromBlsApkRegistry(publicClient, "getOperatorFromPubkeyHash", [pubKeyHash] );
    expect(operatorAddr).toBe(operatorAddress);


}
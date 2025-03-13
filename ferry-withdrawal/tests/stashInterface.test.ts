import { describe, expect, it, vi, beforeEach } from "vitest";
import { stashResponseSchema } from "../src/stash/StashApi.js";

describe('StashInterface tests', () => {


  it('verify example response agains schema', async () => {

    const stashRespoonse =
    {
      transaction: {
        requestId: 1,
        txHash: "0x0fbe055def1246edc5527effcd148121384ec0980f7dada0e1f86e2e005e89af",
        address: "0xf525b86742d20ca26069d39c0c5cbc7fcd6148ed",
        created: 1736776886000,
        updated: 1736776891649,
        status: "BatchedForL1",
        type: "withdrawal",
        chain: "Ethereum",
        amount: "0.3",
        asset_chainId: "unknown",
        asset_address: "0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3",
        proof: "0x00",
        calldata: "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000928f1040adb982d3ab32a62dc8eda57e9b81b4dd000000000000000000000000c351628eb244ec633d5f21fbd6621e1a683b118100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000",
        createdBy: "frontend",
        closedBy: null
      }
    }

    const response = stashResponseSchema.parse(stashRespoonse);
    expect(response.transaction.createdBy).toBe("frontend");
  });

})

import argparse
import json
import os
import sys
from substrateinterface import SubstrateInterface, Keypair, KeypairType
from pprint import pprint
import time
from decimal import Decimal

parser = argparse.ArgumentParser(description="Send airdrop to addresses from json.")
parser.add_argument(
    "--address",
    type=str,
    help="The address used to distribute the funds.",
    required=True,
)
parser.add_argument(
    "--block",
    type=int,
    help="The block number to reverse search from for executed transfers, transfer search goes from block N down to block where nonce is 0. Default is the latest block",
    required=False,
)
parser.add_argument(
    "--nonce",
    type=int,
    help="The nonce for the initial check to end at, default is 0 and will check all transactions from the given address",
    required=False,
    default=0,
)
parser.add_argument(
    "--json", type=str, help="The input file for distribution", required=True
)
parser.add_argument(
    "--url",
    type=str,
    help="The url of the substrate node",
    required=False,
    default="ws://127.0.0.1:9944",
)
parser.add_argument(
    "--parsed",
    type=str,
    help="The file that caches the transactions",
    required=False,
)

args = parser.parse_args()

address = args.address
print(f"Sender address: {address}")

with open(args.json, "r") as f:
    airdrop = json.load(f, parse_float=Decimal)

timestamp = int(time.time())
out_file = f"out/{address}_{timestamp}.json"
input_filename = os.path.basename(args.json)
out_file_aidrop = f"out/{os.path.splitext(input_filename)[0]}_checked.json"
output_dir = os.path.dirname(out_file)
if output_dir and not os.path.exists(output_dir):
    os.makedirs(output_dir)


def get_block_hash(substrate, block_number):
    if block_number is None:
        return substrate.get_chain_head()
    else:
        return substrate.get_block_hash(block_number)


def get_nonce(substrate, at=None):
    return substrate.query(
        module="System",
        storage_function="Account",
        params=[address],
        block_hash=at,
    )["nonce"].value


def check_transfers(substrate, to_nonce, at=None):
    nonce = get_nonce(substrate, at)
    print(f"check transfers from nonce: {nonce} to: {to_nonce}")
    filtered = []

    if nonce == 0:
        print("No extrinsics found")
        # sys.exit()

    while nonce > to_nonce:
        block = substrate.get_block(block_hash=at)
        exts = block["extrinsics"]
        events_at = substrate.get_events(at)
        print(f"block: {block['header']['number']}, nonce: {nonce}")

        for idx, extrinsic in enumerate(exts):
            if (
                "address" in extrinsic.value
                and extrinsic["address"].value.lower() == address.lower()
            ):
                nonce -= 1
                events = list(
                    map(
                        lambda ev: ev.value["event"],
                        filter(
                            lambda ev: ev.value["extrinsic_idx"] == idx,
                            events_at,
                        ),
                    )
                )

                failed = False
                meta = None
                for event in events:
                    if event["event_id"] == "ExtrinsicFailed":
                        meta = None
                        break

                    if event["event_id"] == "Transfer":
                        meta = {
                            "block": block["header"]["number"],
                            "to": event["attributes"]["to"],
                            "amount": event["attributes"]["amount"],
                        }
                        break

                if meta is None:
                    continue

                filtered.append(meta)
                print(
                    f"block: {block['header']['number']}, nonce: {nonce}, trx count: {len(filtered)}"
                )

        at = block["header"]["parentHash"]

        with open(out_file, "w") as out:
            json.dump(filtered, out, default=str, indent=2)


with SubstrateInterface(url=args.url) as substrate:
    input = None
    if args.parsed is None:
        at = get_block_hash(substrate, args.block)
        check_transfers(substrate, args.nonce, at)
        input = out_file

    else:
        input = args.parsed

    if address not in input:
        print(f"Provided address does not match parsed data: {address} != {input}")
        sys.exit()

    with open(input, "r") as f:
        parsed = json.load(f)

    print(f"Checking for existing transactions in {len(parsed)} entries, for air drop of {len(airdrop)} entries")

    cout = 0
    airdrop_len = len(airdrop)
    copy_airdrop = airdrop.copy()
    for idx, entry in enumerate(copy_airdrop):
        for trx in parsed:
            amount = Decimal(entry["amount"]) * 10**18
            if entry["address"] == trx["to"] and amount == trx["amount"]:
                cout += 1
                print(f"Found existing trx for {trx['to']} & {trx['amount']}; {cout}/{airdrop_len}")
                try:
                    airdrop.remove(entry)
                except ValueError:
                    print(f"Entry {idx} already removed, DOUBLE SPEND!!!!")

    with open(out_file_aidrop, "w") as out:
        json.dump(airdrop, out, default=str, indent=2)
    
    print(f"Saving progress, entries left: {len(airdrop)} of {airdrop_len}")

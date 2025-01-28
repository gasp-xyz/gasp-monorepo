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
    "--private-key",
    type=str,
    help="The private key that will distribute the funds.",
    required=True,
)
parser.add_argument(
    "--block",
    type=int,
    help="The block number to reverse search from for executed transfers",
    required=False,
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
    "--check",
    help="check only",
    action="store_true",
)

args = parser.parse_args()

# Load the private key and get the corresponding address
privatekey = bytes.fromhex(args.private_key)
keypair = Keypair.create_from_private_key(privatekey, crypto_type=KeypairType.ECDSA)
address = keypair.ss58_address
print(f"Sender address: {address}")


with open(args.json, "r") as f:
    airdrop = json.load(f, parse_float=Decimal)

timestamp = int(time.time())
out_file = f"out/aidrop_distribution_{timestamp}.json"
output_dir = os.path.dirname(out_file)
if output_dir and not os.path.exists(output_dir):
    os.makedirs(output_dir)

with open(out_file, "w") as out:
    json.dump(airdrop, out, default=str)


BATCH_SIZE = 50


def send_airdrop_batch(substrate, batch):
    nonce = get_nonce(substrate, None)

    for entry in batch:
        recipient = entry["address"]
        amount = entry["amount"] * 10**18

        call = substrate.compose_call(
            call_module="Tokens",
            call_function="transfer",
            call_params={"dest": recipient, "amount": amount, "currency_id": 0},
        )
        extrinsic = substrate.create_signed_extrinsic(
            call=call, keypair=keypair, nonce=nonce
        )
        result = substrate.submit_extrinsic(extrinsic)
        print(f"Sent {amount:.0f} to {recipient} with nonce {nonce}")
        nonce += 1


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


def check_transfers(substrate, to_nonce, batch, at=None):
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
                print(f"block: {block['header']['number']}, nonce: {nonce}, trx count: {len(filtered)}")

        at = block["header"]["parentHash"]

        # pprint(filtered)

    print(f"Found {len(filtered)} transfers, checking against airdrop list")
    for idx, entry in enumerate(batch):
        for meta in filtered:
            amount = entry["amount"] * 10**18
            if entry["address"] == meta["to"] and amount == meta["amount"]:
                print(f"Found existing trx for {meta['to']} & {meta['amount']}")
                try:
                    airdrop.remove(entry)
                except ValueError:
                    print(f"Entry {idx} already removed, DOUBLE SPEND!!!!")

    print(f"Saving progress, entries left: {len(airdrop)}")
    with open(out_file, "w") as out:
        json.dump(airdrop, out, default=str, indent=2)


with SubstrateInterface(url=args.url) as substrate:
    total = len(airdrop)
    # total = 20

    at = get_block_hash(substrate, args.block)
    check_transfers(substrate, 0, airdrop[0:total], at)

    if args.check:
        sys.exit()

    # we are removing entries from airdrop object, so we need to copy it
    copy = airdrop.copy()
    for i in range(0, total, BATCH_SIZE):
        batch = copy[i : i + BATCH_SIZE]
        nonce = get_nonce(substrate)
        send_airdrop_batch(substrate, batch)

        time.sleep(12)
        check_transfers(substrate, nonce, batch)

# Airdrop Send Script

This script is used to send airdrops to multiple addresses from a JSON file using the Substrate blockchain.

## Installation

1. Ensure you have Python 3.12 installed. You can download it from the [official Python website](https://www.python.org/downloads/).

2. Create a virtual environment:

   ```sh
   python3.12 -m venv venv
   ```

3. Activate the virtual environment:

   - On Windows:
     ```sh
     venv\Scripts\activate
     ```
   - On macOS and Linux:
     ```sh
     source venv/bin/activate
     ```

4. Install the required dependencies:

   ```sh
   pip install -r requirements.txt
   ```

### Using Virtualenv with Specific Python Version

If you already have Python installed and want to ensure you are using Python 3.12, you can create a virtual environment with the specific Python version using `virtualenv`.

1.  Install `virtualenv` if you haven't already:

    ```sh
    pip install virtualenv
    ```

2.  Create a virtual environment with the specific Python version:

    ```sh
    virtualenv --python="/usr/bin/python3.12" "/path/to/new/virtualenv/"
    ```

3.  Activate the virtual environment:

    - On Windows:
      ```sh
      /path/to/new/virtualenv/Scripts/activate
      ```
    - On macOS and Linux:
      ```sh
      source /path/to/new/virtualenv/bin/activate
      ```

4.  Install the required dependencies:
    ```sh
    pip install -r requirements.txt
    ```

## Usage

To use the script, run the following command with the appropriate arguments:

```sh
python airdrop_send.py --private-key <PRIVATE_KEY> --json <JSON_FILE> [--block <BLOCK_NUMBER>] [--url <NODE_URL>] [--check]
```

### Arguments

- `--private-key`: The private key that will distribute the funds. No quotes or 0x prefix. (Required)
- `--json`: The input file for distribution. (Required)
- `--block`: The block number to reverse search from for executed transfers, transfer search goes from block N down to block where nonce is 0. Default is latest block. (Optional)
- `--nonce`: The nonce for the initial check to end at, default is 0. Use if address has previous trx we want to ignore when checking for transfers. (Optional)
- `--url`: The URL of the Substrate node. Default is `ws://127.0.0.1:9944`. (Optional)
- `--check`: Check only, do not send transactions. (Optional)

### Example

```sh
python airdrop_send.py --private-key YourPrivateKeyWithout0xPrefix --json airdrop_list.json --nonce 10 --block 123456 --url ws://127.0.0.1:9944 --check
```

This command will check the airdrop list, without sending any transactions, against the blockchain, starting from block 123456 down to the block where the provided address's nonce is 10.
The block is helpful in case of `--check` flag, where the search starts from some block in history. Eg. if the current block is 500, and we know airdrop happend at blocks below 100, we can pass `--block 100` and the search will go down from 100 instead of 500.
The nonce is helpful in case there are previous trx from the given address we want to ignore. Eg. some trx happend at block 100, the nonce was 5. Now we start the script at block 5000, we should pass `--nonce 5` to ignore the unrelated trx. Otherwise the script will search ALL the blocks with trxs from the address.
It is recommended to use archive node when checking, if the trxs happened some time ago.

```sh
python airdrop_send.py --private 8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b  --json out/airdrop_distribution.json
```

This command will check ALL transactions of 'Bob' from latest block, and send the remaining transfers until done. In case of failure or interruption the output will contain the remaining entries. The output json can be used to resume the airdrop.

## Result

After running the script, an output file will be generated in the `out` directory with the name format `aidrop_distribution_<timestamp>.json`. This file contains the updated airdrop list after processing.

### Input vs Output

- **Input JSON File**: This file contains the initial list of addresses and amounts to be airdropped.
- **Output JSON File**: This file contains the remaining addresses and amounts that still need to be airdropped after the script has processed the transactions. If a transaction was successfully sent, the corresponding entry will be removed from the output file.

The output file helps in tracking the progress of the airdrop distribution and ensures that no duplicate transactions are sent.

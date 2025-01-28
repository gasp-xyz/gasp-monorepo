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

- `--private-key`: The private key that will distribute the funds. (Required)
- `--json`: The input file for distribution. (Required)
- `--block`: The block number to reverse search from for executed transfers. (Optional)
- `--url`: The URL of the Substrate node. Default is `ws://127.0.0.1:9944`. (Optional)
- `--check`: Check only, do not send transactions. (Optional)

### Example

```sh
python airdrop_send.py --private-key YourPrivateKeyWithout0xPrefix --json airdrop_list.json --block 123456 --url ws://127.0.0.1:9944 --check
```

This command will check the airdrop list, without sending any transactions, against the blockchain starting from block 123456 until the provided address's nonce is 0.
It is recommended to use archive node when checking, if the trxs happened some time ago.

## Result

After running the script, an output file will be generated in the `out` directory with the name format `aidrop_distribution_<timestamp>.json`. This file contains the updated airdrop list after processing.

### Input vs Output

- **Input JSON File**: This file contains the initial list of addresses and amounts to be airdropped.
- **Output JSON File**: This file contains the remaining addresses and amounts that still need to be airdropped after the script has processed the transactions. If a transaction was successfully sent, the corresponding entry will be removed from the output file.

The output file helps in tracking the progress of the airdrop distribution and ensures that no duplicate transactions are sent.

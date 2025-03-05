"""
📌 MinaMesh Transaction Sender Script

This script automates the process of sending a payment transaction via the MinaMesh Construction API.
It follows these steps:

1️⃣ **Preprocess** - Prepares the transaction structure.
2️⃣ **Metadata** - Retrieves the nonce and suggested fee.
3️⃣ **Payloads** - Generates the unsigned transaction.
4️⃣ **Parse** - Parses the unsigned transaction. (optional)
5️⃣ **Sign** - Uses `signer.exe` (offline OCaml binary) to sign the transaction.
6️⃣ **Parse** - Parse the signed transaction. (optional)
7️⃣ **Combine** - Merges the signature with the unsigned transaction.
8️⃣ **Submit** - Sends the signed transaction to the Mina network.

⚠️ **Prerequisites:**
- `signer.exe` (the Mina Rosetta OCaml signer) must be installed and available on the system `PATH`.  
  - See: [Mina Docs](https://docs.minaprotocol.com/exchange-operators/rosetta/samples/using-signer)

🔹 **Usage:**
    python send_payment.py <sender> <sender_private_key> <amount> <receiver>

🔹 **Example:**
    python send_payment.py B62qnuDy... <PRIVATE_KEY> 1000000000 B62qnvdfRm...

🔹 **Output:**
    ✅ Preprocess done
    ✅ Metadata done | Nonce: 3 | Suggested Fee: 100000000
    ✅ Payloads done
    ✅ Parse Unsigned Transaction done
    ✅ Signed Transaction | Signature: C8103A85D...
    ✅ Combine done
    ✅ Parse Signed Transaction done
    ✅ Transaction Submitted! Hash: 5Jv8CPtFpypbcpfGy5WczpTzLG...
    🔗 Transaction URL: https://minascan.io/devnet/tx/5Jv8CPtFpypbcpfGy5WczpTzLG...

"""

import requests
import json
import subprocess
import sys

# 🌍 Mina Mesh Construction API URL
API_URL = "http://localhost:3000/construction"
NETWORK = "devnet"


# 📝 Function to send POST requests
def post_request(endpoint, data):
    url = f"{API_URL}/{endpoint}"
    headers = {"Content-Type": "application/json", "Accept": "application/json"}
    response = requests.post(url, headers=headers, json=data)
    if response.status_code != 200:
        print(f"❌ Error in {endpoint}: {response.text}")
        sys.exit(1)
    return response.json()


# ✍️ Function to sign the transaction using signer.exe
def sign_transaction(unsigned_tx, private_key):
    command = [
        "signer.exe",
        "sign",
        "-private-key",
        private_key,
        "-unsigned-transaction",
        unsigned_tx,
    ]
    try:
        result = subprocess.run(command, capture_output=True, text=True, check=True)
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"❌ Error in signing: {e.stderr}")
        sys.exit(1)


# ⚙️ Payment transaction operations
def operations(sender, amount, receiver, fee="100000000"):
    return [
        {
            "operation_identifier": {"index": 0},
            "related_operations": [],
            "type": "fee_payment",
            "account": {"address": sender, "metadata": {"token_id": "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf"}},
            "amount": {
                "value": "-" + fee,
                "currency": {"symbol": "MINA", "decimals": 9},
            },
        },
        {
            "operation_identifier": {"index": 1},
            "related_operations": [],
            "type": "payment_source_dec",
            "account": {"address": sender, "metadata": {"token_id": "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf"}},
            "amount": {
                "value": "-" + amount,
                "currency": {"symbol": "MINA", "decimals": 9},
            },
        },
        {
            "operation_identifier": {"index": 2},
            "related_operations": [{"index": 1}],
            "type": "payment_receiver_inc",
            "account": {"address": receiver, "metadata": {"token_id": "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf"}},
            "amount": {
                "value": amount,
                "currency": {"symbol": "MINA", "decimals": 9},
            },
        },
    ]


# 🚀 **Main function to execute the payment transaction**
def send_payment(sender, sender_pvk, amount, receiver):
    # 1️⃣ **Preprocess**
    preprocess_data = {
        "network_identifier": {"blockchain": "mina", "network": NETWORK},
        "operations": operations(sender, amount, receiver),
        "metadata": {"memo": "hello", "valid_until": "200000000"},
    }
    preprocess_response = post_request("preprocess", preprocess_data)
    print("✅ Preprocess done")

    # 2️⃣ **Metadata**
    metadata_data = {
        "network_identifier": {"blockchain": "mina", "network": NETWORK},
        "options": preprocess_response["options"],
    }
    metadata_response = post_request("metadata", metadata_data)
    nonce = metadata_response["metadata"]["nonce"]
    suggested_fee = metadata_response["suggested_fee"][0]["value"]
    print(f"✅ Metadata done | Nonce: {nonce} | Suggested Fee: {suggested_fee}")

    # 3️⃣ **Payloads**
    payloads_data = {
        "network_identifier": {"blockchain": "mina", "network": NETWORK},
        "operations": operations(sender, amount, receiver, suggested_fee),
        "metadata": {**metadata_response["metadata"], "nonce": nonce},
    }
    payloads_response = post_request("payloads", payloads_data)
    unsigned_tx = payloads_response["unsigned_transaction"]
    payload_hex = payloads_response["payloads"][0]["hex_bytes"]
    print("✅ Payloads done")

    # 4️⃣ **Parse** 
    parse_data = {
        "network_identifier": {"blockchain": "mina", "network": NETWORK},
        "signed": False,
        "transaction": unsigned_tx,
    }
    parse_response = post_request("parse", parse_data)
    print(f"✅ Parse Unsigned Transaction done")

    # 5️⃣ **Sign Transaction**
    signature = sign_transaction(unsigned_tx, sender_pvk)
    print(f"✅ Signed Transaction | Signature: {signature}")

    # 6️⃣ **Combine**
    combine_data = {
        "network_identifier": {"blockchain": "mina", "network": NETWORK},
        "signatures": [
            {
                "hex_bytes": signature,
                "signature_type": "schnorr_poseidon",
                "public_key": {"curve_type": "tweedle", "hex_bytes": payload_hex},
                "signing_payload": {"hex_bytes": payload_hex},
            }
        ],
        "unsigned_transaction": unsigned_tx,
    }
    combine_response = post_request("combine", combine_data)
    signed_transaction = combine_response["signed_transaction"]
    print("✅ Combine done")

    # 7️⃣ **Parse**
    parse_data = {
        "network_identifier": {"blockchain": "mina", "network": NETWORK},
        "signed": True,
        "transaction": signed_transaction,
    }
    parse_response = post_request("parse", parse_data)
    print(f"✅ Parse Signed Transaction done")

    # 8️⃣ **Submit**
    submit_data = {
        "network_identifier": {"blockchain": "mina", "network": NETWORK},
        "signed_transaction": signed_transaction,
    }
    submit_response = post_request("submit", submit_data)
    transaction_hash = submit_response["hash"]
    print(f"✅ Transaction Submitted! Hash: {transaction_hash}")
    print(f"🔗 Transaction URL:  https://minascan.io/{NETWORK}/tx/{transaction_hash}")

    print("\n 🚀 Submit curl:")
    print(
        f"curl -X POST -H 'Content-Type: application/json' -d '{json.dumps(submit_data)}' {API_URL}/submit"
    )


# 🏃 **Run the script with CLI arguments**
if __name__ == "__main__":
    if len(sys.argv) != 5:
        print("Usage: python send_payment.py <sender> <sender_pvk> <amount> <receiver>")
        sys.exit(1)

    sender = sys.argv[1]
    sender_pvk = sys.argv[2]
    amount = sys.argv[3]
    receiver = sys.argv[4]

    send_payment(sender, sender_pvk, amount, receiver)

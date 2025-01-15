# L1X Transaction Hash Tool

A lightweight command-line utility for calculating transaction hash values in the L1X blockchain ecosystem.

## 📋 Overview

The `l1x-tx-hash-tool` takes a JSON-formatted transaction as input and generates its corresponding hash value, which is essential for transaction verification and signing processes in the L1X blockchain.

## 🚀 Quick Start

### Prerequisites

- Rust toolchain (latest stable version)
- Cargo package manager

### Installation

```bash
# Clone the repository
git clone [repository-url]
cd l1x-tx-hash-tool

# Build the release version
cargo build --release
```

### How to run

```bash
./target/release/l1x-tx-hash-tool --json 'your_json_string'
```

### Errors

If the tool fails, it prints an error and exits with non zero exit code

```bash
./target/release/l1x-tx-hash-tool --json '{' ; echo $?
```

**Output:**
```
Error: Can't parse json str: EOF while parsing an object at line 1 column 1
1
```

### Example

```bash
./target/release/l1x-tx-hash-tool --json '{
    "nonce":"133",
    "transaction_type":{
        "NativeTokenTransfer":{
            "address":[122,64,57,150,93,21,42,221,43,160,66,48,160,2,195,85,183,91,181,41],
            "amount":"1147999999999999999998"
        }
    },
    "fee_limit":"1",
    "signature":[34,54,100,37,247,5,225,23,153,23,235,35,200,149,5,23,52,252,209,150,80,174,206,155,44,14,219,210,198,203,27,2,52,204,43,
                    58,168,179,19,179,234,121,114,234,235,29,208,27,243,69,68,89,201,15,147,97,26,250,86,43,203,24,126,159],
    "verifying_key":[2,183,104,192,77,23,63,57,139,219,110,116,87,123,254,13,12,156,181,235,101,159,183,130,67,203,111,83,132,17,97,184,33]
}'
```

**Output**
```
067dc0d4693c90af7d12e61b8f6396c39bd4007899fe30fbcaa5ea5f9b7de9c8
```

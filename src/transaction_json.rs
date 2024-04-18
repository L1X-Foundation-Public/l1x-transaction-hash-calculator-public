use anyhow::{anyhow, Error};
use serde::Deserialize;

use crate::transaction::Transaction as SystemTransaction;
use crate::transaction::TransactionType as SystemTransactionType;
use crate::types as sys_types;

#[derive(Debug, Deserialize, Clone)]
pub struct TransactionJson {
    nonce: String,
    transaction_type: TransactionTypeJson,
    fee_limit: String,
    signature: sys_types::SignatureBytes,
    verifying_key: sys_types::VerifyingKeyBytes,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TransactionTypeJson {
    NativeTokenTransfer {
        address: sys_types::Address,
        amount: String,
    },
}

impl TransactionJson {
    pub fn from_json_str(json_str: &str) -> Result<Self, Error> {
        let json_tx: TransactionJson =
            serde_json::from_str(&json_str).map_err(|e| anyhow!("Can't parse json str: {e}"))?;

        Ok(json_tx)
    }

    pub fn transaction_hash(&self) -> Result<sys_types::TransactionHash, Error> {
        let system_tx: SystemTransaction = self.clone().try_into()?;
        system_tx.transaction_hash()
    }
}

impl TryInto<SystemTransaction> for TransactionJson {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<SystemTransaction, Error> {
        Ok(SystemTransaction {
            fee_limit: sys_types::Balance::from_str_radix(&self.fee_limit, 10)
                .map_err(|e| anyhow!("Can't convert 'fee_limit' from string: {e}"))?,
            nonce: sys_types::Nonce::from_str_radix(&self.nonce, 10)
                .map_err(|e| anyhow!("Can't convert 'nonce' from string: {e}"))?,
            transaction_type: self.transaction_type.try_into()?,
            signature: self.signature,
            verifying_key: self.verifying_key,
            eth_original_transaction: None,
        })
    }
}

impl TryInto<SystemTransactionType> for TransactionTypeJson {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<SystemTransactionType, Error> {
        match self {
            Self::NativeTokenTransfer { address, amount } => {
                Ok(SystemTransactionType::NativeTokenTransfer(
                    address,
                    sys_types::Balance::from_str_radix(&amount, 10)
                        .map_err(|e| anyhow!("Can't convert 'amount' from string: {e}"))?,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TransactionJson;
    use crate::transaction::Transaction as SystemTransaction;

    #[test]
    fn test_hash() {
        let json_str = r#"{
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
        }"#;
        let json_tx: TransactionJson = serde_json::from_str(&json_str).unwrap();
        let system_tx: SystemTransaction = json_tx.clone().try_into().unwrap();

        let system_tx_hash = hex::encode(system_tx.transaction_hash().unwrap());
        let expected_hash = "26c649c0d4373ff1c3b6e129d531843af085ed3ce75ebf588766d811268a385a";
        let json_tx_hash = hex::encode(json_tx.transaction_hash().unwrap());

        assert_eq!(system_tx_hash, expected_hash);
        assert_eq!(json_tx_hash, expected_hash);
    }
}

use anyhow::{anyhow, Error};
use serde::Deserialize;

use crate::transaction::Transaction as SystemTransaction;
use crate::transaction::TransactionType as SystemTransactionType;
use crate::transaction::TransactionVersion;
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
            version: TransactionVersion::default(),
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
            "fee_limit":"1000000",
            "nonce":"297",
            "signature":[56,186,206,101,14,96,243,26,67,59,102,94,181,124,10,232,29,124,161,242,216,31,195,45,75,60,218,169,206,116,107,81,89,143,94,
                        251,30,214,208,192,173,136,5,133,209,205,183,250,163,89,22,22,75,10,30,7,125,137,29,237,172,9,147,96],
            "transaction_type":{
                "NativeTokenTransfer":{
                    "address":[59,100,123,70,201,186,79,202,34,28,207,147,60,9,182,83,194,180,88,31],
                    "amount":"1"
                }
            },
            "verifying_key":[2,151,109,120,119,219,158,64,102,159,235,19,99,3,150,218,222,126,4,92,210,104,221,83,230,134,100,127,247,122,126,66,164]
        }"#;
        let json_tx: TransactionJson = serde_json::from_str(&json_str).unwrap();
        let system_tx: SystemTransaction = json_tx.clone().try_into().unwrap();

        let system_tx_hash = hex::encode(system_tx.transaction_hash().unwrap());
        let expected_hash = "1a2b4a6e280bfff6773da10c3d02e732ad22a6644a4b544112b44b440aced95c";
        let json_tx_hash = hex::encode(json_tx.transaction_hash().unwrap());

        assert_eq!(system_tx_hash, expected_hash);
        assert_eq!(json_tx_hash, expected_hash);
    }
}
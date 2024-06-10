use serde::{Deserialize, Serialize};

use crate::types::{
    AccessType, Address, Balance, BlockNumber, ContractArgument, ContractCode, ContractFunction,
    Nonce, Salt, SignatureBytes, TransactionHash, VerifyingKeyBytes,
};
use anyhow::{anyhow, Error, Result};
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionV1 {
    pub nonce: Nonce,
    pub transaction_type: TransactionType,
    pub fee_limit: Balance,
    #[serde(with = "serde_bytes")]
    pub signature: SignatureBytes,
    #[serde(with = "serde_bytes")]
    pub verifying_key: VerifyingKeyBytes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub nonce: Nonce,
    pub transaction_type: TransactionType,
    pub fee_limit: Balance,
    #[serde(with = "serde_bytes")]
    pub signature: SignatureBytes,
    #[serde(with = "serde_bytes")]
    pub verifying_key: VerifyingKeyBytes,
    #[serde(with = "serde_bytes")]
    pub eth_original_transaction: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    NativeTokenTransfer(Address, Balance),
    SmartContractDeployment {
        access_type: AccessType,
        contract_type: ContractType,
        contract_code: ContractCode,
        value: Balance,
        salt: Salt,
    },
    SmartContractInit(Address, ContractArgument),
    SmartContractFunctionCall {
        contract_instance_address: Address,
        function: ContractFunction,
        arguments: ContractArgument,
    },
    CreateStakingPool {
        contract_instance_address: Option<Address>,
        min_stake: Option<Balance>,
        max_stake: Option<Balance>,
        min_pool_balance: Option<Balance>,
        max_pool_balance: Option<Balance>,
        staking_period: Option<BlockNumber>,
    },
    Stake {
        pool_address: Address,
        amount: Balance,
    },
    UnStake {
        pool_address: Address,
        amount: Balance,
    },
    StakingPoolContract {
        pool_address: Address,
        contract_instance_address: Address,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum ContractType {
    L1XVM = 0,
    EVM = 1,
    XTALK = 2,
}

impl Transaction {
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        match bincode::serialize(self) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(anyhow!("Error: {:?}", e)),
        }
    }

    pub fn transaction_hash(&self) -> Result<TransactionHash, Error> {
        let tx_bytes = self.clone().as_bytes()?;

        // Create a Keccak-256 hasher
        let mut hasher = Keccak256::new();

        // Update the hasher with the transaction bytes
        hasher.update(&tx_bytes);

        // Obtain the hash result as a fixed-size array
        let result: TransactionHash = hasher.finalize().into();

        Ok(result)
    }
}

impl TryInto<ContractType> for i8 {
    type Error = Error;

    fn try_into(self) -> Result<ContractType, Self::Error> {
        match self {
            0 => Ok(ContractType::L1XVM),
            1 => Ok(ContractType::EVM),
            2 => Ok(ContractType::XTALK),
            _ => Err(anyhow!("Invalid contract type {}", self)),
        }
    }
}

impl From<TransactionV1> for Transaction {
    fn from(value: TransactionV1) -> Self {
        Self {
            nonce: value.nonce,
            transaction_type: value.transaction_type,
            fee_limit: value.fee_limit,
            signature: value.signature,
            verifying_key: value.verifying_key,
            eth_original_transaction: None,
        }
    }
}

use alloy_primitives::{keccak256, B256, U256};
use alloy_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, RlpDecodable, RlpEncodable, Serialize, Deserialize)]
pub struct AccountState {
    pub nonce: u64,
    pub balance: U256,
    pub storage_hash: B256,
    pub code_hash: B256,
}

impl Default for AccountState {
    fn default() -> Self {
        Self {
            nonce: 0,
            balance: U256::ZERO,
            storage_hash: keccak256(alloy_rlp::encode([])),
            code_hash: keccak256([]),
        }
    }
}

use alloy_primitives::{Address, BlockHash, BlockNumber, B256, U256};
use ethers::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockInfo {
    pub header: Block<Transaction>,
    pub traces: Vec<Trace>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockReward {
    pub address: Address,
    pub value: U256,
}

impl BlockInfo {
    pub fn number(&self) -> BlockNumber {
        match self.header.number {
            None => panic!("Block doesn't have number"),
            Some(number) => number.as_u64(),
        }
    }

    pub fn hash(&self) -> BlockHash {
        match self.header.hash {
            None => panic!("Block {} doesn't have hash", self.number()),
            Some(hash) => BlockHash::from_slice(hash.as_bytes()),
        }
    }

    pub fn state_root(&self) -> B256 {
        B256::from_slice(self.header.state_root.as_bytes())
    }

    pub fn block_rewards(&self) -> Vec<BlockReward> {
        self.traces
            .iter()
            .filter_map(|trace| {
                if let Action::Reward(reward) = &trace.action {
                    Some(BlockReward {
                        address: Address::from_slice(reward.author.as_bytes()),
                        value: U256::from_limbs(reward.value.0),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

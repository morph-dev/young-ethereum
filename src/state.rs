use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::BufReader,
    sync::Arc,
};

use alloy_primitives::{keccak256, Address, BlockNumber, Bytes, B256, U256};
use alloy_rlp::Decodable;
use anyhow::{ensure, Error, Result};
use cita_trie::{MemoryDB, PatriciaTrie, Trie};
use hasher::HasherKeccak;
use serde::{Deserialize, Serialize};

use crate::{account_state::AccountState, block_info::BlockInfo, Args};

#[derive(Debug, Serialize, Deserialize)]
struct AllocBalance {
    balance: U256,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenesisConfig {
    alloc: HashMap<Address, AllocBalance>,
    #[serde(rename = "stateRoot")]
    state_root: B256,
}

pub struct State {
    pub accounts: BTreeSet<Address>,
    trie: Box<dyn Trie<MemoryDB, HasherKeccak>>,
    next_block_number: BlockNumber,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountProof {
    pub address: Address,
    pub state: AccountState,
    pub proof: Vec<Bytes>,
}

impl State {
    pub fn try_new(args: &Args) -> Result<Self> {
        let mut trie = PatriciaTrie::new(
            Arc::new(MemoryDB::new(false)),
            Arc::new(HasherKeccak::new()),
        );
        let mut accounts = BTreeSet::new();

        let genesis_file = File::open(&args.genesis_file)?;
        let genesis: GenesisConfig = serde_json::from_reader(BufReader::new(genesis_file))?;

        for (address, alloc_balance) in genesis.alloc {
            accounts.insert(address);

            let mut account = AccountState::default();
            account.balance += alloc_balance.balance;
            trie.insert(key(&address).to_vec(), alloy_rlp::encode(account))?;
        }

        let root = B256::from_slice(trie.root()?.as_slice());
        ensure!(
            root == genesis.state_root,
            "Root doesn't match state root from genesis file"
        );

        Ok(Self {
            accounts: accounts,
            trie: Box::from(trie),
            next_block_number: 0,
        })
    }

    pub fn process_block(&mut self, block: &BlockInfo) -> Result<BTreeSet<Address>> {
        ensure!(
            self.next_block_number == block.number(),
            "Expected block {}, received {}",
            self.next_block_number,
            block.number()
        );

        let mut updated_accounts = BTreeSet::new();

        for reward in block.block_rewards() {
            println!("Updating account: {} {}", reward.address, reward.value);
            let address = &reward.address;
            let mut account = self.get_account_state(&address)?;
            account.balance += reward.value;
            self.trie
                .insert(key(address).to_vec(), alloy_rlp::encode(account))?;

            self.accounts.insert(address.clone());
            updated_accounts.insert(address.clone());
        }

        if self.get_root()? != block.state_root() {
            panic!(
                "State root doesn't match! Irreversible! Block number: {}",
                self.next_block_number
            )
        }

        self.next_block_number += 1;

        Ok(updated_accounts)
    }

    pub fn get_root(&mut self) -> Result<B256> {
        Ok(B256::from_slice(self.trie.root()?.as_slice()))
    }

    pub fn get_account_state(&self, account: &Address) -> Result<AccountState> {
        let account_state = self.trie.get(key(account).as_slice())?;
        match account_state {
            Some(encoded) => AccountState::decode(&mut encoded.as_slice()).map_err(Error::from),
            None => Ok(AccountState::default()),
        }
    }

    pub fn get_proof(&self, account: &Address) -> Result<AccountProof> {
        Ok(AccountProof {
            address: account.clone(),
            state: self.get_account_state(account)?,
            proof: self
                .trie
                .get_proof(key(account).as_slice())?
                .into_iter()
                .map(Bytes::from)
                .collect(),
        })
    }

    pub fn get_proofs(&self, accounts: &BTreeSet<Address>) -> Result<Vec<AccountProof>> {
        accounts
            .iter()
            .map(|account| self.get_proof(account))
            .collect()
    }
}

fn key(account: &Address) -> B256 {
    keccak256(account)
}

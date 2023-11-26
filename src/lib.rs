mod account_state;
mod block_fetcher;
mod block_info;
mod state;

use std::{
    collections::BTreeMap,
    fs::{create_dir_all, File},
    io::BufWriter,
    path::PathBuf,
};

use alloy_primitives::{keccak256, BlockHash, BlockNumber, Bytes, B256};
use anyhow::{ensure, Error, Result};
use block_fetcher::BlockFetcher;
use block_info::BlockInfo;
use clap::Parser;
use serde::{Deserialize, Serialize};
use state::{AccountProof, State};

const RPC_URL: &str = "https://rpc.ankr.com/eth";
const OUTPUT_DIRECTORY: &str = "output";
const GENESIS_FILEPATH: &str = "genesis.json";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short)]
    pub blocks: u64,

    #[arg(long, default_value_t = String::from(OUTPUT_DIRECTORY))]
    pub output_directory: String,

    #[arg(long, default_value_t = String::from(GENESIS_FILEPATH))]
    pub genesis_file: String,

    #[arg(long, default_value_t = String::from(RPC_URL))]
    pub rpc_url: String,

    #[arg(long)]
    pub disable_full_state_proof_per_block: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockProof {
    block: BlockNumber,
    block_hash: BlockHash,
    state_root: B256,
    proofs: Vec<AccountProof>,
}

fn write_to_file<T: Serialize>(args: &Args, t: T, filename: String) -> Result<()> {
    create_dir_all(&args.output_directory)?;

    let mut path = PathBuf::from(&args.output_directory);
    path.push(filename);

    let file = File::create(path)?;
    serde_json::to_writer_pretty(BufWriter::new(file), &t).map_err(Error::from)
}

fn write_block_proof_to_file(
    args: &Args,
    block: &BlockInfo,
    proofs: Vec<AccountProof>,
    filename: String,
) -> Result<()> {
    let block_proof = BlockProof {
        block: block.number(),
        block_hash: block.hash(),
        state_root: block.state_root(),
        proofs,
    };
    write_to_file(args, block_proof, filename)
}

fn update_tree_nodes(tree_nodes: &mut BTreeMap<B256, Bytes>, proofs: &Vec<AccountProof>) {
    proofs
        .iter()
        .flat_map(|account_proof| &account_proof.proof)
        .for_each(|proof| {
            let key = keccak256(proof);
            if !tree_nodes.contains_key(&key) {
                tree_nodes.insert(key, proof.clone());
            }
        });
}

pub async fn run() -> Result<()> {
    let args = Args::parse();

    let block_fetcher = BlockFetcher::try_new(&args)?;

    let mut state = State::try_new(&args)?;

    let mut all_tree_nodes: BTreeMap<B256, Bytes> = BTreeMap::new();
    update_tree_nodes(&mut all_tree_nodes, &state.get_proofs(&state.accounts)?);

    for block_number in 0..=args.blocks {
        println!("Processing block: {}", block_number);

        let block = block_fetcher.get_block_info(block_number).await?;
        ensure!(block_number == block.number());

        let updated_accounts = state.process_block(&block)?;

        // Updated accounts

        let updated_account_proofs = state.get_proofs(&updated_accounts)?;
        update_tree_nodes(&mut all_tree_nodes, &updated_account_proofs);

        write_block_proof_to_file(
            &args,
            &block,
            updated_account_proofs,
            format!("proofs.block.{}.partial.json", block_number),
        )?;

        // All accounts

        if !args.disable_full_state_proof_per_block {
            let all_account_proofs = state.get_proofs(&state.accounts)?;

            let mut block_tree_nodes: BTreeMap<B256, Bytes> = BTreeMap::new();
            update_tree_nodes(&mut block_tree_nodes, &all_account_proofs);
            write_to_file(
                &args,
                block_tree_nodes,
                format!("tree.block.{}.json", block_number),
            )?;

            write_block_proof_to_file(
                &args,
                &block,
                all_account_proofs,
                format!("proofs.block.{}.full.json", block_number),
            )?;
        }
    }

    write_to_file(
        &args,
        all_tree_nodes,
        format!("tree.archive.{}.json", args.blocks),
    )?;

    Ok(())
}

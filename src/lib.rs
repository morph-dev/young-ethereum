mod account_state;
mod block_fetcher;
mod block_info;
mod block_proof_export;
mod state;

use anyhow::Result;
use clap::Parser;

use block_proof_export::export_block_proofs;

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

pub async fn run() -> Result<()> {
    let args = Args::parse();

    export_block_proofs(&args).await
}

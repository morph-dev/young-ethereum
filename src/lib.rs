mod block_fetcher;
mod block_info;

use anyhow::Result;
use block_fetcher::BlockFetcher;
use clap::Parser;

const RPC_URL: &str = "https://eth.llamarpc.com";
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
}

pub async fn run() -> Result<()> {
    let args = Args::parse();

    let block_fetcher = BlockFetcher::try_new(&args)?;

    for b in 0..=args.blocks {
        println!("{:?}", block_fetcher.get_block_info(b).await?);
    }

    Ok(())
}

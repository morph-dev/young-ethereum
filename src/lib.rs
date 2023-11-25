use anyhow::Result;
use clap::Parser;

pub const RPC_URL: &str = "https://eth.llamarpc.com";

pub const OUTPUT_DIRECTORY: &str = "output";

pub const GENESIS_FILEPATH: &str = "genesis.json";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short)]
    pub blocks: usize,

    #[arg(long, default_value_t = String::from(OUTPUT_DIRECTORY))]
    pub output_directory: String,

    #[arg(long, default_value_t = String::from(GENESIS_FILEPATH))]
    pub genesis_file: String,

    #[arg(long, default_value_t = String::from(RPC_URL))]
    pub rpc_url: String,
}

pub async fn run() -> Result<()> {
    let args = Args::parse();

    println!("{:?}", args);

    Ok(())
}

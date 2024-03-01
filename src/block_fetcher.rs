use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use alloy_primitives::BlockNumber;
use anyhow::{anyhow, Error, Result};
use ethers::prelude::*;

use crate::{block_info::BlockInfo, Args};

pub struct BlockFetcher {
    provider: Provider<Http>,
    output_folder: PathBuf,
}

impl BlockFetcher {
    pub fn try_new(args: &Args) -> Result<Self> {
        let provider = Provider::try_from(&args.rpc_url)?;
        let output = PathBuf::from(&args.output_directory);
        create_dir_all(&output)?;
        Ok(Self {
            provider,
            output_folder: output,
        })
    }

    pub async fn get_block_info(&self, block: BlockNumber) -> Result<BlockInfo> {
        let mut path = PathBuf::from(&self.output_folder);
        path.push(format!("blocks/block.{}.json", block));

        if path.exists() {
            let reader = BufReader::new(File::open(path)?);
            serde_json::from_reader(reader).map_err(Error::from)
        } else {
            let header = self.fetch_block_header(block).await?;
            let traces = self.fetch_block_traces(block).await?;

            let block_info = BlockInfo { header, traces };

            let writer = BufWriter::new(File::create(path)?);
            serde_json::to_writer_pretty(writer, &block_info)?;

            Ok(block_info)
        }
    }

    async fn fetch_block_header(&self, block: BlockNumber) -> Result<Block<Transaction>> {
        self.provider
            .get_block_with_txs(block)
            .await?
            .ok_or(anyhow!("Fetching block {} failed!", block))
    }

    async fn fetch_block_traces(&self, block: BlockNumber) -> Result<Vec<Trace>> {
        self.provider
            .trace_block(types::BlockNumber::from(U64::from(block)))
            .await
            .map_err(Error::from)
    }
}

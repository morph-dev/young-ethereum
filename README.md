# Young ethereum - tool for analyzing the beginning of a blockchain

Lightweight tool for analyzing the beginning of the Ethereum blockchain, with focus on state tree.

## Output

All files are written to `output` directory (configurable with `--output-directory` flag).

All files are in `.json` format and most field are self explanatory, but some might require explanation:

- `block.X.json` - containes block headers and traces
- `proofs.partial.block.X.json` - contains proofs for all modified accounts within a block
    - Each proof is given as a list of tree nodes (RLP encoded), starting from the root
    - **Note:** There is no proof for block 0
- `proofs.full.block.X.json` - contains proofs for all accounts since genesis up until a block
    - Each proof is given as a list of tree nodes (RLP encoded), starting from the root
    - Can be disabled with `--disable-full-state-proof-per-block` flag
- `tree.block.X.json` - Entire tree state at a given block
    - A key-value pairs, where value is RLP encoded tree node, and key is keccak256 of it
    - Can be disabled with `--disable-full-state-proof-per-block` flag
- `tree.archive.X.json` - All tree nodes since genesis up until a block
    - A key-value pairs, where value is RLP encoded tree node, and key is keccak256 of it

## How it works

The important thing to know is that the first transaction on ethereum happened on block 46147.
Until then, state changes were happened only as a result of rewards for mining (including uncle rewards).

This program does following:

1. Initializes the state based on genesis config file
    - Path to the file is configurable with `--genesis-file` flag
    - Default genesis file was copied from [reth repo](https://github.com/paradigmxyz/reth/blob/7de2582000c3ff051dacaefd7720595e5905ed69/crates/primitives/res/genesis/mainnet.json) and it's for ethereum mainnet
1. For each block, starting from 0 and ending with `--block` flag,:
    1. Fetches the block header and trace
        - These are saved locally in `block.X.json` and fetching will be skipped if they are present
    1. Updates the tree state
    1. Export proofs for each modified account into `proofs.partial.block.X.json`
        - **Note:** there will be no proof for block 0
    1. If `--disable-full-state-proof-per-block` flag is not set:
        1. Exports proofs for all accounts in the state tree into `proofs.full.block.X.json`
        1. Exports entire state tree for current block into `tree.block.X.json`
1. Exports all state trees since genesis into `tree.archive.X.json`

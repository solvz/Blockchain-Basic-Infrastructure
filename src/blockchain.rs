use super::{block, Block, BlockHash, Hashable};
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub unspent_outputs: HashSet<BlockHash>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    /// verifies and returns true if the blockchain is valid
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            // not genesis block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            // genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((first, transactions)) = block.transactions.split_first() {
            if !first.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }
            let mut block_spent: HashSet<BlockHash> = HashSet::new();
            let mut block_created: HashSet<BlockHash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_val = transaction.total_input();
                let output_val = transaction.total_output();
                if (output_val > input_val) {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }
                let fee = input_val - output_val;
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if first.total_output() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(first.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));

            self.unspent_outputs.extend(block_created);
        };

        self.blocks.push(block);

        return Ok(());
    }
}               
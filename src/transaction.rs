use super::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> BlockHash {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));
        bytes
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn total_input(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }
    pub fn total_output(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<BlockHash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<BlockHash>>()
    }

    pub fn output_hashes(&self) -> HashSet<BlockHash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<BlockHash>>()
    }

    /// A coinbase transaction is a generation transaction.
    /// It is the transaction for when the block is created by miner.
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> BlockHash {
        let mut bytes = vec![];
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<BlockHash>(),
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<BlockHash>(),
        );

        bytes
    }
}
use std::time::SystemTime;

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: BlockTransactions,
}

#[derive(Debug)]
pub struct BlockHeader {
    pub id: u64,               // Transaction Id.
    pub prev_block_hash: u64,  // Hash of previous block header.
    pub merkle_root_hash: u64, // Merkle root hash.
    pub time: u64,             // Unix epoch timestamp.
    pub n_bits: u8,            // Encoded difficulty.
    pub nonce: u64,
}

#[derive(Debug)]
pub struct BlockTransactions {}

impl Block {}

pub fn new(id: u64, prev_block_hash: u64, merkle_root_hash: u64, n_bits: u8, nonce: u64) -> Block {
    Block {
        header: BlockHeader {
            id,
            prev_block_hash,
            merkle_root_hash,
            time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            n_bits,
            nonce,
        },
        transactions: BlockTransactions {},
    }
}

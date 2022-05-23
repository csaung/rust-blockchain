mod block;
use block::Block;

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn add_block(&mut self, block: Block) {
        if self.block_valid(&block) {
            self.blocks.push(block);
        }
    }

    fn block_valid(&self, block: &Block) -> bool {
        let last_block = self.blocks.last().unwrap();
        if last_block.header.merkle_root_hash != block.header.prev_block_hash {
            return false;
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain { blocks: Vec::new() };

    // First record.
    blockchain.blocks.push(block::new(0, 0, 0, 0, 0));

    for _i in 0..10 {
        let new_block = block::new(0, 0, 0, 0, 0);
        blockchain.add_block(new_block);
        println!("{:#?}", blockchain);
    }
}

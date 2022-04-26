pub mod block;
use crate::block::Block;


fn main() {

    let mut blockchain = vec![];

    /// Genesis block
    let genesis_block: Block = Block::new(
        0, 
        String::from("816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7"), 
        String::from("0"), 
        1465154705, 
        String::from("my genesis block!!"),
    );

    blockchain.push(&genesis_block);

    let hash = genesis_block.calculate_hash(0, String::from("previous hash"), 0, String::from("data"));
    println!("hash::=> {:?}", hash);
    println!("genesis block::=> {:?}", genesis_block);
    println!("blockchain::=> {:?}", blockchain);
    // println!("HHHHHHHHHHHHHHHHHHHHHHHHHH");
}

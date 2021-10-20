mod blockchain;
use actix_web::{web, App, HttpServer, Result};


fn main() {
    let mut blockchain = blockchain::Blockchain::new(blockchain::Blockchain:: DEFAULT_MAX_BLOCK_SIZE);
    println!("Hello, world!");
    println!("Max block size is {}", blockchain.max_block_size);
    println!("{:?}", blockchain.last_block().data);
}

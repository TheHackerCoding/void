mod blockchain;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    println!("Hello, world!");
    println!("{:?}", blockchain.lastBlock().data);
}

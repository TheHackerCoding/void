use std::collections::HashSet;



pub struct BlockData {
  name: String,
  data: String,
  mimetype: String
}

pub struct Block {
  index: i64,
  hash: String,
  timestamp: i64,
  data: Vec<BlockData>,
}

pub struct Blockchain {
  chain: Vec<Block>,
  currentData: Vec<BlockData>,
  nodes: HashSet<String>
}

impl Blockchain {
  pub fn new() -> Blockchain {
    Blockchain {
      chain: Vec::new(),
      currentData: Vec::new(),
      nodes: HashSet::new()
    }
  }

  pub fn newData(&mut self, x: Block) -> i64 {
    self.chain.push(x);
    return self.lastBlock().index;
  }

  pub fn lastBlock(&mut self) -> Block {
    return self.chain[0];
  }
}

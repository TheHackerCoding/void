#![allow(non_camel_case_types)]
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use sha256::digest;


pub fn timestamp() -> i64 {
  return Utc::now().timestamp();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockData {
  pub name: String,
  pub hash: String,
  pub data: String,
  pub mimetype: String
}

pub struct NewBlockOptions {
  pub proof: i64,
  pub previous_hash: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
  pub index: i64,
  pub hash: String,
  pub timestamp: i64,
  pub data: Vec<BlockData>,
  pub previous_hash: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
  pub chain: Vec<Block>,
  pub current_data: Vec<BlockData>,
  pub nodes: HashSet<String>,
  pub MAX_BLOCK_SIZE: i64,
}

impl Blockchain {
  pub const DEFAULT_MAX_BLOCK_SIZE: i64 = 100;

  pub fn hash(block: Block) -> String {
    return digest(serde_json::to_string(&block).unwrap());
  }

  pub fn new(x: i64) -> Blockchain {
    let mut data = Blockchain {
      chain: Vec::new(),
      current_data: Vec::new(),
      nodes: HashSet::new(),
      MAX_BLOCK_SIZE: x
    };
    data.new_block(NewBlockOptions {
      proof: 734,
      previous_hash: Some("g3n3s1s_b10ck".to_string())
    });
    return data;
  }

  pub fn new_data(&mut self, x: Block) -> i64 {
    self.chain.push(x);
    return self.last_block().index;
  }

  pub fn last_block(&self) -> &Block {
    return self.chain.last().unwrap();
  }

  pub fn valid_proof(&self, last_proof: i64, proof: i64) -> bool {
    let guess = format!("{}{}", last_proof, proof);
    let guess_hash = digest(guess);
    return &guess_hash[guess_hash.len()-4..] == "0000";
  }

  pub fn new_block(&mut self, option: NewBlockOptions) -> Block {
    let uhh: String = match option.previous_hash {
      Some(ans) => ans,
      None => self.last_block().hash.clone()
    };
    let mut _block = Block {
      index: self.chain.len() as i64,
      hash: "".to_string(),
      timestamp: timestamp(),
      data: self.current_data.clone(),
      previous_hash: uhh
    };
    // _block.hash = digest(String::from("{}", _block.index.to_string() + &_block.previousHash + &serde_json::to_string(&_block).unwrap()));
    _block.hash = digest(format!("{}{}{}", _block.index, _block.previous_hash, serde_json::to_string(&_block).unwrap()));
    self.current_data.clear();
    let __block = _block.clone();
    self.chain.push(__block);
    _block
  }
  pub fn proof_of_work(&self, last_proof: i64) -> i64 {
    let mut proof: i64 = 0;
    while self.valid_proof(last_proof, proof) == false {
      proof += 1;
    }
    return proof;
  }

  pub fn register_node(&self, addr: String) {
    
  }
}
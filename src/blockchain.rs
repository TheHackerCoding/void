#![allow(non_camel_case_types)]
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use sha256::digest;


fn timestamp() -> i64 {
  return Utc::now().timestamp();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockData {
  pub name: String,
  pub data: String,
  pub mimetype: String
}

pub struct NewBlockOptions {
  pub proof: i64,
  pub previousHash: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
  pub index: i64,
  pub hash: String,
  pub timestamp: i64,
  pub data: Vec<BlockData>,
  pub previousHash: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
  pub chain: Vec<Block>,
  pub currentData: Vec<BlockData>,
  pub nodes: HashSet<String>
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

  pub fn lastBlock(&self) -> &Block {
    return self.chain.last().unwrap();
  }

  pub fn newBlock(&mut self, option: NewBlockOptions) -> Block {
    let uhh: String = match option.previousHash {
      Some(ans) => ans,
      None => self.lastBlock().hash.clone()
    };
    let mut _block = Block {
      index: self.chain.len() as i64,
      hash: "".to_string(),
      timestamp: timestamp(),
      data: self.currentData.clone(),
      previousHash: uhh
    };
    // _block.hash = digest(String::from("{}", _block.index.to_string() + &_block.previousHash + &serde_json::to_string(&_block).unwrap()));
    _block.hash = digest(format!("{}{}{}", _block.index, _block.previousHash, serde_json::to_string(&_block).unwrap()));
    self.currentData.clear();
    let __block = _block.clone();
    self.chain.push(__block);
    _block
  }
}

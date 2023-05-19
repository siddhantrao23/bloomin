extern crate core;
extern crate bit_vec;

use std::cmp::{min, max};

use std::collections::hash_map::RandomState;
use std::hash::Hash;
use bit_vec::BitVec;

use super::hashing::HashIter;

pub struct BloomFilter {
    bits: BitVec,
    num_hash_fn: u32,
    hash_builder_one: RandomState,
    hash_builder_two: RandomState,
}

impl BloomFilter {
    pub fn new(false_pos_rate: f32, expected_num_items: u32) -> BloomFilter {
        let no_bits = estimate_bits(false_pos_rate, expected_num_items);
        BloomFilter {
            bits: BitVec::from_elem(no_bits, false),
            num_hash_fn: estimate_hash(
                no_bits,
                expected_num_items,
            ),
            hash_builder_one: RandomState::new(),
            hash_builder_two: RandomState::new(),
        }
    }

    pub fn insert<T: Hash>(& mut self, value: &T) {
        for h in HashIter::new(value,
                                self.num_hash_fn,
                                &self.hash_builder_one,
                                &self.hash_builder_two ) {
          let idx = (h % self.bits.len() as u64) as usize;
          println!("index to set {}", idx);
          self.bits.set(idx, true);
        }
    }

    pub fn contains<T: Hash>(& mut self, value: &T) -> bool {
      for h in HashIter::new(value,
                                self.num_hash_fn,
                                &self.hash_builder_one,
                                &self.hash_builder_two ) {
        let idx = (h % self.bits.len() as u64) as usize;
        println!("index to check {}", idx);
        match self.bits.get(idx) {
          Some(b) => {
            if !b {
              return false;
            }
          }
          None => { panic!("Hash Mod Failed"); }
        }
      }
      true
    }

    fn clear(&mut self) {
        self.bits.clear();
    }
}

pub fn estimate_bits(false_pos_rate: f32, expected_num_items: u32) -> usize {
    let ln22 = core::f32::consts::LN_2 * core::f32::consts::LN_2;
    (expected_num_items as f32 * ((1.0 / false_pos_rate).ln() / ln22)).round() as usize
}

pub fn estimate_hash(no_bits: usize, expected_num_items: u32) -> u32 {
    let optimal_k = (no_bits as f32 / expected_num_items as f32 * core::f32::consts::LN_2).round() as u32;
    min(200, max(2, optimal_k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut filter = BloomFilter::new(0.01, 100);
        filter.insert(&10);
        assert!(filter.contains(&10));
        assert!(!filter.contains(&20));
        filter.clear();

        assert!(!filter.contains(&10));
    }
}
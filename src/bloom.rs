extern crate core;
extern crate bit_vec;

use std::cmp::{min, max};
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{Hasher, Hash, BuildHasher};
use bit_vec::BitVec;

use super::hashing::HashIter;

pub struct BloomFilter {
    bits: BitVec,
    num_hash_fn: u32,
}

impl BloomFilter {
    pub fn new(false_pos_rate: f32, expected_num_items: u32) -> BloomFilter {
        let no_bits = estimate_bits(false_pos_rate, expected_num_items);
        BloomFilter {
            bits: BitVec::from_elem(no_bits, false),
            num_hash_fn: estimate_hash(
                no_bits,
                expected_num_items
            ),
        }
    }

    pub fn insert<T: Hash>(& mut self, value: &T) {
        for h in HashIter::new(value,
                               self.num_hash_fn) {
          let idx = (h % self.bits.len() as u64) as usize;
          self.bits.set(idx, true);
        }
    }

    pub fn contains<T: Hash>(& mut self, value: &T) -> bool {
      for h in HashIter::new(value,
                                  self.num_hash_fn) {
        let idx = (h % self.bits.len() as u64) as usize;
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

   // fn double_hash<T: Hash>(&mut self, value: &T, hash_fn_idx: usize) -> u64 {
   //     value.hash(&mut self.hasher_one);
   //     value.hash(&mut self.hasher_two);
   //     
   //     let h1 = self.hasher_one.finish();
   //     let h2 = self.hasher_two.finish();

   //     let len = self.bits.len() as u64;
   //     let h = match h2.checked_mul(hash_fn_idx as u64) {
   //         Some(mul_result) => h1.wrapping_add(mul_result),
   //         None => h1,            
   //     };
   //     println!("hash is {} {} with idx  {} = {}", h1, h2, hash_fn_idx, h);
   //     h % len
   // }
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
    fn it_works() {
        let mut filter = BloomFilter::new(0.01, 100);
        filter.insert(&10);
        // TODO: for some reason this is false check
        println!["{}", filter.contains(&10)];
        println!["{}", filter.contains(&20)];
    }
}
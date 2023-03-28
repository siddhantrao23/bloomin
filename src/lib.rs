extern crate core;
extern crate bit_vec;

use std::cmp::{min, max};
use bit_vec::BitVec;

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
            )
        }
    }

    pub fn insert<T>(& mut self, value: &T) -> bool {
        false
    }

    pub fn contains<T>(& mut self, value: &T) -> bool {
        false
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
    fn it_works() {
        let mut filter = BloomFilter::new(0.01, 100);
        filter.insert(&10);
        filter.contains(&10);
        filter.contains(&20);
    }
}

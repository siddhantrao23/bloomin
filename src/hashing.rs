use std::{hash::{BuildHasher, Hash, Hasher}};

pub struct HashIter {
  h1: u64,
  h2: u64,
  i: u32,
  count: u32,
}

impl Iterator for HashIter {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {
      if self.i == self.count {
        return None
      }
      let h = match self.h2.checked_mul(self.i as u64) {
        Some(mul_result) => self.h1.wrapping_add(mul_result),
        None => self.h1,            
      };
      self.i += 1;
      Some(h)
  }
}

impl HashIter {
  pub fn new<T: Hash, R: BuildHasher>(value: &T, num_hash_fn: u32, build_hasher_one: &R, build_hasher_two: &R) -> HashIter {
    let mut hasher_one = build_hasher_one.build_hasher();
    let mut hasher_two = build_hasher_two.build_hasher();
    
    value.hash(&mut hasher_one);
    value.hash(&mut hasher_two);
        
    let h1 = hasher_one.finish();
    let h2 = hasher_two.finish();

    HashIter {
      h1,
      h2,
      i: 0,
      count: num_hash_fn,
    }
  }
}
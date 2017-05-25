extern crate rand;
extern crate siphasher;

mod lock_free_array;

use self::lock_free_array::LockFreeBitArray;

use std::f64;
use std::cmp;
use std::hash::{Hash, Hasher};
use siphasher::sip::SipHasher13;

pub struct BloomFilter {
    bitarray: LockFreeBitArray,
    num_of_bits: usize,
    num_of_functions: u32,
    hash_funs: [SipHasher13; 2]
}

impl BloomFilter {
    pub fn create(expected_insertions: i64, fpp: f64) -> BloomFilter {
        let num_of_bits = BloomFilter::optimal_num_of_bits(expected_insertions, fpp);
        let num_of_functions = BloomFilter::optimal_num_of_functions(expected_insertions, num_of_bits);

        BloomFilter {
            bitarray: LockFreeBitArray::new(num_of_bits),
            num_of_bits: num_of_bits,
            num_of_functions: num_of_functions,
            hash_funs: [BloomFilter::sip_new(), BloomFilter::sip_new()]
        }
    }

    pub fn set<T>(&mut self, item: T) -> bool
    where T: Hash {
        let mut hashes = [0u64, 0u64];
        let mut bit_changed = false;
        for i in 0..self.num_of_functions {
            let combined_hash = self.bloom_hash(&mut hashes, &item, i);

            bit_changed |= self.bitarray.set(combined_hash as usize % self.num_of_bits);
        }

        bit_changed
    }

    fn bloom_hash<T>(&self, hashes: &mut [u64; 2], item: &T, index: u32) -> u64
    where T: Hash{
        if index < 2 {
            let hasher = &mut self.hash_funs[(index as usize)].clone();
            item.hash(hasher);
            hashes[(index as usize)] = hasher.finish();
            hashes[(index as usize)]
        } else {
            hashes[0].wrapping_add((index as u64).wrapping_mul(hashes[1]))
        }
    }

    pub fn might_contain<T>(&self, item: T) -> bool
    where T: Hash {
        let mut hashes = [0u64, 0u64];
        for i in 0..self.num_of_functions {
            let combined_hash = self.bloom_hash(&mut hashes, &item, i);

            if !self.bitarray.get(combined_hash as usize % self.num_of_bits){
                return false;
            }
        }

        true
    }

    fn optimal_num_of_bits(expected_insertions: i64, fpp: f64) -> usize {
        assert!(expected_insertions > 0);
        assert!(fpp > 0.0 && fpp < 1.0);

        let ln2 = f64::consts::LN_2;
        ((- (expected_insertions as f64) * f64::ln(fpp)) / (ln2 * ln2)) as usize
    }

    fn optimal_num_of_functions(expected_insertions: i64, num_of_bits: usize) -> u32 {
        assert!(expected_insertions > 0);
        assert!(num_of_bits > 0);

        let m = expected_insertions as f64;
        let n = num_of_bits as f64;
        let functions = (m / n * f64::consts::LN_2).ceil() as u32;
        cmp::max(functions, 1u32)
    }

    fn sip_new() -> SipHasher13 {
        let mut rng = rand::thread_rng();
        SipHasher13::new_with_keys(rand::Rand::rand(&mut rng),
                                 rand::Rand::rand(&mut rng))
    }
}

#[test]
fn test_set(){
    let mut f1 = BloomFilter::create(100, 0.01);
    assert!(f1.set("Has"));
    assert!(f1.might_contain("Has"));
    assert!(!f1.set("Has"));
}
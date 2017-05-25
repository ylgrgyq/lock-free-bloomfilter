#![allow(dead_code)]

use std::sync::atomic::{AtomicUsize, Ordering, AtomicU64};

pub struct LockFreeBitArray {
    bit_count: AtomicUsize,
    bits: Vec<AtomicU64>
}

impl LockFreeBitArray {
    #[allow(unused_variables)]
    pub fn new(bits: usize) -> LockFreeBitArray {
        let round_size = (bits as f64 / 64.0f64).ceil() as usize;
        let mut bits = Vec::new();
        for i in 0..round_size {
            bits.push(AtomicU64::new(0));
        }
        LockFreeBitArray {
            bits: bits,
            bit_count: AtomicUsize::new(0)
        }
    }

    pub fn set(&mut self, bit_index: usize) -> bool {
        if self.get(bit_index) {
            false
        } else {
            let long_index = bit_index / 64;
            let mask = 1u64.wrapping_shl(bit_index as u32);

            loop {
                let old_value = self.bits[long_index].load(Ordering::Relaxed);
                let new_value = old_value | mask;

                if old_value == new_value {
                    return false;
                } else if self.bits[long_index].compare_and_swap(old_value, new_value, Ordering::Relaxed) == old_value {
                    self.bit_count.fetch_add(1, Ordering::Relaxed);
                    break;
                }
            }

            true
        }
    }

    pub fn get(&self, bit_index: usize) -> bool {
        let long_index = (bit_index / 64) as usize;
        self.bits[long_index].load(Ordering::Relaxed) & 1u64.wrapping_shl(bit_index as u32) != 0
    }

    pub fn get_bit_count(&self) -> usize {
        self.bit_count.load(Ordering::Relaxed)
    }

    pub fn get_bit_size(&self) -> u64 {
        self.bits.len() as u64 * 64u64
    }
}


#[test]
fn test_lock_free_bit_array(){
    let bits_size = 7282;
    let mut bit_array = LockFreeBitArray::new(bits_size);
    assert_eq!(bit_array.get_bit_count(), 0);
    println!("{:?}", bit_array.get_bit_size());
    assert_eq!(bit_array.get_bit_size(), (64 * (bits_size as f64 / 64.0f64).ceil() as u64));
    for i in 0..bit_array.get_bit_size() as usize {
        bit_array.set(i);
        assert!(bit_array.get(i));
        for j in (i + 1)..bit_array.get_bit_size() as usize {
            //                println!("{} {}",i, bit_array.get(j));
            assert!(bit_array.get(j) == false);
        }
    }
}
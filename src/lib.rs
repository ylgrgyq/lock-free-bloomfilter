#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(integer_atomics)]

#![crate_name="lock_free_bloomfilter"]
#![crate_type = "rlib"]

extern crate rand;
extern crate siphasher;

pub mod bloomfilter;

#[cfg(test)]
mod test;
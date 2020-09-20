#![no_std]
extern crate alloc;

mod shuffle;
pub use shuffle::{FisherYates, SatCycles};

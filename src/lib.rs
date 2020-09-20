#![no_std]
extern crate alloc;

mod multi;
mod shuffle;
pub use shuffle::{
    FisherYates,
    SatCycles,
};

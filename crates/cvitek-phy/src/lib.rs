#![no_std]
#![allow(dead_code)]

extern crate alloc;
#[macro_use]
extern crate log;
mod cvitek_defs;
mod cvitek_dev;
pub use cvitek_dev::CvitekPhyDevice;
pub use cvitek_dev::CvitekPhyTraits;
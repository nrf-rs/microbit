#![no_std]
#![cfg_attr(feature="rt",feature(global_asm))]
#![cfg_attr(feature="rt",feature(use_extern_macros))]
#![cfg_attr(feature="rt",feature(used))]
#![feature(const_fn)]
#![allow(non_camel_case_types)]

pub extern crate nrf51_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate bare_metal;
extern crate vcell;

extern crate nrf51;

pub use nrf51::*;
pub use nrf51::interrupt::*;
pub use cortex_m_rt::*;

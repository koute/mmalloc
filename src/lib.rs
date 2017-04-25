#![cfg_attr(feature = "allocator", allocator)]
#![feature(allocator)]
#![feature(linkage)]
#![no_std]

#[macro_use]
extern crate sc;

mod allocator;
pub use allocator::*;

#[cfg(not(test))]
#[cfg(feature = "allocator")]
mod symbols;

#[cfg(not(test))]
#[cfg(feature = "allocator")]
pub use symbols::*;

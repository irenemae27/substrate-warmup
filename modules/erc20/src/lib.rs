#![cfg_attr(not(feature = "std"), no_std)]

mod erc20;

pub use erc20::{Event, Module, Trait};
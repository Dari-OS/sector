//! A stateful vector that provides different memory managment behaviours
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub mod components;
mod sector;
pub mod states;

pub use sector::Sector;

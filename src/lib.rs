//! A stateful vector that provides different memory managment behaviours
//!

#![no_std]
pub mod components;
mod sector;
pub mod states;

pub use sector::Sector;

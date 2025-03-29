//! # Stateful Vector (`Sector`)
//!
//! `Sector` is a versatile vector-like container that provides different memory management behaviors
//! through a **stateful type system**. It allows precise control over allocation, growth, and shrink behavior
//! based on the chosen **state**.
//!
//! ## Features
//! - Multiple **memory management strategies** using distinct states.
//! - **Custom allocation policies**, including dynamic resizing, fixed capacity, manual control, and tight packing.
//! - **Safe and efficient transitions** between states.
//! - Works in **no_std** by default.
//!
//! ## States Overview
//!
//! The `Sector` type is parameterized over a **state**, which defines its behavior:
//!
//! - [`Normal`](crate::states::Normal) – Behaves like a standard `Vec<T>`, growing dynamically as needed.
//! - [`Dynamic`](crate::states::Dynamic) – Allows manual resizing, but still provides automatic growth.
//! - [`Fixed`](crate::states::Fixed) – Has a fixed capacity that cannot grow dynamically.
//! - [`Tight`](crate::states::Tight) – Optimized for minimal memory usage with exact fits.
//! - [`Locked`](crate::states::Locked) – Prevents modification, acting as a frozen buffer.
//! - [`Manual`](crate::states::Manual) – Provides explicit manual control over memory allocation.
//!
//! ## Example Usage
//!
//! ```rust
//! use sector::Sector;
//! use sector::states::Normal;
//!
//! let mut vec: Sector<Normal, i32> = Sector::new();
//! vec.push(10);
//! vec.push(20);
//! assert_eq!(vec.pop(), Some(20));
//! ```
//!
//! ### State Transitions
//!
//! You can **convert between states** using transition methods:
//!
//! ```rust
//! use sector::Sector;
//! use sector::states::{Normal, Fixed};
//!
//! let normal_vec: Sector<Normal, i32> = Sector::new();
//! let fixed_vec: Sector<Fixed, i32> = normal_vec.to_fixed();
//! ```
//!
//! ## `no_std` Compatibility
//!
//! The crate supports by default no_std apps. So just add it using:
//!
//! ```toml
//! [dependencies]
//! sector = { version = "0.1"}
//! ```
//!
//! ## Modules
//! - [`components`](crate::components) – Internal traits defining vector operations.
//! - [`sector`](crate::sector::Sector) – Core implementation of `Sector`.
//! - [`states`](crate::states) – Definitions of different memory management states.
//!
//! ---
//!
//! **License:** MIT & APACHE 2.0
//!
//! **Repository:** [GitHub](https://github.com/Dari-OS/sector)

#![cfg_attr(not(feature = "std"), no_std)]

pub mod components;
mod sector;
pub mod states;

pub use sector::Sector;

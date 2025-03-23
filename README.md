# sector

A **stateful vector implementation** that provides customizable memory management behaviors through Rust traits and state machines.

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
[![MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache 2.0](https://img.shields.io/badge/License-APACHE%202.0-blue.svg)](./LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Dari-OS/sector/.github%2Fworkflows%2Frust.yml
)](https://github.com/Dari-OS/sector/actions)

---

## Table of Contents

- [About](#about)
- [Features](#features)
- [States](#states)
- [Usage](#usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [Licenses](#license)

---

## About

**sector** is a Rust library that provides a stateful vector structure (`Sector<State, T>`) with choosable memory allocation strategies.  
Unlike `Vec<T>`, it allows developers to control how memory grows/shrinks.

> [!NOTE]
> This library is under **active development**. Expect changes and optimizations.

## Features

- [x] **Stateful Memory Management** – Control memory allocation behavior dynamically.

- [x] **Lightweight & Fast** – Minimal overhead while allowing full customization.

- [ ] **No Std Support (Planned)** – Future support for `#![no_std]` environments.

## States

Sector has 6 different states:

- [`Normal`] Acts like the normal `std::vec::Vec<T>`.
- [`Dynamic`] Grows the internal capacity by a factor of 2. Shrinks to 3/4 of the original capacity
- [`Fixed`] Is not able to grow nor shrink. Returns `false` if the capacity is full and you try to add elements.
- [`Locked`] Does not allow to add or remove elements, regardless of the inner capacity.
- [`Manual`] Requires you to grow and shrink the inner capacity manually.
- [`Tight`] The inner capacity is exactly as large as the length

> [!WARNING]
> Be careful!
> [**Zero Sized**](https://doc.rust-lang.org/nomicon/exotic-sizes.html) Types are treated differently by each state.
> Refer to the specific documentation of each state

[`Normal`]: ./src/states/normal.rs#L7
[`Dynamic`]: ./src/states/dynamic.rs#L7
[`Fixed`]: ./src/states/fixed.rs#L14
[`Locked`]: ./src/states/locked.rs#L7
[`Manual`]: ./src/states/manual.rs#L7
[`Tight`]: ./src/states/tight.rs#L7

## Usage

Add `sector` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
sector = "0.1"
```

### Basic Example

```rust
use sector::{states::Normal, Sector};

fn main() {
    let mut sec: Sector<Normal, _> = Sector::new();
    sec.push(10);
    sec.push(20);

    // Access elements
    println!("First element: {:?}", sec.get(0));
}
```

## Documentation

Generate docs locally:

```sh
cargo doc --open
```

Or visit the documentation online:

[docs.rs](https://docs.rs/sector/latest/sector/)

## Contributing

Contributions are welcome!

## Roadmap

- [x] Basic stateful vector implementation
- [x] Capacity management
- [ ] `no_std` support
- [ ] Benchmarks & optimizations

## License

This project is dual licensed:

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

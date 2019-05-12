// **************************************************************************
// Copyright (c) 2019 Roland Ruckerbauer All Rights Reserved.
//
// This file is part of hidapi-rs
// **************************************************************************

//! This crate provides a rust abstraction over the features of the C library
//! hidapi by [signal11](https://github.com/signal11/hidapi).
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/hidapi) and can be
//! used by adding `hidapi` to the dependencies in your project's `Cargo.toml`.
//!
//! # Example
//!
//! **TODO: Write new example**

#[macro_use]
extern crate failure_derive;
extern crate failure;
#[cfg(feature = "linux-rust-hidraw")]
#[macro_use]
pub extern crate nix;

mod backend;
mod error;
mod parser;

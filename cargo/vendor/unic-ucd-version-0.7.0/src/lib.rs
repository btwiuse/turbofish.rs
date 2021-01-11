// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]
#![forbid(unsafe_code, missing_docs, unconditional_recursion)]

//! # UNIC — UCD — Core
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Core create indicating the version of Unicode Character Database.

extern crate unic_common;

pub use unic_common::version::UnicodeVersion;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

mod unicode_version;
pub use unicode_version::UNICODE_VERSION;

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
#![forbid(unsafe_code, unconditional_recursion)]
#![deny(missing_docs)]

//! # UNIC — UCD — Segmentation Properties"
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor for Text Segmentation character properties from Unicode Character Database (UCD).

#[macro_use]
extern crate unic_char_property;

#[macro_use]
extern crate unic_char_range;

extern crate unic_ucd_version;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub mod grapheme_cluster_break;
pub use grapheme_cluster_break::GraphemeClusterBreak;

pub mod sentence_break;
pub use sentence_break::SentenceBreak;

pub mod word_break;
pub use word_break::WordBreak;

use unic_ucd_version::UnicodeVersion;

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");

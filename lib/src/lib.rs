#![feature(option_replace)]
#![deny(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]
//! Documentation
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate serde_url_params;

/// Root URI of the Unsplash API.
pub const API_URL: &'static str = "https://api.unsplash.com/";

/// Endpoints of the Unsplash API.
pub mod endpoint;

/// Errors that can be raised.
pub mod error;

pub use endpoint::photos::Photos;

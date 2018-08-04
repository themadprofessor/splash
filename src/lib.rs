#![feature(option_replace)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
extern crate serde_json;
extern crate serde_url_params;
extern crate serde;
extern crate itertools;
extern crate chrono;
extern crate hyper;
extern crate futures;

use hyper::Uri;

pub const API_URL: &'static str = "https://api.unsplash.com/";

pub mod endpoint;
pub mod error;

pub use endpoint::photos::Photos;

extern crate hyper_rustls;
extern crate splash_rs;
extern crate hyper;
extern crate failure;

use hyper::rt::Future;
use failure::Fail;

fn main() {
    let client = hyper::Client::builder().build(hyper_rustls::HttpsConnector::new(2));
    let fut = splash_rs::Photos::random().get(&client, "87e5c4f5e3db3a47a9cbc9abefbd196e3f7aa9a7cccc1ca4751008ec796e4eb7")
        .map_err(|e| eprintln!("{:?}", e))
        .map(|p| println!("{:?}", p));
    hyper::rt::run(fut);
}

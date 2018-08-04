pub mod photos;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use failure::Fail;
use hyper::{Uri, Client, Request};
use hyper::client::connect::Connect;
use futures::{Future, Stream};

use std::error::Error as StdError;
use std::fmt;

use error::*;

pub trait ToQuery {
    fn to_query(&self) -> String;
}

impl <T> ToQuery for T where T: Serialize {
    fn to_query(&self) -> String {
        let s = ::serde_url_params::to_string(&self).unwrap();
        if s.is_empty() {
            String::new()
        } else {
            let mut string = String::with_capacity(s.len() + 1);
            string.push('?');
            string += &s;
            string
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Errors(Vec<String>);

impl StdError for Errors {}

impl fmt::Display for Errors {
    fn fmt(&self, f: & mut fmt::Formatter) -> Result<(), fmt::Error> {
        for e in self.0.iter() {
            f.write_str(&e)?;
        }
        Ok(())
    }
}

fn parse_err<T>(v: Vec<u8>) -> ::futures::future::FutureResult<T, Error> where T: DeserializeOwned {
    match ::serde_json::from_slice::<::endpoint::Errors>(&v) {
        Ok(j) => ::futures::future::err(Error::from(j.context(ErrorKind::Photos))),
        Err(e) => ::futures::future::err(Error::from(e.context(ErrorKind::Photos)))
    }
}

fn parse_data<T>(v: Vec<u8>) -> ::futures::future::FutureResult<T, Error> where T: DeserializeOwned {
    match ::serde_json::from_slice::<T>(&v){
        Ok(j) => ::futures::future::ok(j),
        Err(e) => ::futures::future::err(Error::from(e.context(ErrorKind::Photos)))
    }
}

fn get<T, C, R>(query: T, client: &Client<C>, access_key: &str, uri: Uri, context: ErrorKind) -> impl Future<Item=R, Error=Error> where T: Serialize, C: Connect + 'static, R: DeserializeOwned {
    let request = Request::get(format!("{}{}", uri, query.to_query()))
        .header("Accept", "application/json")
        .header("Accept-Version", "v1")
        .header("Authorization", format!("Client-ID {}", access_key).as_str())
        .body(::hyper::Body::empty())
        .unwrap();
    client.request(request)
        .map_err(move |e| Error::from(e.context(context)))
        .and_then(|res| {
            let parser = if res.status().is_success() {
                parse_data::<R>
            } else {
                parse_err
            };

            res.into_body().map_err(|e| Error::from(e.context(ErrorKind::Photos)))
                .fold(Vec::new(), fold)
                .and_then(parser)
        })
}

fn fold(mut v: Vec<u8>, chunk: ::hyper::Chunk) -> ::futures::future::Ok<Vec<u8>, Error> {
    v.extend(&chunk[..]);
    ::futures::future::ok(v)
}

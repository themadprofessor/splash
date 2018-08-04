use hyper::{Client, Uri};
use hyper::client::connect::Connect;
use hyper::Request;
use hyper::rt::{Future, Stream};
use failure::{ResultExt, Fail};
use serde::Deserialize;

use super::{Photo, Order};
use error::*;

lazy_static!(
    pub static ref LIST_URI: Uri = format!("{}{}", ::API_URL, "photos").parse().unwrap();
);


#[derive(Debug, Default, Serialize)]
pub struct List {
    page: Option<usize>,
    per_page: Option<usize>,
    order_by: Option<Order>
}

impl List {
    pub fn page(mut self, page: usize) -> Self {
        assert_eq!(0, page, "Pages start a 1, not 0!");
        self.page.replace(page);
        self
    }

    pub fn per_page(mut self, per_page: usize) -> Self {
        assert_eq!(0, per_page, "Cannot have 0 elements per page!");
        self.per_page.replace(per_page);
        self
    }

    pub fn order_by(mut self, order_by: Order) -> Self {
        self.order_by.replace(order_by);
        self
    }

    pub fn get<C>(self, client: &Client<C>, access_key: &str) -> impl Future<Item=Vec<Photo>, Error=Error> where C: Connect + 'static {
        let request = Request::get(format!("{}{}", LIST_URI.to_string(), serialize_query(&self)))
            .header("Accept", "application/json")
            .header("Accept-Version", "v1")
            .header("Authorization", format!("Client-ID {}", access_key).as_str())
            .body(::hyper::Body::empty())
            .unwrap();
        client.request(request)
            .map_err(error_ctx)
            .and_then(|res| {
                let parser = if res.status().is_success() {
                    parse_photos
                } else {
                    ::endpoint::parse_err
                };

                res.into_body().map_err(error_ctx).fold(Vec::new(), fold)
                    .and_then(parser)
            })
    }
}

fn fold(mut v: Vec<u8>, chunk: ::hyper::Chunk) -> ::futures::future::Ok<Vec<u8>, Error> {
    v.extend(&chunk[..]);
    ::futures::future::ok(v)
}

fn error_ctx<T: Fail>(e: T) -> Error {
    Error::from(e.context(ErrorKind::Photos))
}

fn parse_photos(v: Vec<u8>) -> ::futures::future::FutureResult<Vec<Photo>, Error> {
    match ::serde_json::from_slice::<Vec<Photo>>(&v){
        Ok(j) => ::futures::future::ok(j),
        Err(e) => ::futures::future::err(Error::from(e.context(ErrorKind::Photos)))
    }
}

fn parse_err(v: Vec<u8>) -> ::futures::future::FutureResult<Vec<Photo>, Error> {
    match ::serde_json::from_slice::<::endpoint::Errors>(&v) {
        Ok(j) => ::futures::future::err(Error::from(j.context(ErrorKind::Photos))),
        Err(e) => ::futures::future::err(Error::from(e.context(ErrorKind::Photos)))
    }
}

fn serialize_query(query: &List) -> String {
    let q = ::serde_url_params::to_string(query).unwrap();
    if q.is_empty() {
        String::new()
    } else {
        let mut s = String::with_capacity(q.len() + 1);
        s.push('?');
        s += &q;
        s
    }
}
use hyper::{Client, Uri};
use hyper::client::connect::Connect;
use hyper::Request;
use hyper::rt::{Future, Stream};
use chrono::{DateTime, FixedOffset};
use failure::{ResultExt, Fail};

use super::Order;
use error::*;

pub static LIST_URI: Uri = const_concat!(::API_URL, "photos").parse().unwrap();
pub static RANDOM_URI: Uri = const_concat!(::API_URL, "photos/random").parse().unwrap();

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Photos;

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub id: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub width: usize,
    pub height: usize,
    pub color: String,
    pub likes: usize,
    pub libed_by_user: bool,
    pub description: String,
    pub user: User,
    pub current_user_collections: Vec<Collection>,
    pub urls: Urls,
    pub links: PhotoLinks
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub protfolio_url: String,
    pub bio: String,
    pub location: String,
    pub total_likes: usize,
    pub total_photos: usize,
    pub total_collections: usize,
    pub instagram_username: Option<String>,
    pub twitter_username: Option<String>,
    pub profile_image: ProfileImages,
    pub links: UserLinks
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileImages {
    pub small: String,
    pub medium: String,
    pub large: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLinks {
    pub self_link: String, //Rename to self
    pub html: String,
    pub photos: String,
    pub likes: String,
    pub portfolio: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: usize,
    pub title: String,
    pub published_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub curated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Urls {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoLinks {
    pub self_link: String,
    pub html: String,
    pub download: String,
    pub download_locaton: String
}

#[derive(Debug, Default)]
pub struct List {
    page: Option<usize>,
    per_page: Option<usize>,
    order_by: Option<Order>
}

impl Photos {
    pub fn list() -> List {
        List::default()
    }
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
        let request = Request::get(LIST_URI.clone())
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
                    parse_err
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

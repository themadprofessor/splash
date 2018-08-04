use itertools::*;
use hyper::client::connect::Connect;
use hyper::{Client, Uri, Request};
use futures::{Future, Stream};
use failure::Fail;
use serde::de::DeserializeOwned;

use error::*;
use endpoint::ToQuery;
use super::{Orientation, Photo};

lazy_static!(
    pub static ref RANDOM_URI: Uri = format!("{}{}", ::API_URL, "photos/random").parse().unwrap();
);

#[derive(Debug, Default)]
pub struct Random {
    featured: Option<bool>,
    username: Option<String>,
    w: Option<usize>,
    h: Option<usize>,
    orientation: Option<Orientation>,
}

#[derive(Debug, Default)]
pub struct RandomCount {
    rand: Random,
    count: usize
}

#[derive(Debug, Default)]
pub struct RandomQuery {
    rand: Random,
    query: String
}

#[derive(Debug, Default)]
pub struct RandomQueryCount {
    rand: RandomQuery,
    count: usize
}

#[derive(Debug, Default)]
pub struct RandomCollection {
    rand: Random,
    collection: String
}

#[derive(Debug, Default)]
pub struct RandomCollectionCount {
    rand: RandomCollection,
    count: usize
}

#[derive(Debug, Default, Serialize)]
struct RandomSerialize {
    featured: Option<bool>,
    username: Option<String>,
    w: Option<usize>,
    h: Option<usize>,
    orientation: Option<Orientation>,
    collection: Option<String>,
    query: Option<String>
}

#[derive(Debug, Default, Serialize)]
struct RandomCountSerialize {
    featured: Option<bool>,
    username: Option<String>,
    w: Option<usize>,
    h: Option<usize>,
    orientation: Option<Orientation>,
    collection: Option<String>,
    query: Option<String>,
    count: usize
}

impl Random {
    pub fn featured(mut self, feat: bool) -> Self {
        self.featured.replace(feat);
        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.username.replace(username);
        self
    }

    pub fn w(mut self, w: usize) -> Self {
        self.w.replace(w);
        self
    }

    pub fn h(mut self, h: usize) -> Self {
        self.h.replace(h);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation.replace(orientation);
        self
    }

    pub fn query(self, query: String) -> RandomQuery {
        RandomQuery {
            rand: self,
            query
        }
    }

    pub fn collection<I>(self, collection: I) -> RandomCollection where I: IntoIterator<Item=String> {
        RandomCollection {
            rand: self,
            collection: collection.into_iter().join(","),
        }
    }

    pub fn count(self, count: usize) -> RandomCount {
        assert_ne!(count, 0, "Cannot get 0 images!");
        RandomCount {
            rand: self,
            count
        }
    }

    pub fn get<C>(self, client: &Client<C>, access_key: &str) -> impl Future<Item=Photo, Error=Error> where C: Connect +'static {
        let serial = RandomSerialize {
            featured: self.featured,
            username: self.username,
            w: self.w,
            h: self.h,
            orientation: self.orientation,
            collection: None,
            query: None,
        };
        ::endpoint::get(serial, client, access_key, RANDOM_URI.clone(), ErrorKind::Photos)
    }
}

impl RandomQuery {
    pub fn count(self, count: usize) -> RandomQueryCount {
        assert_ne!(count, 0, "Cannot get 0 images!");
        RandomQueryCount {
            rand: self,
            count
        }
    }

    pub fn get<C>(self, client: &Client<C>, access_key: &str) -> impl Future<Item=Photo, Error=Error> where C: Connect +'static {
        let serial = RandomSerialize {
            featured: self.rand.featured,
            username: self.rand.username,
            w: self.rand.w,
            h: self.rand.h,
            orientation: self.rand.orientation,
            collection: None,
            query: Some(self.query),
        };
        ::endpoint::get(serial, &client, access_key, RANDOM_URI.clone(), ErrorKind::Photos)
    }
}

impl RandomCollection {
    pub fn count(self, count: usize) -> RandomCollectionCount {
        assert_ne!(count, 0, "Cannot get 0 images!");
        RandomCollectionCount {
            rand: self,
            count
        }
    }


    pub fn get<C>(self, client: &Client<C>, access_key: &str) -> impl Future<Item=Photo, Error=Error> where C: Connect +'static {
        let serial = RandomSerialize {
            featured: self.rand.featured,
            username: self.rand.username,
            w: self.rand.w,
            h: self.rand.h,
            orientation: self.rand.orientation,
            collection: Some(self.collection),
            query: None,
        };
        ::endpoint::get(serial, client, access_key, RANDOM_URI.clone(), ErrorKind::Photos)
    }
}

impl RandomCount {
    pub fn get<C>(self, client: &Client<C>, access_key: &str) -> impl Future<Item=Vec<Photo>, Error=Error> where C: Connect +'static {
        let serial = RandomCountSerialize {
            featured: self.rand.featured,
            username: self.rand.username,
            w: self.rand.w,
            h: self.rand.h,
            orientation: self.rand.orientation,
            collection: None,
            query: None,
            count: self.count,
        };
        ::endpoint::get(serial, client, access_key, RANDOM_URI.clone(), ErrorKind::Photos)
    }
}

impl RandomQueryCount {
    pub fn get<C>(self, client: &Client<C>, access_key: &str) -> impl Future<Item=Vec<Photo>, Error=Error> where C: Connect +'static {
        let serial = RandomCountSerialize {
            featured: self.rand.rand.featured,
            username: self.rand.rand.username,
            w: self.rand.rand.w,
            h: self.rand.rand.h,
            orientation: self.rand.rand.orientation,
            collection: None,
            query: Some(self.rand.query),
            count: self.count,
        };
        ::endpoint::get(serial, client, access_key, RANDOM_URI.clone(), ErrorKind::Photos)
    }
}

impl RandomCollectionCount {
    pub fn get<C>(self, client: &Client<C>, access_key: &str) -> impl Future<Item=Vec<Photo>, Error=Error> where C: Connect +'static {
        let serial = RandomCountSerialize {
            featured: self.rand.rand.featured,
            username: self.rand.rand.username,
            w: self.rand.rand.w,
            h: self.rand.rand.h,
            orientation: self.rand.rand.orientation,
            collection: Some(self.rand.collection),
            query: None,
            count: self.count,
        };
        ::endpoint::get(serial, client, access_key, RANDOM_URI.clone(), ErrorKind::Photos)
    }
}

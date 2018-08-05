#[macro_use]
extern crate clap;
#[macro_use] extern crate log;
extern crate flexi_logger;
extern crate failure;
extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate splash_rs;
extern crate tokio;
extern crate wallpaper;

use failure::Error;
use hyper::rt::Future;
use tokio::runtime::Runtime;
use flexi_logger::Logger;

mod cli;
use cli::build_cli;

fn main() {
    if let Err(e) = run() {
        error!("{}{}",
                  e,
                  e.iter_causes().map(|err| format!("\n\tcause: {}", err)).fold(String::new(),
                                                                                |mut s, err| {
                                                                                    s += &err;
                                                                                    s
                                                                                }));
        ::std::process::exit(-1);
    }
}

fn run() -> Result<(), Error> {
    Logger::with_env().start().map_err(Error::from)?;

    let matches = build_cli().get_matches();
    let mut fut = splash_rs::Photos::random();

    if let Some(h) = matches.value_of_lossy("height") {
        fut = fut.h(h.parse().map_err(Error::from)?);
    }
    if let Some(w) = matches.value_of_lossy("width") {
        fut = fut.w(w.parse().map_err(Error::from)?);
    }
    if matches.is_present("featured") {
        fut = fut.featured(true);
    }
    if let Some(user) = matches.value_of_lossy("username") {
        fut = fut.username(user.to_string());
    }
    if let Some(orient) = matches.value_of_lossy("orientation") {
        use splash_rs::endpoint::photos::Orientation::*;
        fut = fut.orientation(match orient.as_ref() {
                                  "portrait" => Portrait,
                                  "landscape" => Landscape,
                                  "squarish" => Squarish,
                                  _ => return Err(::failure::err_msg("invalid orientation")),
                              })
    }

    let client = hyper::Client::builder().build(hyper_rustls::HttpsConnector::new(2));
    let mut runtime = Runtime::new().unwrap();
    debug!("initialised runtime");

    if let Some(query) = matches.value_of_lossy("query") {
        debug!("using query");
        trace!("query: {}", query);
        let fut =
            fut.query(query.to_string())
               .get(&client, "87e5c4f5e3db3a47a9cbc9abefbd196e3f7aa9a7cccc1ca4751008ec796e4eb7")
               .map_err(Error::from)
               .and_then(|p| match wallpaper::set_from_url(&p.urls.full) {
                             Ok(_) => ::futures::future::ok(()),
                             Err(e) => ::futures::future::err(::failure::err_msg(format!("{}", e))),
                         })
               .map_err(|e| eprintln!("{:?}", e));
        runtime.block_on(fut).map_err(|_| ::failure::err_msg("failed to wait for response"))?;
    } else if let Some(collections) = matches.values_of_lossy("collections") {
        debug!("using collections");
        trace!("collections: {:?}", collections);
        let fut =
            fut.collection(collections)
               .get(&client, "87e5c4f5e3db3a47a9cbc9abefbd196e3f7aa9a7cccc1ca4751008ec796e4eb7")
               .map_err(Error::from)
               .and_then(|p| match wallpaper::set_from_url(&p.urls.full) {
                             Ok(_) => ::futures::future::ok(()),
                             Err(e) => ::futures::future::err(::failure::err_msg(format!("{}", e))),
                         })
               .map_err(|e| eprintln!("{:?}", e));
        runtime.block_on(fut).map_err(|_| ::failure::err_msg("failed to wait for response"))?;
    } else {
        debug!("no query or collection");
        let fut =
            fut.get(&client, "87e5c4f5e3db3a47a9cbc9abefbd196e3f7aa9a7cccc1ca4751008ec796e4eb7")
               .map_err(Error::from)
               .and_then(|p| match wallpaper::set_from_url(&p.urls.full) {
                             Ok(_) => ::futures::future::ok(()),
                             Err(e) => ::futures::future::err(::failure::err_msg(format!("{}", e))),
                         })
               .map_err(|e| eprintln!("{:?}", e));
        runtime.block_on(fut).map_err(|_| ::failure::err_msg("failed to wait for response"))?;
    }

    runtime.shutdown_now().wait().map_err(|_| ::failure::err_msg("failed to shutdown tokio"))?;
    debug!("shutdown runtime");
    Ok(())
}

#[macro_use]
extern crate clap;
extern crate failure;
extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate splash_rs;
extern crate tokio;
extern crate wallpaper;

use clap::{App, Arg};
use failure::Error;
use hyper::rt::Future;
use tokio::runtime::Runtime;


fn main() {
    if let Err(e) = run() {
        eprintln!("{}{}",
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

    if let Some(query) = matches.value_of_lossy("query") {
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
    Ok(())
}

fn build_cli() -> App<'static, 'static> {
    (app_from_crate!() as App)
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .value_name("HEIGHT")
                .help("Height of image")
                .long_help("Height of image"),
        ).arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .value_name("WIDTH")
                .help("Width of image")
                .long_help("Width of image"),
        ).arg(
            Arg::with_name("featured")
                .short("f")
                .long("featured")
                .help("Only featured images")
                .long_help("Restrict images to only featured images"),
        ).arg(
            Arg::with_name("username")
                .short("u")
                .long("username")
                .takes_value(true)
                .value_name("USERNAME")
                .help("Only from username")
                .long_help("Restrict images to only images from a specific user"),
        ).arg(
            Arg::with_name("orientation")
                .short("o")
                .long("orientation")
                .takes_value(true)
                .possible_values(&["portrait", "landscape", "squarish"])
                .value_name("ORIENTATION"),
        ).arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .takes_value(true)
                .value_name("QUERY")
                .help("Query to find image in")
                .long_help("Restrict images to only images with match this query"),
        ).arg(
            Arg::with_name("collection")
                .short("c")
                .long_help("collection")
                .takes_value(true)
                .multiple(true)
                .value_name("COLLECTION")
                .help("Collection to pick from")
                .long_help(
                    "Restrict images to only images which are in any of the given collections",
                ).conflicts_with("query"),
        )
}

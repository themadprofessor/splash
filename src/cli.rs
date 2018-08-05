use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
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

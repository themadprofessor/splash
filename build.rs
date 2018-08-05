#[macro_use] extern crate clap;

use clap::Shell;
use std::path::Path;

include!("src/cli.rs");

fn main() {
    if let Some(path) = std::env::var_os("SHELL") {
        let path = Path::new(&path);
        if let Some(shell) = path.file_name() {
            match shell.to_string_lossy().parse::<Shell>() {
                Ok(s) => {
                    let mut app = build_cli();
                    app.gen_completions("splash", s, ::std::env::var("OUT_DIR").unwrap());
                },
                Err(e) => eprintln!("{}", e)
            }
        } else {
            eprintln!("Invalid shell env var");
        }
    } else {
        eprintln!("No shell env var set")
    }
}
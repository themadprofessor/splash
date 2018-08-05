# splash-rs

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

> CLI tool to set wallpaper to random photo from Unsplash written in Rust

TODO: Fill out this long description.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contribute](#contribute)
- [License](#license)

## Install

```
git clone https://github.com/themadprofessor/splash.git
cd splash
cargo install
```

## Usage

```
USAGE:
    splash [FLAGS] [OPTIONS]
FLAGS:
    -f, --featured
            Restrict images to only featured images
        --help
            Prints help information
    -V, --version
            Prints version information
OPTIONS:
    -c <COLLECTION>...
            Restrict images to only images which are in any of the given collections
    -h, --height <HEIGHT>
            Height of image
    -o, --orientation <ORIENTATION>
             [possible values: portrait, landscape, squarish]
    -q, --query <QUERY>
            Restrict images to only images with match this query
    -u, --username <USERNAME>
            Restrict images to only images from a specific user
    -w, --width <WIDTH>
            Width of image
```

## Maintainers

[@themadprofessor](https://github.com/themadprofessor)

## Contribute

PRs accepted.

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © 2018 Stuart Reilly

# splash

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

> CLI tool to set wallpaper to random photo from Unsplash written in Rust

Splash is a cross-platform CLI tool which downloads a random photo from Unsplash and sets it as the current wallpaper. 
Without any arguments, splash will not restrict which photos can be chosen from. See [usage](#usage) for more details. 

##### Platform specific information:

- Download location
    - Linux - `$XDG_CACHE_DIR` or `$HOME/.cache`
    - Mac OSX - `$HOME/Library/Caches`
    - Windows - `FOLDERID_LocalAppData` which is defaults to `%LOCALAPPDATA%` which defaults to `%USERPROFILE%\AppData\Local`
- Supported desktops
    - Windows
    - Mac OSX
    - Linux
        - GNOME
        - KDE
        - Cinnamon
        - Unity
        - Budgie
        - XFCE
        - LXDE
        - MATE
        - Deepin
        - i3
        
See [wallpaper.rs](https://github.com/reujab/wallpaper.rs) for more details on supported desktops and [dirs-rs](https://github.com/soc/dirs-rs)
for more details on download locations, specifically cache_dir.

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

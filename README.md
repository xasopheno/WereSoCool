# ***** WereSoCool __!Now In Stereo!__ ******
![Cool Tests](https://github.com/xasopheno/WereSoCool/workflows/Cool%20Tests/badge.svg)

A language for binaural, microtonal composition built in Rust.

Make cool sounds. Impress your friends/pets/plants.

![WereSoCool](https://raw.githubusercontent.com/xasopheno/weresocool/master/imgs/application.png)

WereSoCool is a programming language for composing microtonal music geometrically. This language doesn't necessarily assume
familiarity with either microtonal music or computer programming, but experience with either will certainly help. 


## Listen:

Watch/Listen to some examples form the langauage [here](https://www.weresocool.org/play/arcs).

## Make Cool Sounds:

### Macos:

The most recent version of the Macos application can be downloaded [here](https://www.weresocool.org/downloads).
Inside, you'll find a lot of cool tutorials and demos that should help you get started. If you get stuck, please do not
hesitate to reach out to me weresocool at xasopheno dot com. 

### Linux:
Currently on linux, you'll need to compile this locally. See Development.


### Windows
This does not currently work on Windows...sad panda. If you're interested in using this software on a Windows machine, please
    reach out. 


## Development:
You'll need Rust. Install it with Rustup. It's a great language.
`https://www.rust-lang.org/en-US/install.html` 

You'll need also need portaudio [portaudio](https://github.com/RustAudio/rust-portaudio) and [lame](https://lame.sourceforge.io/)


#### Macos:
`brew install portaudio pkg-config lame`

#### Arch:
`sudo pacman -S portaudio pkg-config lame`

#### Ubuntu:
`sudo apt-get portaudio pkg-config lame`




## Run:
Listen to something created with the framework

`cargo run --release --bin wsc songs/fall/table.socool`

## Usage:

```
USAGE:
    wsc [FLAGS] [filename]

FLAGS:
    -d, --doc        Prints some documentation
    -h, --help       Prints help information
    -j, --json       Prints file to .json
    -p, --print      Prints file to .wav
    -V, --version    Prints version information

ARGS:
    <filename>    filename eg: my_song.socool
```

## Test:

`./test.sh`

Working on additional documentation. :) 

Copyright (C) 2020 - Danny Meyer

This program is free software, licensed under the GPLv3 (see LICENSE).


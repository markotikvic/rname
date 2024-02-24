# rname
Command-line utility for convenient renaming of files. Upon running the current file name/location will be prefilled to input prompt.
```
$ rname file.txt 
New name: file.txt_
```

## Usage
`$ rname <file>`

## Why does this repositoy exist?
I got tired of having to type long paths twice just to change the extension or a typo in some file's name.  
For example: `$ mv /very/long/path/to/some/file.txt /very/long/path/to/some/file.log`

## Installation
- Install Rust https://www.rust-lang.org/tools/install
- Clone this repository and build it
```
$ git clone https://github.com/markotikvic/rname
$ cd rname
$ cargo build --release
```
- Add `rname` to `PATH` by either updating `$PATH` to show to `target/release` subdirectory of this project or by copying the `target/release/rname` executable to a location that's already in the `$PATH` (e.g. `/usr/local/bin`)

## License
MIT

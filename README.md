# rname
Command-line utility for convenient renaming of files. Upon running the current file name/location will be prefilled to input prompt.
```
$ rname file.txt 
New name: file.txt_
```

## Usage
`$ rname <file>`

## Why does this repository exist?
I got tired of having to type long paths twice just to change the extension or a typo in some file's name.  
For example: `$ mv /very/long/path/to/some/file.txt /very/long/path/to/some/file.log`

## Installation
- Install Rust https://www.rust-lang.org/tools/install

- Clone this repository and build it using `make`
```
$ git clone https://github.com/markotikvic/rname
$ cd rname
$ make
```
If you don't have make installed run `$ cargo build --release` instead of `$ make` for the 3rd step.

- To install to `/usr/local/bin` using `make`
```
$ sudo make install
```

### Uninstallation
**IMPORTANT:** Run this only if you've installed `rname` by running `$ sudo make install`.
```
$ sudo make uninstall
```

## License
MIT

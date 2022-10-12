## cxp

[![BUILD](https://github.com/stevelauc/cxp/workflows/Rust/badge.svg)](https://github.com/stevelauc/cxp/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/cxp.svg)](https://crates.io/crates/cxp)
[![docs.rs](https://docs.rs/cxp/badge.svg)](https://docs.rs/cxp)

Bring the Copy, Cut and Paste functionalities from your GUI file manager to your
Terminal.

> This is basically a Rust port of [xcv](https://github.com/busterc/xcv), with 
> cross-platform data directories support.

## Installation

```shell
$ cargo install cxp
```

## Usage

```
USAGE:
cxp command [operand]

    where `command` can be:
        c: copy files
        x: cut files
        p: paste files into $PWD
        l: list files
        t: list files in a tree format
        e: empty file buffer
```

```shell
$ l
Permissions Links Size User  Group Date Modified Name
.rw-r--r--@     1  18k steve steve 12 Oct 09:38  LICENSE
.rw-r--r--@     1  614 steve steve 12 Oct 09:50  README.md
.rw-r--r--@     1   15 steve steve 11 Oct 19:52  rustfmt.toml
$ cxp c README.md
$ mkdir tmp
$ cd tmp
$ cxp p
$ l
Permissions Links Size User  Group Date Modified Name
.rw-r--r--@     1  614 steve steve 12 Oct 09:51  README.md
```

## Contributing

Contributions of all forms are welcome, feel free to file an issue or make a 
pull request!

#### Test before your commit

1. Pass the tests

   ```shell
   $ cargo test
   ``` 
2. Format your code

   ```shell
   $ cargo fmt
   ```

## cxp

[![BUILD](https://github.com/stevelauc/cxp/workflows/Rust/badge.svg)](https://github.com/stevelauc/cxp/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/cxp.svg)](https://crates.io/crates/cxp)

Bring the Copy, Cut and Paste functionalities from your GUI file manager to your
Terminal.

> This is basically a Rust port of [xcv](https://github.com/busterc/xcv), with 
> cross-platform data directories support.

## Usage and Demo

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

![demo](https://user-images.githubusercontent.com/96880612/195283213-5e1f45eb-0285-4098-b00e-b6700715de0e.gif)

## Installation

```shell
$ cargo install cxp
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

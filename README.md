[![crate](https://img.shields.io/crates/v/relp-bin.svg)](https://crates.io/crates/relp-bin)

# Relp

An exact linear program solver written in Rust using the [Relp](https://github.com/vandenheuvel/relp) crate. 

## Usage

Compile the crate with:

<pre>
cargo build <b>--release</b>
</pre>

Make sure to include the `--release` flag to disable (expensive) debug assertions.

You can then solve problems with:

```
target/release/relp-bin my_program.mps
```

For more options, use the `--help` flag:

```
USAGE:
    relp-bin [FLAGS] <problem-file>

ARGS:
    <problem-file>    File containing the problem description

FLAGS:
    -h, --help           Prints help information
        --no-presolve    Disable presolving
        --no-scale       Disable scaling
    -V, --version        Prints version information
```

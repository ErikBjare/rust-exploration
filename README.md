rust-exploration
=============

My own exercises in:

 - calling Rust code from other languages.
 - writing a gravity simulator.

# Usage

This repo currently contains two experiments.

## Gravitysim

Just run with:

```
cargo run
```

You can see all the available command line arguments with:

```
cargo run -- --help
```

## FFI part

Compile the Rust code:

```
cargo build --release
```

Call the library from Python:

```
cd misc
python3 call_with_python.py
```

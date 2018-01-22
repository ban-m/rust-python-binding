## Rust-Python

This is a sandbox-repo for my [blog post](), the title of which is "Handling rust struct from python".


## Test Run

First, install Rust language and Cargo package manager.

Then, clone

```
git clone git@github.com:ban-m/rust-python-binding.git
```

then, build the crate

```
cargo build --release
```

,modify ./src/caller.py for your operating system(currently, for OSX) and finally

```
python ./src/caller.py
```

for running script on python3.6.
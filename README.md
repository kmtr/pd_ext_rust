# Pure Data externals written in Rust

# Read first here

`libpd-rs` is more actively maintained than this repository.
Look at there first.
https://github.com/alisomay/libpd-rs

# Download sources

```
git clone https://github.com/kmtr/pd_ext_rust.git
cd pdrust_example
git submodule init
git submodule update
```

# Build

## build `libpd`

Check this repository.
https://github.com/libpd/libpd

## build externals `helloworld`

```
cd helloworld
make debug
ls target/debug/helloworld.pd_darwin
```

# Next actions

- Adding more examples
- Pure Rust externals (Currntry I use C source)
- Running on multiplatform (Currentry this only available on OS X)
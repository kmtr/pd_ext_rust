# Pure Data externals written in Rust

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
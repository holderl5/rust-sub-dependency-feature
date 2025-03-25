# Dependencies Issue
Simple project to explore a dependency issue with getrandom when compiling for wasm32-unknown-unknown.  All keys and data in this repo are strictly for testing purposes.  

The issue is that getrandom is a dependency about 5 levels deep for age, but fails to build due to the wasm_js feature not being present at build time.

## Development Timeline

1. Initial version (e829211adab672ac410517a7c81e5b31f973bab8), verified working with the following:
```
$ cargo build
$ ./target/debug/rust-age-testing
```
But fails with the following:
```
$ RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --target wasm32-unknown-unknown
```
2. Next, add a direct dependency (dcc4d35297968a8dcf2d4b7b2dd84499c10f2640), no change

## Local development in docker

### Build Image
```
docker build  -t cf-rust .
```
### Run Image
```
docker run --rm -it -p 127.0.0.1:8787:8787 -v .:/home/developer/app -w /home/developer/app --name cf-rust cf-rust
```


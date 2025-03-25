# Dependencies Issue
https://stackoverflow.com/questions/79532617/can-i-enable-rust-features-on-sub-sub-sub-dependencies-especially-when-the-depe

Working with kmdreko on stackoverflow, I discovered age is using an old version of getrandom that does not support the target arch.  I cannot now easily use this project to answer the problem of what to do when we have multiple versions in the tree as moving age to the latest version would unify all of the versions.

PREVIOUSLY:

Simple project to explore a dependency issue with getrandom when compiling for wasm32-unknown-unknown.  All keys and data in this repo are strictly for testing purposes.  

The issue is that getrandom is a dependency about 5 levels deep for age, but fails to build due to the wasm_js feature not being present at build time.

## Local development in docker
There is a docker container for local development so you need not install rust locally.  wasm32-unknown-unknown is not currently in the docker build, it will need to be added with 
```
rustup target add wasm32-unknown-unknown
```


### Build Image
```
docker build  -t cf-rust .
```
### Run Image
Run this command in your cloned repo at the root level
```
docker run --rm -it -p 127.0.0.1:8787:8787 -v .:/home/developer/app -w /home/developer/app --name cf-rust cf-rust
```
### Build
```
$ RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --target wasm32-unknown-unknown
```

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
2. Add a direct dependency (dcc4d35297968a8dcf2d4b7b2dd84499c10f2640), no change
3. Move age to a separate dependencies section (d1b95c7b5d0e83acf29bee90f3f836ac335fd058), add a feature specification for getrandom there, no change



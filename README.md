# Dependencies Issue
Simple project to show how you can set feature flags for sub-sub dependencies, even when you have multiple versions of a library with different feature flag names.  The idea behind this project should be relevant for some time.  The actual demo will break at some point when the lib that uses getrandom 0.2.15 finally updates.

From here:
https://stackoverflow.com/questions/79532617/can-i-enable-rust-features-on-sub-sub-sub-dependencies-especially-when-the-depe


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
4. Per discussion with kmdreko on stackoverflow (a1197e6a2564cf1eaf5804c7eb0221bdaeebf7f0), add configs for each version of getrandom
```
age = {version = "0.11.1", features = ["armor"] }
getrandom02 = { package = "getrandom", version = "0.2.15", features = ["js"] }
getrandom03 = { package = "getrandom", version = "0.3.2", features = ["wasm_js"] }
```
I also added UUID library so you can see the two different versions



# Dependencies Issue
Simple project to explore a dependency issue with getrandom when compiling for wasm32-unknown-unknown.  All keys and data in this repo are strictly for testing purposes.  

The issue is that getrandom is a dependency about 5 levels deep of age, but fails to build due to the wasm_js feature not being present at build time.

## Solutions Reviewed
1. Create a section for age dependencies and include the 

## Local development in docker

### Build Image
```
docker build  -t cf-rust .
```
### Run Image
```
docker run --rm -it -p 127.0.0.1:8787:8787 -v .:/home/developer/app -w /home/developer/app --name cf-rust cf-rust
```


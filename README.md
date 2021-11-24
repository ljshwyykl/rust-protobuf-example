
## install 
```
brew install protobuf
cargo install protobuf-codegen
```



## Generate .rs files:
```
protoc --rust_out . message.proto
```
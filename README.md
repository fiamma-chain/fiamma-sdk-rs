
## Dependencies

In order to build `tonic >= 0.8.0`, you need the `protoc` Protocol Buffers compiler, along with Protocol Buffers resource files. See [tonic crate](https://crates.io/crates/tonic).

### Ubuntu

```shell=
sudo apt update && sudo apt upgrade -y
sudo apt install -y protobuf-compiler libprotobuf-dev
```

### Alpine Linux

```shell=
sudo apk add protoc protobuf-dev
```

### MacOS

```shell=
brew install protobuf
```
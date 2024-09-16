# SecureContainer - Ensuring security in the storage of sensitive data through LUKS containers

## Purpose

The purpose of this component is to provide a way to securely store sensitive data through LUKS containers while checking the integirty of the data.

## Run

Dependencies:

Install libuta unified Trust Anchor API: 
https://github.com/siemens/libuta

```bash
apt-get install cryptestup 
```
To properly utilise this tool, the `secure_container_daemon` must be started as it serves as a gRPC server for the `secure_container_cli`.

Possible commands for `secure_container_cli` are `create`, `open`, `close`, `export` and `import`.

Example: 
```bash
> ./secure_container_cli open <MOUNT_POINT> <PATH> <NAMESPACE> <ID> -auto_open
```


To run the `secure_container_daemon`:

```bash
> ./secure_container_daemon
```

To run the `secure_container_cli`:

```bash
> ./secure_container_cli <COMMAD>
```

### Install debian package

```bash
dpkg -i target/debian/secure-container_0.1.0-1_amd64.deb
```

## Usage
To interact with the secure container daemon, there are three possible ways to achive this:
For all the following examples, the `secure_container_daemon` must be started.

1) Using the `secure_container_cli`:
```bash
> ./secure_container_cli <COMMAND>
```
This will send a gRPC request to the `secure_container_daemon` to execute the command.
This commandline tool can be used system-wide after installing the debian package.

2) Using the `secure_container_lib`:
```rust
use secure_container_lib::*;
```
This will allow you to use the library to interact with the `secure_container_daemon` using the gRPC requests.
For more information, please refer to the documentation.
An Example on how to use the library is provided in the `/src/cli.rs` file.

3) Writing your own gRPC client:
If you want to use the secure container daemon in a different language, then rust, you can write your own gRPC client.
You can find the proto file under `/proto/SecureContainer.proto`
   and thus write your own gRPC client in your desired language.


## Build


### Build dependencies

You can install build dependencies using the command below:

```bash
apt-get install cargo cryptsetup libclang-dev clang llvm
rustup toolchain install nightly
rustup default nightly
```

### Debug build 

```bash
cargo build
```

### Release build 

Optimises for size and strips debug info.

```bash
cargo build --release
```

### Build debian package

First install cargo-deb with the following command:

```bash
cargo install cargo-deb

```
Then you can build the debian package unseing this command:
```bash
cargo deb
```



## Run Test

```bash
# Run unit tests
cargo test --all-features -- --test-threads 1 --nocapture

# Run positive functionality tests
./tests/positive_testing.sh

# Run negative functionality tests
./tests/negative_testing.sh
```

## Documentation
To generate the documentation for the components, run the following command:

For the secure container library:
```bash
cargo doc --open --lib --no-deps
```

For the secure container CLI:
```bash
cargo doc --open --bin secure_container_cli --no-deps
```

For the secure container daemon:
```bash
cargo doc --open --bin secure_container_daemon --no-deps
```

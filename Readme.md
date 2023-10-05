# RustWatchman - Rust Tracing Writer

RustWatchman is a Rust library that provides a logging implementation for writing logs to Amazon CloudWatch. It uses the AWS SDK for Rust to implement the CloudWatch logging functionality.

## Installation

1. Install Rust and Cargo.
2. Clone the repository.
3. Run `cargo build --release` to build the project.
4. Run `cargo test` to run the unit tests.

## Usage

To use RustWatchman in your Rust project, add the follohttps://docs.rs/tracing/latest/tracing/wing to your `Cargo.toml` file:

```toml
[dependencies]
rust_watchman = { git = "https://github.com/prabhat0206/RustWatchman.git" }
```
Then, in your Rust code, import the TracingCWLogger struct from the rust_watchman crate:
    
```rust
use rust_watchman::TracingCWLogger;
```
You can then create a new instance of the TracingCWLogger struct and use it to write logs to CloudWatch:
    
```rust
#[tokio::main]
async fn main() {
    let sdk_config = aws_config::load_from_env().await;
    let writer = Watchman::new(sdk_config, "test".to_string(), "new".to_string()).await.get_writer();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_writer(writer)
        .init();
    tracing::info!("Hello, world!");
}
```

### Contributing
Contributions are welcome! Please open an issue or submit a pull request.

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Helpful Links
- [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust)
- [Tracing](https://docs.rs/tracing/latest/tracing/)

### Authors
- [Prabhat Ranjan](https://github.com/prabhat0206/prabhat0206)

#### Warning
This project is still in development and is not ready for production use. Use at your own risk.

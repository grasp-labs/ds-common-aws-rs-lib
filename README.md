# DS Common AWS Rust Library

[![Crates.io version](https://img.shields.io/crates/v/ds-common-aws-rs-lib.svg)](https://crates.io/crates/ds-common-aws-rs-lib)
[![Documentation](https://docs.rs/ds-common-aws-rs-lib/badge.svg)](https://docs.rs/ds-common-aws-rs-lib)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.76%2B-blue.svg)](https://www.rust-lang.org)

A comprehensive Rust library for AWS services integration, providing high-level abstractions for AWS SDK operations with common utilities and error handling.

## Features

- **Modular Design** - Choose only the AWS services you need with feature flags
- **SSM Parameter Store** - Simplified parameter retrieval and management
- **SQS Queue Service** - Simplified queue operations and message handling
- **Unified Error Handling** - Comprehensive error handling with AWS-specific error types
- **Async Support** - Full async/await support for all operations
- **Type Safety** - Strong typing for AWS operations and responses
- **Zero-cost Abstractions** - High-level APIs with minimal overhead

## Feature Flags

This crate supports the following features:

- `ssm` - AWS Systems Manager Parameter Store support (enabled by default)
- `sqs` - AWS Simple Queue Service support
- `s3` - AWS Simple Storage Service support
- `full` - Enables all features (equivalent to `ssm` + `sqs` + `s3`)

## Installation

```toml
[dependencies]
ds-common-aws-rs-lib = "0.1.0"
```

Or use cargo add:

```sh
cargo add ds-common-aws-rs-lib
```

## Quick Start

### Using the unified AWS service (recommended)

```rust,no_run
use aws_sdk_ssm::types::ParameterType;
use ds_common_aws_rs_lib::client::AwsClient;
use ds_common_aws_rs_lib::error::{Result, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create AWS service once with shared configuration
    let aws = AwsClient::new().await?;

    // Get specialized services (very fast, no network calls)
    let ssm = aws.ssm();
    let sqs = aws.sqs();
    let s3 = aws.s3();
    // Use services with shared client
    let parameter = ssm.get_parameter("/myapp/database/url").await?;
    let queue_url = sqs.get_queue("my-queue").await?;
    sqs.send_message(&queue_url, "Hello, world!", None).await?;
    ssm.put_parameter("/myapp/database/url", "my-database-url", ParameterType::String, true).await?;
    s3.put_object("my-bucket", "my-key", b"test-value".to_vec(), None).await?;
    Ok(())
}
```

### Benefits of AwsClient

- ✅ **Single configuration load** - Expensive credential resolution happens once
- ✅ **Fast service creation** - No network calls when creating services
- ✅ **Shared connection pools** - Better resource management
- ✅ **Easy extensibility** - Simple to add S3, DynamoDB, etc.
- ✅ **Better performance** - Reuse clients instead of creating new ones

## API Reference

### SSM Parameter Store

```rust,no_run
use ds_common_aws_rs_lib::client::AwsClient;
use aws_sdk_ssm::types::ParameterType;
use ds_common_aws_rs_lib::error::{Result, Error};

// Get a parameter
let aws = AwsClient::new().await?;
let ssm = aws.ssm();

let value = ssm.get_parameter("/myapp/database/url").await?;

// Put a parameter
ssm.put_parameter(
    "/myapp/database/url",
    "postgresql://localhost:5432/mydb",
    ParameterType::String,
    true
).await?;
```

### SQS Queue Service

```rust,no_run
use ds_common_aws_rs_lib::client::AwsClient;
use ds_common_aws_rs_lib::sqs::SqsService;
use ds_common_aws_rs_lib::error::{Result, Error};

// Get queue URL
let aws = AwsClient::new().await?;
let sqs = aws.sqs();
let queue_url = sqs.get_queue("my-queue").await?;

// Send message with no delay
sqs.send_message(&queue_url, "Hello, world!", None).await?;

// Send message with 10 second delay
sqs.send_message(&queue_url, "Delayed message", Some(10)).await?;
```

## Error Handling

The library provides unified error handling across all modules:

```rust,no_run
use ds_common_aws_rs_lib::error::{Result, Error};
use ds_common_aws_rs_lib::client::AwsClient;

// All functions return Result<T, Error>
async fn example() -> Result<(), Error> {
    // SSM errors are automatically converted to Error::Ssm
    let aws = AwsClient::new().await?;
    let ssm = aws.ssm();
    let sqs = aws.sqs();

    let param = ssm.get_parameter("/myapp/config").await?;
    let queue_url = sqs.get_queue("my-queue").await?;

    Ok(())
}
```

## Supported Services

- **SSM Parameter Store** - Secure parameter storage and retrieval
- **SQS Queue Service** - Message queuing and processing
- **Unified Error Handling** - Consistent error types across all services
- **Async Operations** - Full async/await support for all AWS operations

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

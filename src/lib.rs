//! DS Common AWS Rust Library
//!
//! A comprehensive Rust library for AWS services integration, providing high-level abstractions
//! for AWS SDK operations with common utilities and error handling.
//!
//! # Features
//!
//! - **AWS SDK Integration** - High-level abstractions for AWS services
//! - **SSM Parameter Store** - Simplified parameter retrieval and management
//! - **Error Handling** - Comprehensive error handling with AWS-specific error types
//! - **Configuration Management** - Easy AWS configuration setup and management
//! - **Async Support** - Full async/await support for all operations
//! - **Type Safety** - Strong typing for AWS operations and responses
//!
//! # Features
//!
//! This crate supports the following features:
//! - `ssm` - AWS Systems Manager Parameter Store support (enabled by default)
//! - `sqs` - AWS Simple Queue Service support
//! - `full` - Enables all features (equivalent to `ssm` + `sqs`)
//!
//! # Quick Start
//!
//! ## Using the unified AWS service (recommended)
//! ```rust,no_run
//! use aws_sdk_ssm::types::ParameterType;
//! use ds_common_aws_rs_lib::client::AwsClient;
//! use ds_common_aws_rs_lib::error::{Result, Error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     // Create AWS client once with shared configuration
//!     let aws = AwsClient::new().await?;
//!
//!     // Get specialized services (very fast, no network calls)
//!     let ssm = aws.ssm();
//!     let sqs = aws.sqs();
//!
//!     // Use services with shared client
//!     let parameter = ssm.get_parameter("/myapp/database/url").await?;
//!     let queue_url = sqs.get_queue("my-queue").await?;
//!     sqs.send_message(&queue_url, "Hello, world!", None).await?;
//!     ssm.put_parameter("/myapp/database/url", "my-database-url", ParameterType::String, true).await?;
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;

#[cfg(feature = "ssm")]
pub mod ssm;
#[cfg(feature = "sqs")]
pub mod sqs;


// Re-export commonly used AWS types for convenience
pub use aws_config;
#[cfg(feature = "ssm")]
pub use aws_sdk_ssm;
#[cfg(feature = "sqs")]
pub use aws_sdk_sqs;

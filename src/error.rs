//! Error module.
//!
//! ## Overview
//! This module contains the unified error types for the DS Common AWS Rust Library.
//! All functions return `Result<T, Error>` for consistent error handling.
//!
//! ## Error Hierarchy
//!
//! - [`SsmError`]: Errors from SSM operations
//! - [`SqsError`]: Errors from SQS operations
//! - [`S3Error`]: Errors from S3 operations
//!

use thiserror::Error;

#[cfg(feature = "s3")]
use crate::s3::error::S3Error;
#[cfg(feature = "sqs")]
use crate::sqs::error::SqsError;
#[cfg(feature = "ssm")]
use crate::ssm::error::SsmError;

/// Unified result type for all operations.
///
/// This is equivalent to `Result<T, Error>` and provides consistent
/// error handling across all modules.
pub type Result<T, E = Error> = std::result::Result<T, E>;

// region: --> Error

/// Top-level error enum that unifies all module errors.
///
/// This enum provides a single error type for all operations,
/// making error handling consistent and ergonomic for users.
#[derive(Error, Debug)]
pub enum Error {
    /// Errors from SSM operations
    #[cfg(feature = "ssm")]
    #[error(transparent)]
    Ssm(#[from] SsmError),

    /// Errors from SQS operations
    #[cfg(feature = "sqs")]
    #[error(transparent)]
    Sqs(#[from] SqsError),

    /// Errors from S3 operations
    #[cfg(feature = "s3")]
    #[error(transparent)]
    S3(#[from] S3Error),
}

// endregion: --> Error

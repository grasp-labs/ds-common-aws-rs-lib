//! Error module.
//!
//! ## Overview
//! This module contains the unified error types for the DS Common AWS Rust Library.
//! All functions return `Result<T, Error>` for consistent error handling.
//!
//! ## Error Hierarchy
//!
//! - [`Error`]: Top-level error enum that unifies all module errors
//! - [`SsmError`]: Errors from SSM operations
//! - [`SqsError`]: Errors from SQS operations
//!

use thiserror::Error;

#[cfg(feature = "ssm")]
use crate::ssm::error::SsmError;
#[cfg(feature = "sqs")]
use crate::sqs::error::SqsError;

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
}

// endregion: --> Error

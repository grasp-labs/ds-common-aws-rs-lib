//! AWS S3 error module.
//!
//! This module contains error types specific to AWS S3 operations.

use thiserror::Error;

// region: --> S3Error

/// Errors that can occur during AWS S3 operations.
///
/// This enum covers all possible errors when creating S3 clients or processing objects,
/// including S3 client or operation errors, and not found errors.
#[derive(Error, Debug)]
pub enum S3Error {
    /// Request build failed
    #[error("request build failed")]
    Build,

    /// Network/dispatch failure
    #[error("network/dispatch failed")]
    Transport {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
        request_id: Option<String>,
    },

    /// Request timeout
    #[error("timeout")]
    Timeout { request_id: Option<String> },

    /// Service error
    #[error("service error: {0}")]
    Service(#[source] Box<dyn std::error::Error + Send + Sync>),

    /// Invalid response
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

// endregion: --> S3Error

// region: --> Helpers

/// Convert AWS SDK errors to S3Error
pub fn map_s3_err<E, R>(e: aws_smithy_runtime_api::client::result::SdkError<E, R>) -> S3Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    use aws_smithy_runtime_api::client::result::SdkError::*;
    match e {
        ConstructionFailure(_cf) => S3Error::Build,
        TimeoutError(_te) => S3Error::Timeout { request_id: None },
        DispatchFailure(_df) => S3Error::Transport {
            request_id: None,
            source: Box::new(std::io::Error::other("Dispatch failure")),
        },
        ResponseError(_re) => S3Error::Service(Box::new(std::io::Error::other("Response error"))),
        ServiceError(se) => S3Error::Service(Box::new(se.into_err())),
        _ => S3Error::Service(Box::new(std::io::Error::other("Unknown error"))),
    }
}

// endregion: --> Helpers

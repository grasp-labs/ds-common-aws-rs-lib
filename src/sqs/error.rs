//! AWS SQS error module.
//!
//! This module contains error types specific to AWS SQS operations.

use thiserror::Error;

// region: --> SqsError

/// Errors that can occur during AWS SQS operations.
///
/// This enum covers all possible errors when creating SQS clients or processing messages,
/// including SQS client or operation errors, and not found errors.
#[derive(Error, Debug)]
pub enum SqsError {
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

// endregion: --> SqsError

// region: --> Helpers

/// Convert AWS SDK errors to SqsError
pub fn map_sqs_err<E, R>(e: aws_smithy_runtime_api::client::result::SdkError<E, R>) -> SqsError
where
    E: std::error::Error + Send + Sync + 'static,
{
    use aws_smithy_runtime_api::client::result::SdkError::*;
    match e {
        ConstructionFailure(_cf) => SqsError::Build,
        TimeoutError(_te) => SqsError::Timeout { request_id: None },
        DispatchFailure(_df) => SqsError::Transport {
            request_id: None,
            source: Box::new(std::io::Error::other("Dispatch failure")),
        },
        ResponseError(_re) => SqsError::Service(Box::new(std::io::Error::other("Response error"))),
        ServiceError(se) => SqsError::Service(Box::new(se.into_err())),
        _ => SqsError::Service(Box::new(std::io::Error::other("Unknown error"))),
    }
}

// endregion: --> Helpers

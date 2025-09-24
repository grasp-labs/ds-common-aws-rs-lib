//! AWS SSM error module.
//!
//! This module contains error types specific to AWS SSM operations.

use thiserror::Error;

// region: --> SsmError

/// Errors that can occur during AWS SSM operations.
///
/// This enum covers all possible errors when creating SSM clients or processing parameters,
/// including SSM client or operation errors, and not found errors.
#[derive(Error, Debug)]
pub enum SsmError {
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

// endregion: --> SsmError

// region: --> Helpers

/// Convert AWS SDK errors to SsmError
pub fn map_ssm_err<E, R>(e: aws_smithy_runtime_api::client::result::SdkError<E, R>) -> SsmError
where
    E: std::error::Error + Send + Sync + 'static,
{
    use aws_smithy_runtime_api::client::result::SdkError::*;
    match e {
        ConstructionFailure(_cf) => SsmError::Build,
        TimeoutError(_te) => SsmError::Timeout { request_id: None },
        DispatchFailure(_df) => SsmError::Transport {
            request_id: None,
            source: Box::new(std::io::Error::other("Dispatch failure")),
        },
        ResponseError(_re) => SsmError::Service(Box::new(std::io::Error::other("Response error"))),
        ServiceError(se) => SsmError::Service(Box::new(se.into_err())),
        _ => SsmError::Service(Box::new(std::io::Error::other("Unknown error"))),
    }
}

// endregion: --> Helpers

//! AWS Service module.
//!
//! This module provides a unified AWS service that manages shared clients
//! and provides specialized services for different AWS services.

use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;

use crate::error::{Error, Result};

/// Main AWS client that manages shared configuration and clients
///
/// This client provides a unified way to access AWS services with shared
/// configuration and client management for better performance and resource usage.
pub struct AwsClient {
    config: SdkConfig,
}

impl AwsClient {
    /// Create a new AWS client with default configuration
    ///
    /// # Returns
    ///
    /// Returns a new AwsClient instance, or an error if the operation fails.
    pub async fn new() -> Result<Self, Error> {
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(RegionProviderChain::default_provider().or_else("eu-north-1"))
            .load()
            .await;
        Ok(Self { config })
    }

    /// Create a new AWS client with a custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The AWS SDK configuration to use
    ///
    /// # Returns
    ///
    /// Returns a new AwsClient instance with the provided configuration.
    pub fn with_config(config: SdkConfig) -> Self {
        Self { config }
    }

    /// Get the underlying SDK configuration
    ///
    /// # Returns
    ///
    /// Returns a reference to the SDK configuration.
    pub fn config(&self) -> &SdkConfig {
        &self.config
    }

    /// Create an SQS service using the shared configuration
    ///
    /// # Returns
    ///
    /// Returns a new SqsService instance using the shared configuration.
    #[cfg(feature = "sqs")]
    pub fn sqs(&self) -> crate::sqs::SqsService {
        crate::sqs::SqsService::with_config(&self.config)
    }

    /// Create an SSM service using the shared configuration
    ///
    /// # Returns
    ///
    /// Returns a new SsmService instance using the shared configuration.
    #[cfg(feature = "ssm")]
    pub fn ssm(&self) -> crate::ssm::SsmService {
        crate::ssm::SsmService::with_config(&self.config)
    }
}

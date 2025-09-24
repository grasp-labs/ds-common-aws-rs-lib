//! AWS SQS module.
//!
//! This module contains functions for interacting with AWS SQS.

pub mod error;

use aws_sdk_sqs::Client;

use crate::error::{Error, Result};
use crate::sqs::error::{map_sqs_err, SqsError};

/// SQS service for interacting with AWS SQS
///
/// This service provides a high-level interface for SQS operations
/// with dependency injection for better performance and testability.
pub struct SqsService {
    client: Client,
}

impl SqsService {
    /// Create a new SQS service with a custom client
    ///
    /// # Arguments
    ///
    /// * `client` - The SQS client to use
    ///
    /// # Returns
    ///
    /// Returns a new SqsService instance with the provided client.
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// Create a new SQS service with a shared configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The AWS SDK configuration to use
    ///
    /// # Returns
    ///
    /// Returns a new SqsService instance with the provided configuration.
    pub fn with_config(config: &aws_config::SdkConfig) -> Self {
        Self {
            client: Client::new(config),
        }
    }

    /// Get an SQS queue URL
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the queue to get
    ///
    /// # Returns
    ///
    /// Returns the queue URL as a String
    ///
    /// # Errors
    ///
    /// * [`SqsError::Service`] - If the SQS client fails to create.
    /// * [`SqsError::Timeout`] - If the request times out.
    /// * [`SqsError::Transport`] - If the request fails to dispatch.
    /// * [`SqsError::Build`] - If the request fails to build.
    /// * [`SqsError::InvalidResponse`] - If the queue URL is not found.
    pub async fn get_queue(&self, name: &str) -> Result<String> {
        let response = self
            .client
            .get_queue_url()
            .queue_name(name)
            .send()
            .await
            .map_err(map_sqs_err)?;

        match response.queue_url() {
            Some(url) => Ok(url.to_string()),
            None => Err(Error::from(SqsError::InvalidResponse(name.to_string()))),
        }
    }

    /// Send an SQS message
    ///
    /// # Arguments
    ///
    /// * `queue_url` - The URL of the queue to send
    /// * `body` - The message to send
    /// * `delay_seconds` - The delay in seconds before the message is sent
    ///
    /// # Returns
    ///
    /// Returns the message ID as a String
    ///
    /// # Errors
    ///
    /// * [`SqsError::Service`] - If the SQS client fails to create.
    /// * [`SqsError::Timeout`] - If the request times out.
    /// * [`SqsError::Transport`] - If the request fails to dispatch.
    /// * [`SqsError::Build`] - If the request fails to build.
    /// * [`SqsError::InvalidResponse`] - If the message ID is not found.
    pub async fn send_message(&self, queue_url: &str, body: &str, delay_seconds: Option<i32>) -> Result<String> {
        let response = self
            .client
            .send_message()
            .queue_url(queue_url)
            .message_body(body)
            .delay_seconds(delay_seconds.unwrap_or(0))
            .send()
            .await
            .map_err(map_sqs_err)?;

        match response.message_id() {
            Some(id) => Ok(id.to_string()),
            None => Err(Error::from(SqsError::InvalidResponse(queue_url.to_string()))),
        }
    }
}

//! AWS S3 module.
//!
//! This module contains functions for interacting with AWS S3.

pub mod error;

use aws_sdk_s3::{primitives::ByteStream, Client};

use crate::error::{Error, Result};
use crate::s3::error::{map_s3_err, S3Error};

/// S3 service for interacting with AWS S3
///
/// This service provides a high-level interface for S3 operations
/// with dependency injection for better performance and testability.
pub struct S3Service {
    client: Client,
}

impl S3Service {
    /// Create a new S3 service with a custom client
    ///
    /// # Arguments
    ///
    /// * `client` - The S3 client to use
    ///
    /// # Returns
    ///
    /// Returns a new S3Service instance with the provided client.
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// Create a new S3 service with a shared configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The AWS SDK configuration to use
    ///
    /// # Returns
    ///
    /// Returns a new S3Service instance with the provided configuration.
    pub fn with_config(config: &aws_config::SdkConfig) -> Self {
        Self {
            client: Client::new(config),
        }
    }

    /// Fetch an object from S3 and return it as a UTF-8 string.
    ///
    /// # Arguments
    ///
    /// * `bucket` - Name of the S3 bucket.
    /// * `key` - Object key within the bucket.
    ///
    /// # Returns
    ///
    /// The object contents as a `String`.
    ///
    /// # Errors
    ///
    /// Returns [`S3Error`] variants on client, network, or response failures.
    pub async fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>> {
        let response = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(map_s3_err)?;

        let data = response
            .body
            .collect()
            .await
            .map_err(|e| {
                Error::from(S3Error::Service(Box::new(std::io::Error::other(format!(
                    "Failed to collect S3 body: {}",
                    e
                )))))
            })?
            .into_bytes()
            .to_vec();

        Ok(data)
    }

    /// Put an S3 object
    ///
    /// # Arguments
    ///
    /// * `bucket` - The bucket of the object to put
    /// * `key` - The key of the object to put
    /// * `value` - The value of the object to put
    /// * `content_type` - The content type of the object
    ///
    /// # Returns
    ///
    /// Returns nothing
    ///
    /// # Errors
    ///
    /// * [`S3Error::Service`] - If the S3 client fails to create.
    /// * [`S3Error::Timeout`] - If the request times out.
    /// * [`S3Error::Transport`] - If the request fails to dispatch.
    /// * [`S3Error::Build`] - If the request fails to build.
    pub async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        value: impl Into<ByteStream>,
        content_type: Option<&str>,
    ) -> Result<()> {
        let mut req = self.client.put_object().bucket(bucket).key(key).body(value.into());

        if let Some(ct) = content_type {
            req = req.content_type(ct);
        }

        req.send().await.map_err(map_s3_err)?;
        Ok(())
    }
}

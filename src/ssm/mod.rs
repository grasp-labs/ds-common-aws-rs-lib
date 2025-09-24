//! AWS SSM module.
//!
//! This module contains functions for interacting with AWS SSM.

pub mod error;

use aws_sdk_ssm::{Client, types::ParameterType};

use crate::ssm::error::{SsmError, map_ssm_err};
use crate::error::{Result, Error};

/// SSM service for interacting with AWS Systems Manager
///
/// This service provides a high-level interface for SSM operations
/// with dependency injection for better performance and testability.
pub struct SsmService {
    client: Client,
}

impl SsmService {
    /// Create a new SSM service with a custom client
    ///
    /// # Arguments
    ///
    /// * `client` - The SSM client to use
    ///
    /// # Returns
    ///
    /// Returns a new SsmService instance with the provided client.
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// Create a new SSM service with a shared configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The AWS SDK configuration to use
    ///
    /// # Returns
    ///
    /// Returns a new SsmService instance with the provided configuration.
    pub fn with_config(config: &aws_config::SdkConfig) -> Self {
        Self {
            client: Client::new(config),
        }
    }

    /// Get an SSM parameter
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the parameter to get
    ///
    /// # Returns
    ///
    /// Returns the parameter value as a String
    ///
    /// # Errors
    ///
    /// * [`SsmError::Service`] - If the SSM client fails to create.
    /// * [`SsmError::Timeout`] - If the request times out.
    /// * [`SsmError::Transport`] - If the request fails to dispatch.
    /// * [`SsmError::Build`] - If the request fails to build.
    /// * [`SsmError::InvalidResponse`] - If the parameter is not found.
    pub async fn get_parameter(&self, name: &str) -> Result<String> {
        let response = self.client
            .get_parameter()
            .name(name)
            .with_decryption(true)
            .send()
            .await
            .map_err(map_ssm_err)?;

        match response.parameter() {
            Some(parameter) => match parameter.value() {
                Some(value) => Ok(value.to_string()),
                None => Err(Error::from(SsmError::InvalidResponse(name.to_string()))),
            },
            None => Err(Error::from(SsmError::InvalidResponse(name.to_string()))),
        }
    }

    /// Put an SSM parameter
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the parameter to put
    /// * `value` - The value of the parameter to put
    /// * `parameter_type` - The type of the parameter
    /// * `overwrite` - Whether to overwrite an existing parameter
    ///
    /// # Returns
    ///
    /// Returns nothing
    ///
    /// # Errors
    ///
    /// * [`SsmError::Service`] - If the SSM client fails to create.
    /// * [`SsmError::Timeout`] - If the request times out.
    /// * [`SsmError::Transport`] - If the request fails to dispatch.
    /// * [`SsmError::Build`] - If the request fails to build.
    pub async fn put_parameter(&self, name: &str, value: &str, parameter_type: ParameterType, overwrite: bool) -> Result<()> {
        self.client
            .put_parameter()
            .name(name)
            .value(value)
            .r#type(parameter_type)
            .overwrite(overwrite)
            .send()
            .await
            .map_err(map_ssm_err)?;

        Ok(())
    }
}

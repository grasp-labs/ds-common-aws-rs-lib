use ds_common_aws_rs_lib::client::AwsClient;

#[tokio::test]
async fn test_aws_client_creation_panic_safety() {
    // Test that AWS client creation doesn't panic
    let result = AwsClient::new().await;
    match result {
        Ok(client) => {
            assert!(client.config().region().is_some());
        }
        Err(_) => {
            // It's okay if it fails due to missing credentials, but it shouldn't panic
            // Test passes if we reach this point without panicking
        }
    }
}

#[tokio::test]
async fn test_aws_client_with_config_panic_safety() {
    // Test that AWS client creation with custom config doesn't panic
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region("us-west-2")
        .load()
        .await;

    let client = AwsClient::with_config(config);
    assert!(client.config().region().is_some());
}

#[cfg(feature = "ssm")]
#[tokio::test]
async fn test_ssm_service_creation_panic_safety() {
    // Test that SSM service creation doesn't panic
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region("us-west-2")
        .load()
        .await;

    let aws_client = AwsClient::with_config(config);
    let _ssm_service = aws_client.ssm();
    // Test that we can create the service without panicking
    // Service creation is successful if we reach this point
}

#[cfg(feature = "sqs")]
#[tokio::test]
async fn test_sqs_service_creation_panic_safety() {
    // Test that SQS service creation doesn't panic
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region("us-west-2")
        .load()
        .await;

    let aws_client = AwsClient::with_config(config);
    let _sqs_service = aws_client.sqs();
    // Test that we can create the service without panicking
    // Service creation is successful if we reach this point
}

#[cfg(feature = "s3")]
#[tokio::test]
async fn test_s3_service_creation_panic_safety() {
    // Test that S3 service creation doesn't panic
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region("us-west-2")
        .load()
        .await;

    let aws_client = AwsClient::with_config(config);
    let _s3_service = aws_client.s3();
    // Test that we can create the service without panicking
    // Service creation is successful if we reach this point
}

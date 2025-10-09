use ds_common_aws_rs_lib::client::AwsClient;

#[tokio::test]
async fn test_aws_client_creation() {
    let client = AwsClient::new().await.unwrap();
    assert!(client.config().region().is_some());
}

#[tokio::test]
async fn test_aws_client_with_custom_config() {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region("us-west-2")
        .load()
        .await;

    let client = AwsClient::with_config(config);
    assert!(client.config().region().is_some());
}

#[cfg(feature = "ssm")]
#[tokio::test]
async fn test_ssm_service_creation() {
    let aws_client = AwsClient::new().await.unwrap();
    let _ssm_service = aws_client.ssm();
    // Test that we can create the service without panicking
    // Service creation is successful if we reach this point
}

#[cfg(feature = "sqs")]
#[tokio::test]
async fn test_sqs_service_creation() {
    let aws_client = AwsClient::new().await.unwrap();
    let _sqs_service = aws_client.sqs();
    // Test that we can create the service without panicking
    // Service creation is successful if we reach this point
}

#[cfg(feature = "s3")]
#[tokio::test]
async fn test_s3_service_creation() {
    let aws_client = AwsClient::new().await.unwrap();
    let _s3_service = aws_client.s3();
    // Test that we can create the service without panicking
    // Service creation is successful if we reach this point
}

#[tokio::test]
async fn test_aws_client_config_access() {
    let client = AwsClient::new().await.unwrap();
    let config = client.config();
    assert!(config.region().is_some());
}

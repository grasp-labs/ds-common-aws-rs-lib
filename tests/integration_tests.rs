use ds_common_aws_rs_lib::aws_config::{BehaviorVersion, Region};
use ds_common_aws_rs_lib::aws_sdk_ssm::Client as SsmClient;

#[tokio::test]
async fn test_aws_config_creation() {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("us-west-2"))
        .load()
        .await;

    assert!(config.region().is_some());
}

#[tokio::test]
async fn test_ssm_client_creation() {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("us-west-2"))
        .load()
        .await;

    let client = SsmClient::new(&config);
    assert!(!client.config().region().is_none());
}

#[test]
fn test_aws_region_creation() {
    let region = Region::new("us-east-1");
    assert_eq!(region.as_ref(), "us-east-1");
}

#[test]
fn test_aws_region_default() {
    let region = Region::new("us-west-2");
    assert_eq!(region.as_ref(), "us-west-2");
}

#[tokio::test]
async fn test_aws_config_with_custom_region() {
    let region = Region::new("eu-north-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .load()
        .await;

    assert_eq!(config.region().unwrap().as_ref(), "eu-north-1");
}

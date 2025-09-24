use ds_common_aws_rs_lib::aws_config::{BehaviorVersion, Region};
use ds_common_aws_rs_lib::aws_sdk_ssm::Client as SsmClient;

#[tokio::test]
async fn test_aws_config_panic_safety() {
    // Test that AWS config creation doesn't panic
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("us-west-2"))
        .load()
        .await;

    assert!(config.region().is_some());
}

#[tokio::test]
async fn test_ssm_client_creation_safety() {
    // Test that SSM client creation doesn't panic
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("us-west-2"))
        .load()
        .await;

    let client = SsmClient::new(&config);
    assert!(!client.config().region().is_none());
}

#[test]
fn test_region_creation_safety() {
    // Test region creation with various inputs doesn't panic
    let regions = ["us-east-1", "us-west-2", "eu-north-1", "ap-southeast-1"];

    for region_str in regions {
        let region = Region::new(region_str);
        assert_eq!(region.as_ref(), region_str);
    }
}

#[test]
fn test_region_creation_with_empty_string() {
    // Test region creation with empty string doesn't panic
    let region = Region::new("");
    assert_eq!(region.as_ref(), "");
}

#[test]
fn test_region_creation_with_long_string() {
    // Test region creation with very long string doesn't panic
    let long_region = "x".repeat(1000);
    let region = Region::new(long_region.clone());
    assert_eq!(region.as_ref(), long_region);
}

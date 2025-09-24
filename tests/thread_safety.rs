use ds_common_aws_rs_lib::aws_config::{BehaviorVersion, Region};
use ds_common_aws_rs_lib::aws_sdk_ssm::Client as SsmClient;
use std::sync::Arc;
use std::thread;

#[tokio::test]
async fn test_aws_config_creation_thread_safety() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let region_str = format!("us-west-{}", i % 3 + 1);
                    let region = Region::new(region_str);
                    let config = aws_config::defaults(BehaviorVersion::latest())
                        .region(region)
                        .load()
                        .await;
                    config
                })
            })
        })
        .collect();

    for handle in handles {
        let config = handle.join().unwrap();
        assert!(config.region().is_some());
    }
}

#[tokio::test]
async fn test_ssm_client_creation_thread_safety() {
    let config = Arc::new(
        aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new("us-west-2"))
            .load()
            .await
    );

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                let client = SsmClient::new(&config);
                assert!(!client.config().region().is_none());
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_region_creation_concurrent() {
    let handles: Vec<_> = (0..20)
        .map(|i| {
            thread::spawn(move || {
                let regions = ["us-east-1", "us-west-2", "eu-north-1", "ap-southeast-1"];
                let region_str = regions[i % regions.len()];
                let region = Region::new(region_str);
                (region, region_str)
            })
        })
        .collect();

    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();

    assert_eq!(results.len(), 20);
    for (region, expected) in results {
        assert_eq!(region.as_ref(), expected);
    }
}

#[test]
fn test_aws_config_shared_across_threads() {
    let config = Arc::new(
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            aws_config::defaults(BehaviorVersion::latest())
                .region(Region::new("us-west-2"))
                .load()
                .await
        })
    );

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                let client = SsmClient::new(&config);
                assert_eq!(client.config().region().unwrap().as_ref(), "us-west-2");
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

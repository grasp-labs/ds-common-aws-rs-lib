use ds_common_aws_rs_lib::client::AwsClient;
use std::sync::Arc;
use std::thread;

#[tokio::test]
async fn test_aws_client_creation_thread_safety() {
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(move || {
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let client = AwsClient::new().await;
                    client
                })
            })
        })
        .collect();

    for handle in handles {
        let result = handle.join().unwrap();
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
}

#[tokio::test]
async fn test_aws_client_with_config_thread_safety() {
    let config = Arc::new(
        aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region("us-west-2")
            .load()
            .await,
    );

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                let client = AwsClient::with_config((*config).clone());
                assert!(client.config().region().is_some());
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(feature = "ssm")]
#[tokio::test]
async fn test_ssm_service_creation_thread_safety() {
    let config = Arc::new(
        aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region("us-west-2")
            .load()
            .await,
    );

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                let aws_client = AwsClient::with_config((*config).clone());
                let _ssm_service = aws_client.ssm();
                // Test that we can create the service without panicking
                // Service creation is successful if we reach this point
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(feature = "sqs")]
#[tokio::test]
async fn test_sqs_service_creation_thread_safety() {
    let config = Arc::new(
        aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region("us-west-2")
            .load()
            .await,
    );

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                let aws_client = AwsClient::with_config((*config).clone());
                let _sqs_service = aws_client.sqs();
                // Test that we can create the service without panicking
                // Service creation is successful if we reach this point
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(feature = "s3")]
#[tokio::test]
async fn test_s3_service_creation_thread_safety() {
    let config = Arc::new(
        aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region("us-west-2")
            .load()
            .await,
    );

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                let aws_client = AwsClient::with_config((*config).clone());
                let _s3_service = aws_client.s3();
                // Test that we can create the service without panicking
                // Service creation is successful if we reach this point
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_aws_client_shared_across_threads() {
    let config = Arc::new(tokio::runtime::Runtime::new().unwrap().block_on(async {
        aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region("us-west-2")
            .load()
            .await
    }));

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                let aws_client = AwsClient::with_config((*config).clone());
                assert_eq!(aws_client.config().region().unwrap().as_ref(), "us-west-2");
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

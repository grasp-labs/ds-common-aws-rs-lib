use ds_common_aws_rs_lib::client::AwsClient;
use ds_common_aws_rs_lib::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let aws = AwsClient::new().await?;
    let s3 = aws.s3();

    // Example: Put a simple text object
    let content = b"Hello, S3! This is a test object.".to_vec();
    s3.put_object("my-test-bucket", "test-object.txt", content, Some("text/plain"))
        .await?;
    Ok(())
}

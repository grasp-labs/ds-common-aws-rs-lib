use ds_common_aws_rs_lib::client::AwsClient;
use ds_common_aws_rs_lib::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let aws = AwsClient::new().await?;
    let s3 = aws.s3();

    // Example: Get an object from S3
    let object = s3.get_object("my-test-bucket", "test-object.txt").await?;
    let object_str = String::from_utf8(object).unwrap();
    println!("Object content: {}", object_str);
    Ok(())
}

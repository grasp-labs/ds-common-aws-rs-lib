use ds_common_aws_rs_lib::client::AwsClient;
use ds_common_aws_rs_lib::error::{Result, Error};
use ds_common_aws_rs_lib::sqs::error::SqsError;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let aws = AwsClient::new().await?;
    let sqs = aws.sqs();
    match sqs.get_queue("test-queue").await {
        Ok(queue_url) => {
            println!("Queue URL: {}", queue_url);
            Ok(())
        }
        Err(Error::Sqs(SqsError::InvalidResponse(name))) => {
            println!("Invalid response: {}", name);
            Err(Error::Sqs(SqsError::InvalidResponse(name)))
        }
        Err(Error::Sqs(SqsError::Timeout { request_id })) => {
            println!("Request timed out, request_id: {:?}", request_id);
            Err(Error::Sqs(SqsError::Timeout { request_id }))
        }
        Err(Error::Sqs(SqsError::Service(source))) => {
            println!("SQS service error: {}", source);
            Err(Error::Sqs(SqsError::Service(source)))
        }
        Err(Error::Sqs(e)) => {
            println!("Other SQS error: {}", e);
            Err(Error::Sqs(e))
        }
        Err(e) => {
            println!("Unknown error: {}", e);
            Err(e)
        }
    }
}
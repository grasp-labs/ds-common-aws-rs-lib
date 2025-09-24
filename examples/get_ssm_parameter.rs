use ds_common_aws_rs_lib::client::AwsClient;
use ds_common_aws_rs_lib::error::{Error, Result};
use ds_common_aws_rs_lib::ssm::error::SsmError;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let aws = AwsClient::new().await?;
    let ssm = aws.ssm();
    match ssm.get_parameter("test-parameter").await {
        Ok(parameter) => {
            println!("Parameter: {}", parameter);
            Ok(())
        }
        Err(Error::Ssm(SsmError::InvalidResponse(name))) => {
            println!("Invalid response: {}", name);
            Err(Error::Ssm(SsmError::InvalidResponse(name)))
        }
        Err(Error::Ssm(SsmError::Timeout { request_id })) => {
            println!("Request timed out, request_id: {:?}", request_id);
            Err(Error::Ssm(SsmError::Timeout { request_id }))
        }
        Err(Error::Ssm(SsmError::Service(source))) => {
            println!("SSM service error: {}", source);
            Err(Error::Ssm(SsmError::Service(source)))
        }
        Err(Error::Ssm(e)) => {
            println!("Other SSM error: {}", e);
            Err(Error::Ssm(e))
        }
        Err(e) => {
            println!("Unknown error: {}", e);
            Err(e)
        }
    }
}

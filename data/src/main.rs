mod aws;
mod model;
mod sql;

#[tokio::main] // This attribute effectively makes your main function asynchronous
async fn main() {
    let _ = aws::check_aws_profile();
    if let Err(e) = aws::check_permissions().await {
        eprintln!("You do not have the right permissions: {}", e);
    }
    if let Err(e) = aws::check_aws_config().await {
        eprintln!("Failed to check AWS config: {}", e);
    }
    if let Err(e) = aws::check_s3_deep_glacier().await {
        eprintln!("Failed to upload test data to bucker: {}", e);
    }
}


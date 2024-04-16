mod aws;

#[tokio::main] // This attribute effectively makes your main function asynchronous
async fn main() {
    if let Err(e) = aws::check_permissions().await {
        eprintln!("You do not have the right permissions: {}", e);
    }
    if let Err(e) = aws::check_aws_config().await {
        eprintln!("Failed to check AWS config: {}", e);
    }
}


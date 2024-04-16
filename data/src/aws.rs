use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_types::region::Region;
use aws_sdk_s3::Client as S3Client;
use aws_config::SdkConfig;
use aws_config::profile::ProfileFileCredentialsProvider;
use std::error::Error;
use std::env;
use colored::*;

/// Checks for the AWS_PROFILE environment variable and guides the user if not set.
pub fn check_aws_profile() -> Result<String, String> {
    match env::var("AWS_PROFILE") {
        Ok(profile) => {
            println!("{} {}", "Using AWS profile:".green(), profile);
            Ok(profile)
        },
        Err(_) => {
            let error_message = "The AWS_PROFILE environment variable is not set.";
            println!("{}", error_message.red());
            println!("Please run {} to set up your AWS profile:", "aws configure --profile <profile-name>".yellow());
            println!("After setting up, you can run this application with {}.", "AWS_PROFILE=<profile-name>".yellow());
            Err("Missing AWS_PROFILE environment variable.".to_string())
        }
    }
}

// Function to create an AWS SDK configuration
async fn create_aws_config() -> Result<SdkConfig, Box<dyn Error>> {
    // Retrieve the AWS profile name from environment variables or default to "default"
    let profile_name = env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());

    let credentials_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(&profile_name)
        .build();

    // Since `default_region` is now an owned String, it can safely be used here
    let region_provider = RegionProviderChain::default_provider()
        .or_else(Region::new("us-east-1"));  // No lifetime issues now

    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(credentials_provider)
        .region(region_provider)
        .load()
        .await;

    Ok(config)
}

pub async fn check_permissions() -> Result<(), String> {
    // Create AWS configuration
    let config = create_aws_config().await
        .map_err(|e| e.to_string())?; 

    let s3_client = S3Client::new(&config);

    // Attempt to list buckets to check permissions
    match s3_client.list_buckets().send().await {
        Ok(_) => Ok(()),
        Err(e) => {
            // Interpret error message directly
            if e.to_string().contains("AccessDenied") {
                Err("Access denied: Insufficient permissions to list buckets.".to_string())
            } else {
                Err(format!("Failed to list buckets: {}", e))
            }
        },
    }
}

pub async fn check_aws_config() -> Result<(), String> {
    // Create AWS configuration
    let config = create_aws_config().await
        .map_err(|e| e.to_string())?;

    // Create an S3 client with the final configuration
    let s3_client = S3Client::new(&config);

    // Attempt to list buckets as a simple connection test
    s3_client.list_buckets().send().await
        .map_err(|err| format!("Error connecting to AWS: {}", err))?;

    Ok(())
}


use aws_config::meta::region::RegionProviderChain;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::BehaviorVersion;
use aws_config::SdkConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use aws_types::region::Region;
use aws_types::request_id::RequestId;
use colored::*;
use rand::Rng;
use scopeguard::defer;
use std::env;
use std::error::Error;
use std::time::Instant;
use tokio::task;
use uuid::Uuid;

/// Checks for the AWS_PROFILE environment variable and guides the user if not set.
pub fn check_aws_profile() -> Result<String, String> {
    match env::var("AWS_PROFILE") {
        Ok(profile) => {
            println!("{} {}", "Using AWS profile:".green(), profile);
            Ok(profile)
        }
        Err(_) => {
            let error_message = "The AWS_PROFILE environment variable is not set.";
            println!("{}", error_message.red());
            println!(
                "Please run {} to set up your AWS profile:",
                "aws configure --profile <profile-name>".yellow()
            );
            println!(
                "After setting up, you can run this application with {}.",
                "AWS_PROFILE=<profile-name>".yellow()
            );
            Err("Missing AWS_PROFILE environment variable.".to_string())
        }
    }
}

/// Function to create an AWS SDK configuration
async fn create_aws_config() -> Result<SdkConfig, Box<dyn Error>> {
    // Retrieve the AWS profile name from environment variables or default to "default"
    let profile_name = env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());

    let credentials_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(&profile_name)
        .build();

    // Since `default_region` is now an owned String, it can safely be used here
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1")); // No lifetime issues now

    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(credentials_provider)
        .region(region_provider)
        .load()
        .await;

    Ok(config)
}

/// Check the permissions of the aws user
pub async fn check_permissions() -> Result<(), String> {
    // Create AWS configuration
    let config = create_aws_config().await.map_err(|e| e.to_string())?;

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
        }
    }
}

/// Check the users aws config
pub async fn check_aws_config() -> Result<(), String> {
    // Create AWS configuration
    let config = create_aws_config().await.map_err(|e| e.to_string())?;

    // Create an S3 client with the final configuration
    let s3_client = S3Client::new(&config);

    // Attempt to list buckets as a simple connection test
    s3_client
        .list_buckets()
        .send()
        .await
        .map_err(|err| format!("Error connecting to AWS: {}", err))?;

    Ok(())
}

/// A connection and upload test function ensuring you can upload data.
pub async fn check_s3_deep_glacier() -> Result<bool, String> {
    // Create AWS configuration
    let config = create_aws_config().await.map_err(|e| e.to_string())?;

    // Create an S3 client
    let s3_client = S3Client::new(&config);

    // Create a test bucket name (you can use a different naming scheme if needed)
    let test_bucket = format!("lernspark-{}", Uuid::new_v4());

    // Create the test bucket
    let create_bucket_resp = s3_client
        .create_bucket()
        .bucket(&test_bucket)
        .send()
        .await
        .map_err(|err| format!("Error creating test bucket: {}", err))?;

    // Extract relevant information from the create_bucket_resp
    let bucket_location = create_bucket_resp
        .clone()
        .location
        .unwrap_or_else(|| "/".to_string());
    let request_id = create_bucket_resp.request_id().unwrap_or_else(|| "Unknown");

    println!(
        "{}",
        format!(
            "[AWS] Test bucket '{}' created successfully\nLocation: {}\nRequest ID: {}",
            test_bucket.green(),
            bucket_location.cyan(),
            request_id.yellow()
        )
    );

    // Get the bucket's location
    let get_bucket_location_resp = s3_client
        .get_bucket_location()
        .bucket(&test_bucket)
        .send()
        .await
        .map_err(|err| format!("Error retrieving bucket location: {}", err))?;

    // Check if the bucket's location supports the "DEEP_ARCHIVE" storage class
    let deep_glacier_available = match &get_bucket_location_resp.location_constraint {
        Some(location) => matches!(
            location.as_ref(),
            "us-east-1" | "us-east-2" | "us-west-2" | "eu-west-1"
        ),
        None => true, // Assume default location supports "DEEP_ARCHIVE"
    };

    // Upload random byte data to the S3 bucket using parallel processes
    let mut rng = rand::thread_rng();
    let num_uploads = rng.gen_range(5..=100);
    let data_sizes: Vec<usize> = (0..num_uploads)
        .map(|_| rng.gen_range(1..=16) * 1024 * 1024)
        .collect();

    // Declare the object_keys variable with explicit type
    let mut object_keys: Vec<String> = Vec::new();
    let mut upload_tasks = Vec::new();

    // Tell me how much data we are about to upload
    let total_data_size: usize = data_sizes.iter().sum();
    println!(
        "{}",
        format!(
            "Total data to be uploaded: {} MB",
            total_data_size / (1024 * 1024)
        )
        .yellow()
    );

    // Register the cleanup action using defer!
    defer! {
        let s3_client = s3_client.clone();
        let test_bucket = test_bucket.clone();
        let object_keys: Vec<String> = Vec::new();
        let object_keys_clone = object_keys.clone();
        tokio::spawn(async move {
            if let Err(e) = delete_objects_and_bucket(&s3_client, &test_bucket, &object_keys_clone).await {
                eprintln!("{}", "Error during cleanup:".red());
                eprintln!("{}", e.to_string().red());
            }
        });
    }

    // Start the timer for the entire upload process
    let start_time = Instant::now();

    for (index, data_size) in data_sizes.into_iter().enumerate() {
        let object_key = format!("random_data_{}mb.bin", data_size / (1024 * 1024));
        object_keys.push(object_key.clone());

        let s3_client_clone = s3_client.clone();
        let bucket_name_clone = test_bucket.clone();
        let upload_task = task::spawn(async move {
            println!(
                "{}",
                format!(
                    "Thread {} spawned for uploading {} MB of data.",
                    index.to_string().green(),
                    (data_size / (1024 * 1024)).to_string().cyan()
                )
            );

            upload_random_data(&s3_client_clone, &bucket_name_clone, &object_key, data_size).await
        });
        upload_tasks.push(upload_task);
    }

    // Wait for all upload tasks to complete
    for upload_task in upload_tasks {
        if let Err(err) = upload_task.await {
            return Err(format!("Error joining upload task: {}", err));
        }
    }

    // Calculate the total time taken for the upload process
    let total_time = start_time.elapsed();
    println!(
        "{}",
        format!("Total time taken for upload: {:?}", total_time).green()
    );

    // Clean up: Delete the test objects and bucket
    delete_objects_and_bucket(&s3_client, &test_bucket, &object_keys).await?;

    Ok(deep_glacier_available)
}

/// Function testing how to upload byte data to a bucket
pub async fn upload_random_data(
    s3_client: &aws_sdk_s3::Client,
    bucket_name: &str,
    object_key: &str,
    data_size: usize,
) -> Result<(), String> {
    // Generate random byte data
    let start_time = Instant::now();
    let random_data: Vec<u8> = (0..data_size).map(|_| rand::thread_rng().gen()).collect();
    let data_generation_time = start_time.elapsed();

    // Store the random byte data in the S3 bucket
    let start_time = Instant::now();
    let put_object_resp = s3_client
        .put_object()
        .bucket(bucket_name)
        .key(object_key)
        .body(ByteStream::from(random_data))
        .send()
        .await;
    let upload_time = start_time.elapsed();

    match put_object_resp {
        Ok(_) => {
            let upload_speed = data_size as f64 / upload_time.as_secs_f64() / (1024.0 * 1024.0);
            println!(
                "{}",
                format!(
                    "Random byte data (~{}MB) stored in object '{}' in bucket '{}'\n\
                     Data generation time: {:?}\n\
                     Upload time: {:?}\n\
                     Upload speed: {:.2} MB/s",
                    data_size / (1024 * 1024),
                    object_key.green(),
                    bucket_name.cyan(),
                    data_generation_time,
                    upload_time,
                    upload_speed
                )
            );
            Ok(())
        }
        Err(err) => Err(format!(
            "Error storing random byte data in bucket '{}': {}",
            bucket_name, err
        )),
    }
}

pub async fn delete_objects_and_bucket(
    s3_client: &aws_sdk_s3::Client,
    bucket_name: &str,
    object_keys: &[String],
) -> Result<(), String> {
    // Delete the objects from the bucket
    for object_key in object_keys {
        let delete_object_resp = s3_client
            .delete_object()
            .bucket(bucket_name)
            .key(object_key)
            .send()
            .await;

        match delete_object_resp {
            Ok(_) => {
                println!(
                    "{}",
                    format!(
                        "Object '{}' deleted successfully from bucket '{}'",
                        object_key.green(),
                        bucket_name.cyan()
                    )
                );
            }
            Err(err) => {
                return Err(format!(
                    "Error deleting object '{}' from bucket '{}': {}",
                    object_key, bucket_name, err
                ));
            }
        }
    }

    // Delete the bucket
    let delete_bucket_resp = s3_client.delete_bucket().bucket(bucket_name).send().await;

    match delete_bucket_resp {
        Ok(_) => {
            println!(
                "{}",
                format!("Bucket '{}' deleted successfully", bucket_name.cyan())
            );
            Ok(())
        }
        Err(err) => Err(format!("Error deleting bucket '{}': {}", bucket_name, err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_s3_upload() {
        if let Err(e) = check_s3_deep_glacier().await {
            eprintln!("Failed to upload test data to bucker: {}", e);
        }
    }
}

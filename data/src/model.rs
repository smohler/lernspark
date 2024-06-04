//! Data model based off data.model created by the user
//! during initialization of lernspark
use chrono::NaiveDate;
use colored::Colorize;
use fake::faker::{
    address::en::*, color::en::*, company::en::*, creditcard::en::*, internet::en::*, job::en::*,
    lorem::en::*, name::en::*, number::en::*, phone_number::en::*,
};
use fake::Fake;
use rand::Rng;
use regex::Regex;

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use tar::Builder;

use flate2::write::GzEncoder;
use flate2::Compression;

use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use parquet::file::reader::FileReader;
use parquet::file::reader::SerializedFileReader;
use parquet::record::RowAccessor;

use arrow::array::{ArrayRef, BooleanArray, Date32Array, Float32Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema}; // Make sure this Schema is from the correct crate
use arrow::record_batch::RecordBatch; // Ensure this is the RecordBatch expected by parquet

use crate::sql::{self, DataType as SqlDataType, Table};

/// Load Schema from data.sql
pub fn load_data_model() -> Vec<Table> {
    // Get the crate root directory
    let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Define the path to data.sql starting from root
    let data_sql_path = Path::new(&crate_root).join("data.sql");
    let data_sql_content = std::fs::read_to_string(&data_sql_path).expect("Unable to read data.sql file");
    sql::parse_sql_file(&data_sql_content)
}

pub fn generate_sandbox_example_random_files(tables: &Vec<Table>) {
    // Get the local downloads directory
    let downloads_dir = dirs::download_dir().unwrap();
    // Create a new tar.gz archive in downloads directory
    let tar_gz_file_path = downloads_dir.join("examples.tar.gz");
    // Create a new file for the tar.gz archive
    let tar_gz_file = File::create(&tar_gz_file_path).unwrap();
    // Create a new GzEncoder with the tar.gz file
    let mut gz_encoder = GzEncoder::new(tar_gz_file, Compression::default());

    {
        // Create a new tar archive
        let mut tar_builder = Builder::new(&mut gz_encoder);
        // Iterate over each table and create a random Parquet file
        for table in tables {
            let file_path = format!("{}.parquet", table.name);
            // Create a random Parquet file for the table
            create_random_parquet_from_datasql(&file_path, table);
            // Add the Parquet file to the tar archive
            tar_builder
                .append_file(&file_path, &mut File::open(&file_path).unwrap())
                .unwrap();
            // Remove the individual Parquet file
            std::fs::remove_file(&file_path).unwrap();
        }
        // Finish writing the tar archive
        tar_builder.finish().unwrap();
    } // The tar_builder is dropped here, ending the mutable borrow

    // Finish writing the GzEncoder
    gz_encoder.try_finish().unwrap();
    // Create a string representation of the tar.gz file path
    let tar_gz_file_path_str = tar_gz_file_path.to_str().unwrap();
    println!(
        "{}",
        format!(
            "🎉 Fantastic example Parquet files have been generated and compressed into '{}'!",
            &tar_gz_file_path_str.bold().green()
        )
    );
}

/// Mapping between SQL and Parquet structs
fn map_sql_to_arrow_type(sql_type: &SqlDataType) -> DataType {
    match sql_type {
        SqlDataType::Int(_) => DataType::Int32,
        SqlDataType::Float(_) => DataType::Float32,
        SqlDataType::String(_) => DataType::Utf8,
        SqlDataType::DateTime(_) => DataType::Date32,
        SqlDataType::UUID(_) => DataType::Utf8,
        SqlDataType::Boolean(_) => DataType::Boolean,
    }
}

/// Function to create a random Parquet data file using Arrow and Parquet APIs
fn create_random_parquet_file(file_path: &str) {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("age", DataType::Int32, false),
    ]));

    let file = File::create(Path::new(file_path)).unwrap();
    let props = WriterProperties::builder().build();

    // Make sure to pass the properties wrapped in Some to match the expected Option type
    let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(props)).unwrap();

    let mut ids = Vec::new();
    let mut names = Vec::new();
    let mut ages = Vec::new();

    for i in 0..100 {
        ids.push(i);
        names.push(format!("Person {}", i));
        ages.push(rand::thread_rng().gen_range(18..=65));
    }

    let id_array: ArrayRef = Arc::new(Int32Array::from(ids));
    let name_array: ArrayRef = Arc::new(StringArray::from(names));
    let age_array: ArrayRef = Arc::new(Int32Array::from(ages));

    let record_batch =
        RecordBatch::try_new(schema.clone(), vec![id_array, name_array, age_array]).unwrap();

    // Ensure the write method receives the correct RecordBatch type
    writer.write(&record_batch).unwrap();
    writer.close().unwrap();
}

/// Generates fake data based on the column name.
fn generate_fake_string_data(col_name: &str, num_rows: usize) -> Vec<String> {
    let mut data = Vec::with_capacity(num_rows);

    // Compile regex once to avoid recompilation in the loop.
    let name_regex = Regex::new(r"(?i)name|fullname|username").expect("Invalid regex for name");
    let email_regex = Regex::new(r"(?i)email").expect("Invalid regex for email");
    let address_regex = Regex::new(r"(?i)address|street|city|state|country|zip|postal")
        .expect("Invalid regex for address");
    let company_regex =
        Regex::new(r"(?i)company|industry|buzzword|business").expect("Invalid regex for company");
    let internet_regex = Regex::new(r"(?i)domain|ip|mac").expect("Invalid regex for internet");
    let payment_regex = Regex::new(r"(?i)credit|card|number").expect("Invalid regex for payment");
    let phone_regex = Regex::new(r"(?i)phone").expect("Invalid regex for phone");
    let color_regex = Regex::new(r"(?i)color|rgb|hex").expect("Invalid regex for color");
    let time_regex = Regex::new(r"(?i)time|zone").expect("Invalid regex for time");
    let job_regex =
        Regex::new(r"(?i)job|field|position|seniority|title").expect("Invalid regex for job");
    let lorem_regex = Regex::new(r"(?i)tweet|text|post|comment|review|paragraph|sentence|word")
        .expect("Invalid regex for lorem");

    for _ in 0..num_rows {
        if name_regex.is_match(col_name) {
            let name: String = Name().fake();
            data.push(name);
        } else if email_regex.is_match(col_name) {
            let email: String = SafeEmail().fake();
            data.push(email);
        } else if address_regex.is_match(col_name) {
            let address: String = match col_name.to_lowercase().as_str() {
                _ if col_name.to_lowercase().as_str().contains("street") => StreetName().fake(),
                _ if col_name.to_lowercase().as_str().contains("city") => CityName().fake(),
                _ if col_name.to_lowercase().as_str().contains("state") => StateName().fake(),
                _ if col_name.to_lowercase().as_str().contains("country") => CountryName().fake(),
                _ if col_name.to_lowercase().as_str().contains("zip") => ZipCode().fake(),
                _ if col_name.to_lowercase().as_str().contains("postal") => PostCode().fake(),
                _ => {
                    let address: String = NumberWithFormat("####").fake();
                    let street_address: String = StreetName().fake();
                    let secondary_address: String = SecondaryAddress().fake();
                    let city: String = CityName().fake();
                    let zip_code: String = ZipCode().fake();
                    let country: String = CountryName().fake();
                    format!(
                        "{} {}, {}, {}, {}, {}",
                        address, street_address, secondary_address, city, zip_code, country
                    )
                }
            };
            data.push(address);
        } else if company_regex.is_match(col_name) {
            let company: String = match col_name.to_lowercase().as_str() {
                _ if col_name.to_lowercase().as_str().contains("company") => CompanyName().fake(),
                _ if col_name.to_lowercase().as_str().contains("buisness") => CompanyName().fake(),
                _ if col_name.to_lowercase().as_str().contains("industry") => Industry().fake(),
                _ if col_name.to_lowercase().as_str().contains("buzzword") => Buzzword().fake(),
                _ => CompanySuffix().fake(),
            };
            data.push(company);
        } else if internet_regex.is_match(col_name) {
            let internet: String = match col_name.to_lowercase().as_str() {
                _ if col_name.to_lowercase().as_str().contains("domain") => DomainSuffix().fake(),
                _ if col_name.to_lowercase().as_str().contains("ip") => IPv4().fake(),
                _ if col_name.to_lowercase().as_str().contains("mac") => MACAddress().fake(),
                _ => Username().fake(),
            };
            data.push(internet);
        } else if payment_regex.is_match(col_name) {
            let payment: String = CreditCardNumber().fake();
            data.push(payment);
        } else if phone_regex.is_match(col_name) {
            let phone: String = PhoneNumber().fake();
            data.push(phone);
        } else if color_regex.is_match(col_name) {
            let color: String = match col_name.to_lowercase().as_str() {
                _ if col_name.to_lowercase().as_str().contains("rgb") => RgbColor().fake(),
                _ if col_name.to_lowercase().as_str().contains("hex") => HexColor().fake(),
                _ => Color().fake(),
            };
            data.push(color);
        } else if time_regex.is_match(col_name) {
            let time: String = match col_name.to_lowercase().as_str() {
                _ if col_name.to_lowercase().as_str().contains("zone") => TimeZone().fake(),
                _ => Word().fake(), // Fallback to random string
            };
            data.push(time);
        } else if job_regex.is_match(col_name) {
            let job: String = match col_name.to_lowercase().as_str() {
                _ if col_name.to_lowercase().as_str().contains("field") => Field().fake(),
                _ if col_name.to_lowercase().as_str().contains("position") => Position().fake(),
                _ if col_name.to_lowercase().as_str().contains("seniority") => Seniority().fake(),
                _ => Word().fake(), // Fallback to random string
            };
            data.push(job);
        } else if lorem_regex.is_match(col_name) {
            let lorem: String = match col_name.to_lowercase().as_str() {
                _ if col_name.to_lowercase().as_str().contains("paragraph")
                    || col_name.to_lowercase().as_str().contains("review")
                    || col_name.to_lowercase().as_str().contains("post") =>
                {
                    Paragraph(3..10).fake()
                }
                _ if col_name.to_lowercase().as_str().contains("sentence")
                    || col_name.to_lowercase().as_str().contains("tweet")
                    || col_name.to_lowercase().as_str().contains("comment") =>
                {
                    Sentence(5..15).fake()
                }
                _ if col_name.to_lowercase().as_str().contains("word") => Word().fake(),
                _ => Word().fake(),
            };
            data.push(lorem);
        } else {
            // Generate random string for other cases
            let random_string: String = Word().fake();
            data.push(random_string);
        }
    }

    data
}

/// Function to create a random Parquet data file using Arrow and Parquet APIs
fn create_random_parquet_from_datasql(file_path: &str, table: &Table) {
    let schema = Arc::new(Schema::new(
        table
            .columns
            .iter()
            .map(|col| Field::new(&col.name, map_sql_to_arrow_type(&col.data_type), true))
            .collect(),
    ));

    let file = File::create(Path::new(file_path)).unwrap();
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(props)).unwrap();

    let num_rows = rand::thread_rng().gen_range(1000..=100000);
    let mut arrays: Vec<ArrayRef> = Vec::new();
    println!(
        "{}",
        format!(
            "🚀 Generating a stunning Parquet file with {} rows for table '{}'...",
            num_rows.to_string().bold().cyan(),
            table.name.bold().yellow()
        )
    );

    for col in &table.columns {
        match &col.data_type {
            SqlDataType::Int(_) => {
                let mut data = Vec::with_capacity(num_rows);
                for _ in 0..num_rows {
                    data.push(rand::thread_rng().gen_range(0..=100));
                }
                arrays.push(Arc::new(Int32Array::from(data)));
            }
            SqlDataType::Float(_) => {
                let mut data = Vec::with_capacity(num_rows);
                for _ in 0..num_rows {
                    data.push(rand::thread_rng().gen_range(0.0..=100.0));
                }
                arrays.push(Arc::new(Float32Array::from(data)));
            }
            SqlDataType::String(_) => {
                let fake_data = generate_fake_string_data(&col.name, num_rows);
                arrays.push(Arc::new(StringArray::from(fake_data)));
            }
            SqlDataType::DateTime(_) => {
                let mut data = Vec::with_capacity(num_rows);
                for _ in 0..num_rows {
                    data.push(rand::thread_rng().gen_range(0..=100));
                }
                arrays.push(Arc::new(Date32Array::from(data)));
            }
            SqlDataType::UUID(_) => {
                let mut data = Vec::with_capacity(num_rows);
                for _ in 0..num_rows {
                    data.push(uuid::Uuid::new_v4().to_string());
                }
                arrays.push(Arc::new(StringArray::from(data)));
            }
            SqlDataType::Boolean(_) => {
                let mut data = Vec::with_capacity(num_rows);
                for _ in 0..num_rows {
                    data.push(rand::thread_rng().gen_bool(0.5));
                }
                arrays.push(Arc::new(BooleanArray::from(data)));
            }
        }
    }

    let record_batch = RecordBatch::try_new(schema.clone(), arrays).unwrap();
    writer.write(&record_batch).unwrap();
    writer.close().unwrap();
    println!(
        "{}",
        format!(
            "✅ Parquet file '{}' has been brilliantly created with {} rows!",
            file_path.bold().green(),
            num_rows.to_string().bold().cyan()
        )
    );
    println!("🔍 Example values from the Parquet file:");
    print_example_values(file_path, &schema, 1);
}

fn print_example_values(file_path: &str, schema: &Schema, num_examples: usize) {
    let file = File::open(file_path).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();
    let mut iter = reader.get_row_iter(None).unwrap();

    for _ in 0..num_examples {
        if let Some(row) = iter.next() {
            for i in 0..row.len() {
                let column_name = schema.field(i).name();
                let value = match schema.field(i).data_type() {
                    DataType::Int32 => row.get_int(i).map(|v| v.to_string()).unwrap_or_default(),
                    DataType::Float32 => {
                        row.get_float(i).map(|v| v.to_string()).unwrap_or_default()
                    }
                    DataType::Utf8 => row.get_string(i).map(|v| v.to_string()).unwrap_or_default(),
                    DataType::Date32 => {
                        let date = row.get_int(i).unwrap_or_default();
                        NaiveDate::from_num_days_from_ce_opt(date as i32)
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_default()
                    }
                    DataType::Boolean => row.get_bool(i).map(|v| v.to_string()).unwrap_or_default(),
                    _ => "Unsupported data type".to_string(),
                };
                println!("  {}: {}", column_name.bold().cyan(), value.yellow());
            }
            println!();
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_random_parquet_file() {
        let file_path = "temp_random_data.parquet";
        create_random_parquet_file(file_path);

        // Check if the file was created
        assert!(Path::new(file_path).exists());

        // Clean up
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_create_random_parquet_files_from_datasql() {
        let tables = load_data_model();

        for table in &tables {
            let file_path = format!("{}_{}.parquet", table.name, uuid::Uuid::new_v4());
            create_random_parquet_from_datasql(&file_path, table);

            // Check if the file was created
            assert!(Path::new(&file_path).exists());

            // Clean up
            std::fs::remove_file(&file_path).unwrap();
        }
    }

    #[test]
    fn test_creating_random_parquet_files_and_tar_from_datasql() {
        let tables = load_data_model();
        generate_sandbox_example_random_files(&tables);

        // Check that example.tar.gz exists in the user downloads directory
        // Get the local downloads directory
        let downloads_dir = dirs::download_dir().unwrap();
        let tar_file_path = downloads_dir.join("examples.tar.gz");
        assert!(Path::new(&tar_file_path).exists());

        // Clean up
        std::fs::remove_file(tar_file_path).unwrap();
    }
}

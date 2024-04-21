//! Data model based off data.model created by the user
//! during initialization of lernspark
use colored::Colorize;
use chrono::NaiveDate;
use rand::Rng;
use fake::{Fake, Faker};
use fake::faker::name::en::Name;
use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::Word;
use regex::Regex;

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use parquet::file::reader::SerializedFileReader;
use parquet::file::reader::FileReader;
use parquet::record::RowAccessor;

use arrow::array::{ArrayRef, BooleanArray, Date32Array, Float32Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema}; // Make sure this Schema is from the correct crate
use arrow::record_batch::RecordBatch; // Ensure this is the RecordBatch expected by parquet

use crate::sql::{self, DataType as SqlDataType, Table};

/// Load Schema from data.sql
fn load_data_model() -> Vec<Table> {
    let data_sql_content = std::fs::read_to_string("./data.sql").expect("Unable to read data.sql file");
    sql::parse_sql_file(&data_sql_content)
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
    let name_regex = Regex::new(r"name|fullname|username").expect("Invalid regex for name");
    let email_regex = Regex::new(r"email").expect("Invalid regex for email");

    for _ in 0..num_rows {
        if name_regex.is_match(col_name) {
            let name: String = Name().fake();
            data.push(name);
        } else if email_regex.is_match(col_name) {
            let email: String = SafeEmail().fake();
            data.push(email);
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

    let num_rows = rand::thread_rng().gen_range(100000..=10000000);
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
                    DataType::Float32 => row.get_float(i).map(|v| v.to_string()).unwrap_or_default(),
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
}

//! Data model based off data.model created by the user
//! during initialization of lernspark

use std::sync::Arc; use std::fs::File;
use std::path::Path;
use rand::Rng;

use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

use arrow::datatypes::{Schema, Field, DataType}; // Make sure this Schema is from the correct crate
use arrow::record_batch::RecordBatch; // Ensure this is the RecordBatch expected by parquet
use arrow::array::{Int32Array, StringArray, ArrayRef};

/// Load Schema from data.model
fn load_data_model(){
    // data.model is located in the crate root
    todo!();
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

    let record_batch = RecordBatch::try_new(schema.clone(), vec![id_array, name_array, age_array]).unwrap();

    // Ensure the write method receives the correct RecordBatch type
    writer.write(&record_batch).unwrap();
    writer.close().unwrap();
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
}


use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
enum DataType {
    Int,
    Float,
    String,
    DateTime(NaiveDateTime),
    UUID(Uuid),
    // Add more data types as needed
}

#[derive(Debug, PartialEq)]
struct Column {
    name: String,
    data_type: DataType,
    constraints: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct Table {
    name: String,
    columns: Vec<Column>,
}

fn parse_create_table(statement: &str) -> Table {
    let table_name = extract_table_name(statement);
    let columns = extract_columns(statement);

    Table {
        name: table_name,
        columns,
    }
}

fn extract_table_name(statement: &str) -> String {
    statement
        .split_whitespace()
        .nth(2)
        .unwrap_or("")
        .trim_end_matches('(')
        .to_string()
}

fn extract_columns(statement: &str) -> Vec<Column> {
    statement
        .split('(')
        .nth(1)
        .unwrap_or("")
        .trim_end_matches(')')
        .trim_end_matches(';')
        .split(',')
        .map(|col| col.trim())
        .filter(|col| !col.is_empty())
        .map(|col| {
            let parts: Vec<&str> = col.split_whitespace().collect();
            let name = parts[0].to_string();
            let data_type = match parts[1].to_uppercase().as_str() {
                "INT" => DataType::Int,
                "FLOAT" => DataType::Float,
                "VARCHAR" | "TEXT" => DataType::String,
                "DATE" | "DATETIME" => {
                    let datetime = NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
                    DataType::DateTime(datetime)
                }
                "UUID" => {
                    let uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
                    DataType::UUID(uuid)
                }
                // Add more data type mappings as needed
                _ => panic!("Unsupported data type: {}", parts[1]),
            };
            let constraints = parts[2..]
                .iter()
                .flat_map(|c| c.split_whitespace())
                .map(|c| c.trim_end_matches(')').trim_end_matches(';').to_string())
                .filter(|c| !c.is_empty())
                .collect();
            Column {
                name,
                data_type,
                constraints,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_create_table() {
        let sql = "CREATE TABLE Foo (
            ID INT AUTO_INCREMENT PRIMARY KEY,
            Bar DATE UNIQUE NOT NULL,
            Jar FLOAT,
            Baz UUID,
            Qux DATETIME
        );";

        let expected_table = Table {
            name: "Foo".to_string(),
            columns: vec![
                Column {
                    name: "ID".to_string(),
                    data_type: DataType::Int,
                    constraints: vec!["AUTO_INCREMENT".to_string(), "PRIMARY".to_string(), "KEY".to_string()],
                },
                Column {
                    name: "Bar".to_string(),
                    data_type: DataType::DateTime(NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
                    constraints: vec!["UNIQUE".to_string(), "NOT".to_string(), "NULL".to_string()],
                },
                Column {
                    name: "Jar".to_string(),
                    data_type: DataType::Float,
                    constraints: vec![],
                },
                Column {
                    name: "Baz".to_string(),
                    data_type: DataType::UUID(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()),
                    constraints: vec![],
                },
                Column {
                    name: "Qux".to_string(),
                    data_type: DataType::DateTime(NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
                    constraints: vec![],
                },
            ],
        };

        let parsed_table = parse_create_table(sql);
        assert_eq!(parsed_table, expected_table);
    }
}

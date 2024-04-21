use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
enum DataType {
    Int(u32),
    Float(f32),
    String(String),
    DateTime(NaiveDateTime),
    UUID(Uuid),
    Boolean(bool),
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
                "INT" => DataType::Int(0),       // Use a default value of 0 for u32
                "FLOAT" => DataType::Float(0.0), // Use a default value of 0.0 for f32
                "VARCHAR(255)" | "TEXT" => DataType::String("".to_string()), // Use an empty string for String
                "DATE" | "DATETIME" => {
                    let datetime =
                        NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                            .unwrap();
                    DataType::DateTime(datetime)
                }
                "UUID" => {
                    let uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
                    DataType::UUID(uuid)
                }
                "BOOLEAN" => DataType::Boolean(false), // Use a default value of false for bool
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

/// Parse the data.sql into the tables
fn parse_sql_file(sql_content: &str) -> Vec<Table> {
    let lines: Vec<&str> = sql_content.lines().collect();
    let sql_content = if lines[0].trim_start().starts_with("--") {
        lines[1..].join("\n")
    } else {
        sql_content.to_string()
    };

    let statements: Vec<&str> = sql_content
        .split(";")
        .map(|stmt| stmt.trim())
        .filter(|stmt| !stmt.is_empty())
        .collect();

    statements
        .into_iter()
        .map(|stmt| parse_create_table(stmt))
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
                    data_type: DataType::Int(0),
                    constraints: vec![
                        "AUTO_INCREMENT".to_string(),
                        "PRIMARY".to_string(),
                        "KEY".to_string(),
                    ],
                },
                Column {
                    name: "Bar".to_string(),
                    data_type: DataType::DateTime(
                        NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                            .unwrap(),
                    ),
                    constraints: vec!["UNIQUE".to_string(), "NOT".to_string(), "NULL".to_string()],
                },
                Column {
                    name: "Jar".to_string(),
                    data_type: DataType::Float(0.0),
                    constraints: vec![],
                },
                Column {
                    name: "Baz".to_string(),
                    data_type: DataType::UUID(
                        Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
                    ),
                    constraints: vec![],
                },
                Column {
                    name: "Qux".to_string(),
                    data_type: DataType::DateTime(
                        NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                            .unwrap(),
                    ),
                    constraints: vec![],
                },
            ],
        };

        let parsed_table = parse_create_table(sql);
        assert_eq!(parsed_table, expected_table);
    }

    #[test]
    fn test_parse_sql_file() {
        let sql_content = r#"-- SQL Database Schema
    CREATE TABLE Lanisters (
        ID INT AUTO_INCREMENT PRIMARY KEY,
        King TEXT UNIQUE,
        Army INT NOT NULL
    );

    CREATE TABLE Starks (
        ID INT AUTO_INCREMENT PRIMARY KEY,
        King TEXT NOT NULL,
        Army INT NOT NULL,
        IS_TRUE_KING BOOLEAN
    );
    "#;

        let expected_tables = vec![
            Table {
                name: "Lanisters".to_string(),
                columns: vec![
                    Column {
                        name: "ID".to_string(),
                        data_type: DataType::Int(0),
                        constraints: vec![
                            "AUTO_INCREMENT".to_string(),
                            "PRIMARY".to_string(),
                            "KEY".to_string(),
                        ],
                    },
                    Column {
                        name: "King".to_string(),
                        data_type: DataType::String(String::new()),
                        constraints: vec!["UNIQUE".to_string()],
                    },
                    Column {
                        name: "Army".to_string(),
                        data_type: DataType::Int(0),
                        constraints: vec!["NOT".to_string(), "NULL".to_string()],
                    },
                ],
            },
            Table {
                name: "Starks".to_string(),
                columns: vec![
                    Column {
                        name: "ID".to_string(),
                        data_type: DataType::Int(0),
                        constraints: vec![
                            "AUTO_INCREMENT".to_string(),
                            "PRIMARY".to_string(),
                            "KEY".to_string(),
                        ],
                    },
                    Column {
                        name: "King".to_string(),
                        data_type: DataType::String(String::new()),
                        constraints: vec!["NOT".to_string(), "NULL".to_string()],
                    },
                    Column {
                        name: "Army".to_string(),
                        data_type: DataType::Int(0),
                        constraints: vec!["NOT".to_string(), "NULL".to_string()],
                    },
                    Column {
                        name: "IS_TRUE_KING".to_string(),
                        data_type: DataType::Boolean(false),
                        constraints: vec![],
                    },
                ],
            },
        ];

        let parsed_tables = parse_sql_file(sql_content);
        assert_eq!(parsed_tables, expected_tables);
    }
}

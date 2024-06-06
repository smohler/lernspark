use chrono::NaiveDateTime;
use regex::Regex;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int(u32),
    Float(f32),
    String(String),
    DateTime(NaiveDateTime),
    UUID(Uuid),
    Boolean(bool),
    VarChar(usize),
}

#[derive(Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub constraints: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

/// Parse the data.sql into the tables
pub fn parse_sql_file(sql_content: &str) -> Vec<Table> {
    let lines: Vec<&str> = sql_content.lines().collect();
    let sql_content = if lines.get(0).map(|line| line.trim_start().starts_with("--")).unwrap_or(false) {
        lines[1..].join("\n")
    } else {
        sql_content.to_string()
    };

    let statements: Vec<&str> = sql_content
        .split(';')
        .map(|stmt| stmt.trim())
        .filter(|stmt| !stmt.is_empty())
        .collect();

    statements
        .into_iter()
        .map(|stmt| parse_create_table(stmt))
        .collect()
}

pub fn parse_create_table(statement: &str) -> Table {
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
    let column_definitions = statement
        .split_once('(')
        .and_then(|(_, cols)| cols.rsplit_once(')').map(|(cols, _)| cols))
        .unwrap_or("");

    let re = Regex::new(r"(?m)\s*([^,]+(?:\([^)]*\))?[^,]*)(?:,|$)").unwrap();
    re.captures_iter(column_definitions)
        .filter_map(|cap| {
            let col_def = cap.get(1).unwrap().as_str().trim();
            if col_def.is_empty() {
                None
            } else {
                Some(parse_column(col_def))
            }
        })
        .collect()
}

fn parse_column(col_def: &str) -> Column {
    let parts: Vec<&str> = col_def.split_whitespace().collect();
    let name = parts[0].to_string();
    let data_type_str = parts[1..].join(" ");

    let mut data_type = DataType::String(String::new());
    let mut constraints = vec![];

    if let Some((dt, cn)) = parse_data_type_and_constraints(&data_type_str) {
        data_type = dt;
        constraints = cn;
    }

    Column {
        name,
        data_type,
        constraints,
    }
}

fn parse_data_type_and_constraints(input: &str) -> Option<(DataType, Vec<String>)> {
    let re = Regex::new(r"(?i)^(VARCHAR\((\d+)\)|INT|FLOAT|TEXT|DATE|DATETIME|UUID|BOOLEAN)\s*(.*)$").unwrap();

    re.captures(input).map(|cap| {
        let data_type = match &cap[1].to_uppercase()[..] {
            "INT" => DataType::Int(0),
            "FLOAT" => DataType::Float(0.0),
            "TEXT" => DataType::String(String::new()),
            "DATE" | "DATETIME" => DataType::DateTime(
                NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
            ),
            "UUID" => DataType::UUID(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()),
            "BOOLEAN" => DataType::Boolean(false),
            varchar if varchar.starts_with("VARCHAR") => {
                let len = cap[2].parse::<usize>().unwrap_or(255);
                DataType::VarChar(len)
            },
            _ => panic!("Unsupported data type: {}", &cap[1]),
        };

        let constraints = cap[3].split_whitespace().map(|c| c.to_string()).collect();

        (data_type, constraints)
    })
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
        Qux DATETIME,
        Name VARCHAR(255)
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
                        NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
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
                        NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                    ),
                    constraints: vec![],
                },
                Column {
                    name: "Name".to_string(),
                    data_type: DataType::VarChar(255),
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
        Army INT NOT NULL,
        Alias VARCHAR(100)
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
                    Column {
                        name: "Alias".to_string(),
                        data_type: DataType::VarChar(100),
                        constraints: vec![],
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


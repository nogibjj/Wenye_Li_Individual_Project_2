// lib.rs
use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::Local;

const DB_FILE: &str = "DrugUseDB.db";
const LOG_FILE: &str = "db_operations.md";

// Helper function for logging
fn log_operation(operation: &str, details: &str) -> Result<(), Box<dyn Error>> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("### {}\n**Operation:** {}\n**Details:** {}\n\n", timestamp, operation, details);
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)?;
    
    file.write_all(log_entry.as_bytes())?;
    Ok(())
}

#[derive(Debug)]
struct DataRow(
    String, i32, f64, f64, f64, f64, Option<f64>, Option<f64>, Option<f64>, Option<f64>, 
    Option<f64>, Option<f64>, f64, f64, f64, Option<f64>, f64, f64, f64, Option<f64>, 
    f64, f64, f64, f64, Option<f64>, Option<f64>, f64, f64
);

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    log_operation("Extract", &format!("Downloading file from {}", url))?;
    
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let response = get(url)?;
    if response.status().is_success() {
        let mut file = File::create(file_path)?;
        file.write_all(&response.bytes()?)?;
        log_operation("Extract", &format!("File successfully downloaded to {}", file_path))?;
    } else {
        let error_msg = format!("Failed to retrieve the file from {}", url);
        log_operation("Extract Error", &error_msg)?;
        return Err(error_msg.into());
    }

    Ok(())
}

pub fn load(dataset: &str) -> Result<(), Box<dyn Error>> {
    log_operation("Load", &format!("Loading data from {}", dataset))?;

    let mut conn = Connection::open(DB_FILE)?;
    conn.execute("DROP TABLE IF EXISTS DrugUse", [])?;
    
    // Create table
    conn.execute(
        "CREATE TABLE DrugUse (
            age TEXT,
            n INTEGER,
            alcohol_use REAL,
            alcohol_frequency REAL,
            marijuana_use REAL,
            marijuana_frequency REAL,
            cocaine_use REAL,
            cocaine_frequency REAL,
            crack_use REAL,
            crack_frequency REAL,
            heroin_use REAL,
            heroin_frequency REAL,
            hallucinogen_use REAL,
            hallucinogen_frequency REAL,
            inhalant_use REAL,
            inhalant_frequency REAL,
            pain_releiver_use REAL,
            pain_releiver_frequency REAL,
            oxycontin_use REAL,
            oxycontin_frequency REAL,
            tranquilizer_use REAL,
            tranquilizer_frequency REAL,
            stimulant_use REAL,
            stimulant_frequency REAL,
            meth_use REAL,
            meth_frequency REAL,
            sedative_use REAL,
            sedative_frequency REAL
        )",
        [],
    )?;

    let file = File::open(dataset)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(file);

    let tx = conn.transaction()?;
    let mut row_count = 0;
    for result in rdr.records() {
        let record = result?;
        tx.execute(
            "INSERT INTO DrugUse VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &record[0],
                record[1].parse::<i32>()?,
                record[2].parse::<f64>()?,
                record[3].parse::<f64>()?,
                record[4].parse::<f64>()?,
                record[5].parse::<f64>()?,
                parse_float(&record[6])?,
                parse_float(&record[7])?,
                parse_float(&record[8])?,
                parse_float(&record[9])?,
                parse_float(&record[10])?,
                parse_float(&record[11])?,
                record[12].parse::<f64>()?,
                record[13].parse::<f64>()?,
                record[14].parse::<f64>()?,
                parse_float(&record[15])?,
                record[16].parse::<f64>()?,
                record[17].parse::<f64>()?,
                record[18].parse::<f64>()?,
                parse_float(&record[19])?,
                record[20].parse::<f64>()?,
                record[21].parse::<f64>()?,
                record[22].parse::<f64>()?,
                record[23].parse::<f64>()?,
                parse_float(&record[24])?,
                parse_float(&record[25])?,
                record[26].parse::<f64>()?,
                record[27].parse::<f64>()?
            ],
        )?;
        row_count += 1;
    }
    tx.commit()?;
    
    log_operation("Load", &format!("Successfully loaded {} rows into database", row_count))?;
    Ok(())
}

fn parse_float(value: &str) -> Result<Option<f64>, Box<dyn Error>> {
    if value == "-" {
        Ok(None)
    } else {
        Ok(Some(value.parse::<f64>()?))
    }
}

pub fn insert_row(data: (String, i32, f64, f64, f64, f64, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, f64, f64, f64, Option<f64>, f64, f64, f64, Option<f64>, f64, f64, f64, f64, Option<f64>, Option<f64>, f64, f64)) -> Result<(), Box<dyn Error>> {
    log_operation("Insert", &format!("Inserting new row for age group: {}", data.0))?;

    let conn = Connection::open(DB_FILE)?;
    conn.execute(
        "INSERT INTO DrugUse VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            data.0, data.1, data.2, data.3, data.4, data.5, data.6, data.7, data.8, data.9, 
            data.10, data.11, data.12, data.13, data.14, data.15, data.16, data.17, data.18, 
            data.19, data.20, data.21, data.22, data.23, data.24, data.25, data.26, data.27
        ],
    )?;
    
    log_operation("Insert", "Row inserted successfully")?;
    Ok(())
}

pub fn select_rows() -> Result<(), Box<dyn Error>> {
    log_operation("Select", "Retrieving all rows from database")?;

    let conn = Connection::open(DB_FILE)?;
    let mut stmt = conn.prepare("SELECT * FROM DrugUse")?;
    
    let rows = stmt.query_map([], |row| {
        Ok(DataRow(
            row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, 
            row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?, row.get(9)?, 
            row.get(10)?, row.get(11)?, row.get(12)?, row.get(13)?, row.get(14)?, 
            row.get(15)?, row.get(16)?, row.get(17)?, row.get(18)?, row.get(19)?, 
            row.get(20)?, row.get(21)?, row.get(22)?, row.get(23)?, row.get(24)?, 
            row.get(25)?, row.get(26)?, row.get(27)?
        ))
    })?;

    let mut row_count = 0;
    for row in rows {
        println!("{:?}", row?);
        row_count += 1;
    }

    log_operation("Select", &format!("Retrieved {} rows", row_count))?;
    Ok(())
}

pub fn update_row(age: &str, updates: Vec<(&str, &dyn rusqlite::ToSql)>) -> Result<(), Box<dyn Error>> {
    log_operation("Update", &format!("Updating row for age group: {}", age))?;

    let conn = Connection::open(DB_FILE)?;
    let set_clause = updates.iter().map(|(col, _)| format!("{} = ?", col)).collect::<Vec<_>>().join(", ");
    let query = format!("UPDATE DrugUse SET {} WHERE age = ?", set_clause);

    let mut params = updates.into_iter().map(|(_, value)| value).collect::<Vec<_>>();
    params.push(&age);

    let rows_affected = conn.execute(query.as_str(), params.as_slice())?;
    log_operation("Update", &format!("Updated {} rows", rows_affected))?;
    Ok(())
}

pub fn delete_row(age: &str) -> Result<(), Box<dyn Error>> {
    log_operation("Delete", &format!("Deleting row for age group: {}", age))?;

    let conn = Connection::open(DB_FILE)?;
    let rows_affected = conn.execute("DELETE FROM DrugUse WHERE age = ?", params![age])?;
    
    log_operation("Delete", &format!("Deleted {} rows", rows_affected))?;
    Ok(())
}
use clap::{Parser, Subcommand};
use drug_use_sqlite::{delete_row, custom_query, extract, insert_row, load_transform, select_rows, update_row};
use std::error::Error;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "Drug Use Database CLI")]
#[command(about = "A CLI for managing drug use statistics database", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract data from URL
    Extract,
    /// Load data into database
    Load,
    /// Select all rows
    Select,
    /// Insert new test data
    Insert,
    /// Update existing data
    Update {
        /// Age group to update
        #[arg(short, long)]
        age: String,
        /// Field to update (e.g., "alcohol_use")
        #[arg(short, long)]
        field: String,
        /// New value
        #[arg(short, long)]
        value: f64,
    },
    /// Delete a row
    Delete {
        /// Age group to delete
        #[arg(short, long)]
        age: String,
    },
    /// Execute custom SQL query
    Query {
        /// SQL query to execute
        #[arg(short, long)]
        sql: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Extract => {
            let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/drug-use-by-age/drug-use-by-age.csv";
            let file_path = "data/drug-use-by-age.csv";
            extract(url, file_path)?;
        }
        Commands::Load => {
            load_transform("data/drug-use-by-age.csv")?;
        }
        Commands::Select => {
            select_rows()?;
        }
        Commands::Insert => {
            let test_data = create_test_data();
            insert_row(test_data)?;
        }
        Commands::Update { age, field, value } => {
            let updates = vec![(field.as_str(), &value as &dyn rusqlite::ToSql)];
            update_row(&age, updates)?;
        }
        Commands::Delete { age } => {
            delete_row(&age)?;
        }
        Commands::Query { sql } => {
            let query = if let Some(q) = sql {
                q
            } else {
                print!("Enter your SQL query: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input
            };
            custom_query(&query)?;
        }
    }

    println!("Operation completed successfully!");
    Ok(())
}

fn create_test_data() -> (String, i32, f64, f64, f64, f64, Option<f64>, Option<f64>, Option<f64>, 
    Option<f64>, Option<f64>, Option<f64>, f64, f64, f64, Option<f64>, f64, f64, f64, Option<f64>, 
    f64, f64, f64, f64, Option<f64>, Option<f64>, f64, f64) {
    (
        String::from("75+"),
        1000,
        50.0,
        40.0,
        30.0,
        20.0,
        Some(5.0),
        Some(1.0),
        Some(0.0),
        Some(0.0),
        Some(0.0),
        Some(0.0),
        10.0,
        5.0,
        3.0,
        Some(2.0),
        5.0,
        3.0,
        0.5,
        Some(0.5),
        1.0,
        1.0,
        2.0,
        1.0,
        Some(0.0),
        Some(0.0),
        0.5,
        0.3,
    )
}
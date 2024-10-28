use drug_use_sqlite::{delete_row, extract, insert_row, load_transform, select_rows, update_row};

#[test]
fn test_extract() {
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/drug-use-by-age/drug-use-by-age.csv";
    let file_path = "data/drug-use-by-age.csv";
    let result = extract(url, file_path);
    assert!(result.is_ok());
    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_load_transform() {
    let dataset = "data/drug-use-by-age.csv";
    let result = load_transform(dataset);
    assert!(result.is_ok());
    // Check if database file was created
    assert!(std::fs::metadata("DrugUseDB.db").is_ok());
}

#[test]
fn test_insert_row() {
    let test_data = (
        String::from("TEST-AGE"),
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
    );
    let result = insert_row(test_data);
    assert!(result.is_ok());
}

#[test]
fn test_select_rows() {
    let result = select_rows();
    assert!(result.is_ok());
}

#[test]
fn test_update_row() {
    let updates = vec![
        ("alcohol_use", &70.0 as &dyn rusqlite::ToSql),
        ("marijuana_use", &15.0 as &dyn rusqlite::ToSql),
    ];
    let result = update_row("30-34", updates);
    assert!(result.is_ok());
}

#[test]
fn test_delete_row() {
    let result = delete_row("TEST-AGE");
    assert!(result.is_ok());
}
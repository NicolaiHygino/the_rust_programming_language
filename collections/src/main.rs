#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// STRINGS ARE COMPLEX!!!!!!!!!!!!!!

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    fn return_modified_values(spreadsheet: &Vec<SpreadsheetCell>) -> Vec<SpreadsheetCell> {
        let mut new_spreadsheet: Vec<SpreadsheetCell> = vec![];

        for i in spreadsheet {
            match i {
                SpreadsheetCell::Int(value) => {
                    new_spreadsheet.push(SpreadsheetCell::Int(value + 50))
                }
                SpreadsheetCell::Text(value) => {
                    let mut new_value = String::from(value);
                    new_value.push_str(" nicolai é lindo");
                    new_spreadsheet.push(SpreadsheetCell::Text(new_value))
                }
                SpreadsheetCell::Float(value) => {
                    new_spreadsheet.push(SpreadsheetCell::Float(value + 50.50))
                }
            }
        }

        new_spreadsheet
    }

    let new_spreadsheet_modified = return_modified_values(&row);

    for i in &new_spreadsheet_modified {
        println!("{:?}", i);
    }

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s1 = format!("{s1}-{s2}-{s3}");

    println!("{s1}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}

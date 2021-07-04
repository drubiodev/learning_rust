enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(4);
    v.push(5);
    v.push(6);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        print!("{},", i)
    }

    // use (*) dereference to get value in i
    for i in &mut v {
        *i += 50;
    }
    println!(" ");
    for i in &v {
        print!("{},", i)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            _ => println!("No Match"),
        }
    }
}

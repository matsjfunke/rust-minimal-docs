fn main() {
    // empty vector
    let mut v: Vec<i32> = Vec::new();
    // push into vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // get the thrid item with .get
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // pre populated vector
    let mut x = vec![100, 32, 57];
    // iterating over vector
    for i in &mut x {
        *i += 50;
    }
    println!("index 1 {:?}", x.get(1));

    // using enum to store multiple types
    enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let text = row.get(1);
    match text {
        Some(SpreadsheetCell::Int(value)) => println!("The first element is {}", value),
        Some(SpreadsheetCell::Float(value)) => println!("The first element is {}", value),
        Some(SpreadsheetCell::Text(value)) => println!("The first element is {}", value),
        None => println!("There is no first element."),
    }
}

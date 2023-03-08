fn main() {
    let s = String::new();
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = &v[2];
    println!("The third element is {third}");

    let v = vec![1, 2, 3, 4, 5];
    //let does_not_exist = &v[100]; // Error
    let does_not_exist = v.get(100); // Option, not error

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("The vector is {v:?}");

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Hello world")),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

use std::io;

fn main() {
    parse_num();
    compound_types_tuple();
    compound_types_array();
    compound_types_array_indexing();
    // overflow_int();
}

fn parse_num() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The answer to the universe is {guess}");
}

fn compound_types_tuple() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("Your tuple is {tuple:?}");

    let (x, _, _) = tuple;
    println!("The first value of the tuple is {x}");

    let second_value = tuple.1;
    println!("The second value of the tuple is {second_value}");
}

fn compound_types_array() {
    let arr = [1, 2, 3, 4, 5, 6];
    println!("Simply array is {arr:?}");

    let months = ["January", "February", "March"];
    println!("Months are {months:?}");

    let repeat_arr = [3; 5];
    println!("Your repeat arr is {repeat_arr:?}");
}

fn compound_types_array_indexing() {
    let a = [1, 2, 3, 4, 5, 6];
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to readl ine");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the elem,ent at index {index} is: {element}");
}

fn overflow_int() {
    // let i:u8 = 256; // Not allowed, compiler catches this early.
    let i: u8 = "256".parse().expect("Not a valid number!"); // Overflow, thread panic!
    println!("Your number is {i}");
    // If above compiled with release flag, will wrap (?? Can't reproduce ??)
}

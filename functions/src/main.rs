fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    print_value(32);
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");

    let x = five();
    print_value(x);
    let x = plus_one(x);
    print_value(x);
}
fn print_value(x: i32) {
    println!("The value of x is {x}");
}

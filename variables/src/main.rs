// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // immutable_err();
    mutable();
    shadowing();
    shadowing_type_change();
    // mutable_type_change_err();
}

// fn immutable_err(){
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
fn mutable(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing(){
    let x = 5;
    let x = x + 1;

    // Inner scope shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn shadowing_type_change(){
    let spaces = "        ";
    let spaces = spaces.len();
    println!("We have {spaces} spaces");
}

// fn mutable_type_change_err(){
//     let mut spaces = "        ";
//     spaces = spaces.len();
//     println!("We have {spaces} spaces");
// }
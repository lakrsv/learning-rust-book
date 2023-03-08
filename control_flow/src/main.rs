fn main() {
    simple_conditional();
    assignment_conditional();
    // forever_loop();
    returning_value_from_loop();
    loop_labels();
    while_condition();
    index_vs_for_loop();
    reverse_loop();
}
fn simple_conditional() {
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}
fn assignment_conditional() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
// fn forever_loop(){
//     loop {
//         println!("Again!");
//     }
// }
fn returning_value_from_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}
fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
fn while_condition() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn index_vs_for_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while (index < a.len()) {
        println!("The value is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value is {element}");
    }
}

fn reverse_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

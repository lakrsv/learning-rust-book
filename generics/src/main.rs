use std::cmp::PartialOrd;

fn main() {
    let number_list = vec![10, 20, 50, 60, 70, 80];
    // let mut largest = &number_list[0];
    // for number in &number_list{
    //     if number > largest{
    //         largest = number;
    //     }
    // }
    let largest_int = largest(&number_list);
    println!("The largest number is {largest_int}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char = largest(&char_list);
    println!("The largest char is {largest_char}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.0, y: 2.3 };
}

// fn largest(list: &[i32]) -> &i32 {
//         let mut largest = &list[0];

//      for number in list{
//         if number > largest{
//             largest = number;
//         }
//     }
//     largest
// }

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

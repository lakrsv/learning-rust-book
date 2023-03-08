use std::slice;

fn main() {
    raw_pointers();
    arbitrary_raw_pointer();

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = split_at_mut_example(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

fn raw_pointers() {
    let mut num = 5;
    let r1 = &num as *const i32; // Immutable raw pointer
    let r2 = &mut num as *mut i32; // Mutable raw pointer

    // Can print reference
    println!("Raw immutable pointer is {:?}", r1);
    println!("Raw mutable pointer is {:?}", r2);

    // But we have to dereference in an unsafe block
    unsafe {
        println!("Raw immutable pointer is {:?}", *r1);
        println!("Raw mutable pointer is {:?}", *r2);
    }
}

fn arbitrary_raw_pointer() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

unsafe fn dangerous() {}

// fn split_at_mut_example(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
//     let len = values.len();

//     assert!(mid <= len);

//     // Rust doesn't understand we're borrowing different parts of the slice, it only knows we're borrowing from the same slice twice.
//     // This only works in an unsafe block
//     (&mut values[..mid], &mut values[mid..])
// }

fn split_at_mut_example(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

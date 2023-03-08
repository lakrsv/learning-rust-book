fn main() {
    let mut s = String::from("Hello, world!");
    let word = first_word(&s); // Value will be 6
    s.clear(); // This empties the string, making it equal to ""
               // Word still have the value 6 here, but there's no more string that
               // we could meaningfully is the value 6 with. Word is now totally invalid! It isn't connected to the state of s.

    // Solution? String slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // This is the same, dropping start index
    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    // This is the same as well, dropping end toklen
    let slice = &s[3..len];
    let slice = &s[3..];

    // This is also the same, dropping both start and end
    let slice = &s[0..len];
    let slice = &s[..];

    // Rewriting first_word using slices, much better
    let mut s = String::from("hello world");
    let word = first_word_slice(&s);
    // s.clear(); // Error!
    println!("The first word is {}", word);

    let s = "Hello, world!"; // Type is &str, it's a slice pointing to that specific point of the binary. This is why string literals are immutable. &str is an immutable reference.

    // Integer slide
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

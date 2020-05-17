// primitive str = immutable fixed length string
// string = growable, heap-allocated data structure


pub fn run() {
    let hello = "Hello"; // immutable
    let mut bye = String::from("Goodbye ");

    println!("{}", hello);

    println!("length of goodbye: {}", bye.len());

    bye.push('W');

    bye.push_str("orld!");

    println!("{}", bye);

    // capacity in bytes
    println!("capacity: {}", bye.capacity());

    // check if empty
    println!("is empty: {}", bye.is_empty());

    // contains string
    println!("contains 'World' {}", bye.contains("World"));

    // replace
    println!("replace: {}", bye.replace("World", "there"));

    // loop through string by whitespace
    for word in bye.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());  // does nothing if it's true, reports error if it's false
    assert_eq!(10, s.capacity());  

    println!("{}", s);

}


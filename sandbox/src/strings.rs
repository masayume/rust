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

}


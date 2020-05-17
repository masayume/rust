pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("number: {}", 1);
    println!("{} are from {}", "women", "venus");

    // positional parameters
    println!(
        "{0} likes {1} but {1} are expensive",
        "marco", "manga"
    );

    // named arguments
    println!(
        "{name} likes to play {activity}", 
        name = "John", 
        activity = "baseball"
    );

    // placeholder traits
    println!("10 converted in Binary: {:b} Hex {:x} Octal {:o}", 10, 10, 10);

    // placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
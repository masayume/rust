// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("recursion: 13! is equal to:");
    println!("{}", rec_factorial(13));

    println!("iterative: 13! is equal to:");
    println!("{}", iter_factorial(13));

}

fn rec_factorial(i: u64) -> u64 {
    match i {
        0 => 1,
        n => n * rec_factorial(n-1)
    }
}

fn iter_factorial(i: u64) -> u64 {
    let mut acc = 1;
    for num in 2..=i {
        acc *= num;
    }
    acc
}

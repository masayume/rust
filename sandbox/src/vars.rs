// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block scoped language

pub fn run() {
    let name = "James";
    let mut age = 37;

    println!("My name is {} and I am {}", name, age);

    age = 38; // without "mut" this'd be an error because it'd be immutable

    println!("My name is {} and I am {}", name, age);

    // define constant (must have a type)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables at once
    let (my_name, my_age) = ("Marco", 49);
    println!("{} ss {}", my_name, my_age);
}
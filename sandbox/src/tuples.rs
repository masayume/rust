// touples are groups of values of DIFFERENT type
// max 12 elements

pub fn run() {

    let person: (&str, &str, i8) = ("James", "London", 49);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

}
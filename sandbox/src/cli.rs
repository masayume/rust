use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Marco";
    let status = "100%";

    println!("args: {:?}", args);
    println!("command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you ?", name);
    } else if command == "status" {
        println!("status is {}", status);
    } else {
        println!("not a valid command");
    }
}
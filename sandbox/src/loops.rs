// loops - used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // infinite loop
    loop {
        count += 1;
        println!("number: {}", count);

        if count == 20 {
            break;
        }
    }

    // while loop (fizzbuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("{}: fizzbuzz", count);
        } else if count % 3 == 0 {
            println!("{}: fizz", count);
        } else if count % 5 == 0 {
            println!("{}: buzz", count);
        } else {
            println!("{}:", count);
        }

        count += 1;
    }

    // for range loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("{}: fizzbuzz", x);
        } else if x % 3 == 0 {
            println!("{}: fizz", x);
        } else if x % 5 == 0 {
            println!("{}: buzz", x);
        } else {
            println!("{}:", x);
        }

    }


}
// vectors are RESIZABLE arrays where elements are the SAME DATA TYPE

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // reassign value
    numbers[2] = 20;

    // add on to vector
    numbers.push(5);
    numbers.push(6);

    // pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // get 1st value
    println!("{}", numbers[0]);

    // get array length
    println!("vector length: {}", numbers.len());

    // arrays are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice from array
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("numbers vec: {:?}", numbers);
    

}
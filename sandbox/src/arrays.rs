// arrays are FIXED lists where elements are the SAME DATA TYPE

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // reassign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // get 1st value
    println!("{}", numbers[0]);

    // get array length
    println!("array length: {}", numbers.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice from array
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

}
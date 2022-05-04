// Arrays - Fixed list where elements are thesame data types
use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get Array length
    println!("Array Length: {}", numbers.len());

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get Slice
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

}
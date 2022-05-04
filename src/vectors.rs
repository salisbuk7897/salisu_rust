
use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to a Vector
    numbers.push(6);
    numbers.push(7);

    // Pop off the last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get Vector length
    println!("Vector Length: {}", numbers.len());

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get Slice
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // Loop and Mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
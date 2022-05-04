// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Salisu";
    let mut age = 24;
    println!("My name is {} and I am {}", name, age);
    age = 25;
    println!("My name is {} and I am {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign Multiple vars
    let (my_name, my_age) = ("Salisu", 25);
    println!("{} is {}", my_name, my_age);

    
}
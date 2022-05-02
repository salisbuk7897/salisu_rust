pub fn run(){
    //print to console
    println!("Hello from print.rs file");
    //Basic Formatting
    println!("Number: {}", 1);
    //Basic Formatting
    println!("{} is from {}", "Salisu", "Nigeria");
    //positional arguments
    println!("{0} is from {1} and {0} loves to {2}", "Salis", "Nigeria", "code");
    //named arguments
    println!("{name} likes to {act}", name="Salisu", act="read");
    //placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10,10,10);
    //placeholder for debug trait
    println!("{:?}", (1, true, "hi"));
    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
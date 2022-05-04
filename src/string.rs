// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
pub fn run(){
    // Primitive str
    let hello = "Hello";
    // String
    let mut hello1 = String::from("Hello ");
    // Lenght of string
    println!("Length: {:?}", (hello.len(), hello1.len()));

    // push to String
    hello1.push('W'); // push char

    hello1.push_str("orld!"); // push string

    // Capacity in bytes
    println!("Capacity: {}", hello1.capacity());

    // isEmpty
    println!("isEmpty: {}", hello1.is_empty());

    // Contains Substring
    println!("Contains 'World' {}", hello1.contains("World"));

    // Replace
    println!("Replace: {}", hello1.replace("World", "There"));

    //loop through string by whitespace
    for word in hello1.split_whitespace(){
        println!("{}", word);
    }

    // Create string with a certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //Assertion testing
    assert_eq!(3, s.len());

    println!("{:?}", (hello, hello1));
}
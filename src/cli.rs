use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Salisu";
    let status = "90%";
    //println!("Args: {:?}", args);
    println!("Commnad: {}", command);
    if command == "hello" {
        println!("Hi {}, How are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Invalid Command");
    }
}
// Turples group together values of different types
// Max 12 elements

pub fn run(){
    let persons : (&str, &str, i8) = ("Salisu", "Ali", 24);

    println!("{} {} is {}", persons.0, persons.1, persons.2)
}
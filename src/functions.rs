// Functions - Used to store blocks of code for re-use

pub fn run(){
    greeting("hello", "Salisu");
    //Bind function values to variables
    let get_sum = add(3,4);
    println!("Sum {}", get_sum);

    //Closure
    let n3 = 10;
    let add_nums = |n1: i32, n2 : i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(9,9));
}

fn greeting(greet: &str, name: &str){
    println!("{} {} nice to meet you!", greet, name)
}

fn add(n1: i32, n2: i32) -> i32{
    n1 + n2
}
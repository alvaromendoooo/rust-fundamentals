use std::io::Read;

fn main() {

    // Test 01
    //println!("Hello, Rust!");

    // Test 02
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_whitespace(); // Split the inputs from
    // whitespace separation
    let a: i64 = numbers.next().unwrap().parse().unwrap(); // No need explicit parse type because it
    // takes it from variable definition type
    let b: i64 = numbers.next().unwrap().parse().unwrap();
    
    println!("{}", a + b);*/

    // Test 03
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_whitespace();
    let w: i64 = numbers.next().unwrap().parse().unwrap();
    let h: i64 = numbers.next().unwrap().parse().unwrap();
    
    println!("{}", w * h);*/

    // Test 04
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.to_uppercase());*/

    // Test 05
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut strings = input.split_whitespace();
    let name: &str = strings.next().unwrap();
    let age: &str = strings.next().unwrap();

    println!("Hi, {}! You are {} years old.", name, age);*/

    // Test 06
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_whitespace();
    let n: i64 = numbers.next().unwrap().parse().unwrap();
    
    match (n % 3 == 0, n % 5 == 0) {
        (true, true)   => println!("FizzBuzz"),
        (true, false)  => println!("Fizz"),
        (false, true)  => println!("Buzz"),
        (false, false) => println!("{}", n),
    }*/

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_whitespace();
    let n = numbers.next().unwrap().parse().unwrap();
    
    let mut sum: i64 = 0;
    for i in 1..=n {
        sum += i;
    }

    println!("{}", sum);
}

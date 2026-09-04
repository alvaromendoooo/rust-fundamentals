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
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.to_uppercase());
}

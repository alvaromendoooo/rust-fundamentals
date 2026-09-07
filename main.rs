use std::io::Read;
use std::collections::HashSet;

enum Light {
    Red,
    Yellow,
    Green
}


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
    
    // Test 07
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_whitespace();
    let n = numbers.next().unwrap().parse().unwrap();
    
    let mut sum: i64 = 0;
    for i in 1..=n {
        sum += i;
    }
    println!("{}", sum);*/
    
    // Test 08
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_whitespace();
    let n = numbers.next().unwrap().parse().unwrap();

    println!("{}", square(n));*/

    // Test 09
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();

    println!("{}", count(&lines.next().unwrap()));*/

    // Test 10
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let line_of_numbers = input.lines().next().unwrap();
    let numbers: Vec<i64> = line_of_numbers.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    println!("{}", numbers.iter().max().unwrap());*/

    // Test 11
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut line = input.lines().next().unwrap();
    let mut seen: HashSet<&str> = HashSet::new();
    
    for word in line.split_whitespace() {
        seen.insert(word);
    }

    println!("{}", seen.len())*/

    // Test 12
    /*let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let line = input.lines().next().unwrap();
    let numbers: Vec<i64> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let result: i64 = numbers.iter()
        .filter(|n| *n % 2 == 0)
        .map(|n| n * n)
        .sum();

    println!("{}", result);*/

    // Test 13
    /*pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Point {
        fn distance_sq(&self, other: &Point) -> i32 {
            let first_expression = other.x - self.x;
            let second_expression = other.y - self.y;

            return (first_expression * first_expression) + (second_expression * second_expression)
        }
    }

    let lines: Vec<i32> = (0..4)
        .map(|_| {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim().parse().unwrap()
        })
        .collect();

    let p1 = Point { x: lines[0], y: lines[1] };
    let p2 = Point { x: lines[2], y: lines[3] };

    println!("{}", p1.distance_sq(&p2))*/

    // Test 14
    /*let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let current = match input.trim() {
        "red" => Light::Red,
        "yellow" => Light::Yellow,
        _ => Light::Green
    };

    println!("{}", name(&next(current)))*/

    // Test 15
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    match parse_int(&input) {
        Ok(n) => println!("ok: {}", n),
        Err(e) => println!("error: {}", e)
    }

}

fn square(n: i64) -> i64 {
    n * n
}

fn count(line: &str) -> i32 {
    line.len() as i32
}

fn next(l: Light) -> Light {
    match l {
        Light::Red => Light::Green,
        Light::Yellow => Light::Red,
        Light::Green => Light::Yellow,
    }
}

fn name(l: &Light) -> &str {
    match l {
        Light::Red => "red",
        Light::Yellow => "yellow",
        Light::Green => "green"
    }
}

fn parse_int(s: &str) -> Result<i32, String> {
    s.trim()
        .parse::<i32>()
        .map_err(|_| "not a number".to_string())
}

use std::io;

fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    if n == 0 {
        return a
    }
    else if n == 1{
        return b
    }
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    return b
}

fn main() {
    println!("Please enter nth fibonacci number:");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: u32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    let result = fibonacci(value);
    println!("Result: {result}"); 
}

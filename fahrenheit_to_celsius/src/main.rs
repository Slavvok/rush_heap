use std::io;

fn convert_to_celsius(x: f32) -> f32 {
    return (x - 32.0) / (9.0/5.0)
}

fn main() {
    println!("Please input Fahrenheit value:");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read the line.");
    let value: f32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0, 
    };
    let result = convert_to_celsius(value.into());
    println!("Fahrenheit value: {value}");
    println!("Celsius value: {result}");
}

fn celsius_to_fahrenheit (celsius: f64) -> f64 {
    (celsius - 32.0) / 1.8
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit * 1.8) + 32.0
}

fn main() {
    let celsius :f64 = 53.6;
    let fahrenheit :f64 = 12.0;
    println!("Celsius_to_Fahrenheit result is: {}", celsius_to_fahrenheit(celsius));
    println!("Fahrenheit_to_Celsius result is: {}", fahrenheit_to_celsius(fahrenheit));
}
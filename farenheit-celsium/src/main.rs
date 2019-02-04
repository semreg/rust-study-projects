use std::io;

fn main() {
    let option = get_input("From what to what?\n1) Farenheit -> Celsius, 2) Celsius -> Farenheit\nYour choose:".to_string());
    let value = get_input("Enter your value:".to_string());
    let result = convert(option, value);

    println!("\nYour result: {}", result);
}

// Converting temperature
fn convert(opt: isize, val: isize) -> std::string::String {
    if opt == 1 {
        let tmp = (val - 32) * 5/9;
        format!("{}°C", tmp)
    } else {
        let tmp = (val * 9/5) + 32;
        format!("{}°F", tmp)
    }
}

// Getting input and converting to int
fn get_input(msg: std::string::String) -> isize {
    let mut input = String::new();

    println!("\n{}", msg);

    io::stdin().read_line(&mut input)
        .expect("String is not correct");

    input
        .trim()
        .parse()
        .unwrap()
}

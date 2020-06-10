use std::io;

fn fahrenheit_to_celcius(fah: i32) -> i32 {
    (fah - 32) * (5 / 9)
}

fn celcius_to_fahrenheit(cel: i32) -> i32 {
    (cel * 9 / 5) + 32
}

pub fn temp_convert() {
    println!("Type in 1 - for F to C and 2 - for C to F");

    let mut convert_type: String = String::new();

    io::stdin()
        .read_line(&mut convert_type)
        .expect("Failed to read line");

    println!("You have chosen:: {}", convert_type);

    println!("*************\nNow please enter the temp to convert: ");

    let mut from_temp = String::new();

    io::stdin()
        .read_line(&mut from_temp)
        .expect("Failed to read line");

    let from_temp: i32 = match from_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    let result: i32;

    if convert_type == "1" {
        result = fahrenheit_to_celcius(from_temp);
    } else {
        result = celcius_to_fahrenheit(from_temp);
    }

    println!("DONE FINALLY. Your result is {}", result);
}

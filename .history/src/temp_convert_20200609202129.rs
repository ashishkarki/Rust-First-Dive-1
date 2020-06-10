use std::io;

fn fahrenheit_to_celcius(fah: i32) {}

pub fn temp_convert() {
    println!("Type in 1 - for F to C and 2 - for C to F");

    let mut convert_type: String = String::new();

    io::stdin()
        .read_line(&mut convert_type)
        .expect("Failed to read line");

    println!("You have chosen:: {}", convert_type);

    println!("*************\n Now please enter the temp to convert: ");

    let mut from_temp = String::new();

    io::stdin()
        .read_line(&mut from_temp)
        .expect("Failed to read line");

    let from_temp: i32 = match from_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => println!("there was an error converting to numerical value!!!"),
    };
    fahrenheit_to_celcius(30);
}

use std::io;

pub fn temp_convert() {
    println!("Type in 1 - for F to C and 2 - for C to F");

    let mut convert_type: String = String::new();

    io::stdin()
        .read_line(&mut convert_type)
        .expect("Failed to read line");

    println!("You have chosen:: {}", convert_type);

    println!("Now please enter the temp to convert: ");

    let mut from_temp = String::new();

    io::stdin()
        .read_line(&mut from_temp)
        .expect("Failed to read line");
}

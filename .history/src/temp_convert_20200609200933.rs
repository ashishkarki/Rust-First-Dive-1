use std::io;

pub fn temp_convert() {
    println!("Type in 1 - for F to C and 2 - for C to F");

    let convert_type: String;

    io::stdin()
        .read_line(&mut convert_type)
        .expect("Failed to read line");
}

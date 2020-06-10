use std::io

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

pub fn fibonacci(n: u32) {
    println!("Please enter the number for nth fibonacci number")

    let mut fib_num = String::new()

    io::stdin()
    .read_line(&mut fib_num)
    .expect("Failed to read line");

    let fib_num: u32 = match fib_num.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    }

    let nth_fib_num = fib(fib_num);
}

use std::io::{self, Write};
use std::fmt::Display;
use std::process;

fn main() {
       println!("\tArithematic Operations(Add, Sub, Mul, Div )");
       println!("\n");

    let v1: i32 = grab_input("Enter First Number ")
        .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
        .trim()
        .parse()
        .unwrap_or_else(|e| exit_err(&e, 2));

    let v2: i32 = grab_input("Enter Second Number ")
        .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
        .trim()
        .parse()
        .unwrap_or_else(|e| exit_err(&e, 2));

    let sum = v1 + v2;
    let sub = v1 - v2;
    let mul = v1 * v2;
    let div = v1 / v2;

    println!("\n");
    println!("The addition of {} and {} is {}. ",v1,v2,sum);
    println!("\n");
    println!("The subtraction of {} and {} is {}. ",v1,v2,sub);
    println!("\n");
    println!("The multiplication of {} and {} is {}. ",v1,v2,mul);
    println!("\n");
    println!("The division of {} and {} is {}. ",v1,v2,div);
    println!("\n");
}

fn grab_input(msg: &str) -> io::Result<String> {
    let mut buf = String::new();
    print!("{}: ", msg);
    try!(io::stdout().flush());

    try!(io::stdin().read_line(&mut buf));
    Ok(buf)
}

fn exit_err<T: Display>(msg: T, code: i32) -> ! {
    let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
    process::exit(code)
}

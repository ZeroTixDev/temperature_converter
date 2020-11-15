use std::io;
fn f_to_c(number: i32) -> f32 {
    ((number - 32) * 5 / 9) as f32
}
fn c_to_f(number: i32) -> f32 {
    ((number * 9 / 5) + 32) as f32
}
fn main() {
    /*println!("Hello, world!");
    println!("50F in celsuis is {}C",f_to_c(50));
    println!("50C in fahrenheit is {}F",c_to_f(50));*/
    let mut convert_mode_c: bool = true; // defaults to f -> c
    println!("To convert f -> c, type f, c -> f, type c. after you can put in numbers to convert (default is f -> c)");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim() == "f" {
            convert_mode_c = true;
            println!("f -> c");
        } else if guess.trim() == "c" {
            convert_mode_c = false;
            println!("c -> f")
        } else {
            let number: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                	println!("Invalid input");
                    continue;
                }
            };
            if convert_mode_c {
                println!("{}F is {}C", number, f_to_c(number));
            } else if !convert_mode_c {
                println!("{}C is {}F", number, c_to_f(number));
            }
        }
    }
}

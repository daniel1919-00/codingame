use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of temperatures to analyse
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    
    let mut closest_to_zero = 0;
    let mut closest_to_zero_abs = 0;

    for i in inputs.split_whitespace() {
        let temp = parse_input!(i, i32); // current temp
        let temp_abs = temp.abs();

        if temp == 0 {
            break;
        }
        else if closest_to_zero == 0 {
            closest_to_zero = temp;
            closest_to_zero_abs = temp.abs();
        }
        else if temp_abs < closest_to_zero_abs {
            closest_to_zero_abs = temp_abs;
            closest_to_zero = temp;
        }
        else if temp_abs == closest_to_zero_abs && temp > 0 {
            closest_to_zero = temp;
        }
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{closest_to_zero}");
}

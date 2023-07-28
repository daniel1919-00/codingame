use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

macro_rules! encode_bits {
    ($last_bit: expr, $series_bit_count: expr) => (&std::format!("{} {} ", if $last_bit == '1' {"0"} else {"00"}, "0".repeat($series_bit_count)))
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let message: String = input_line.trim_matches('\n').to_string();

    let mut encodded_message = String::new();
    let mut last_bit: char = ' ';
    let mut series_bit_count = 0;

    for byte in message.into_bytes() {
        let binary_char = std::format!("{:07b}", byte);

        for bit in binary_char.chars() {
            if bit != last_bit {
                if last_bit != ' ' {
                    encodded_message.push_str(encode_bits!(last_bit, series_bit_count));
                }
                
                series_bit_count = 0;
                last_bit = bit;
            }
            
            series_bit_count += 1;
        }
    }

    if series_bit_count > 0 {
        encodded_message.push_str(encode_bits!(last_bit, series_bit_count));
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", encodded_message.trim());
}

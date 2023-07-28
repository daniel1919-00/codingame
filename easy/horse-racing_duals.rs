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
    let n = parse_input!(input_line, i32);
    
    let mut horse_powers: Vec<i32> = Vec::new();

    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, i32);

        if pi <= 0 {
            continue;
        }

        horse_powers.push(pi);
    }

    horse_powers.sort();

    let mut last_pow = 0;
    let mut smallest_diff = -1;
    for power in &horse_powers {
        if smallest_diff == -1 {
            last_pow = *power;
            smallest_diff = *power;
            continue;
        }

        let diff = power - last_pow;

        // Early exit in the event that we somehow have 2 equal horses
        if diff == 0 {
            smallest_diff = 0;
            break;
        }

        if diff < smallest_diff {
            smallest_diff = diff;
        }

        last_pow = *power;
    }

    println!("{smallest_diff}");
}

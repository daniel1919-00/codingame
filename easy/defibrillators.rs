use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().replace(",", ".").parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let usr_long = parse_input!(input_line, f64).to_radians();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let usr_lat = parse_input!(input_line, f64).to_radians();
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let n = parse_input!(input_line, i32);
    let mut shortest_dist: f64 = 0.0;
    let mut answer = String::new();
    
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let defib = input_line.trim_matches('\n').to_string();
        let defib_info: Vec<&str> = defib.split(';').collect();

        if defib_info.len() != 6 {
            continue;
        }

        let defib_long = parse_input!(defib_info[4], f64).to_radians();
        let defib_lat = parse_input!(defib_info[5], f64).to_radians();

        let x = (defib_long - usr_long) * ((usr_lat + defib_lat) / 2.0).cos();
        let y = defib_lat - usr_lat;

        let dist = (x * x + y * y).sqrt() * 6371.0;

        if shortest_dist > dist || i == 0 {
            shortest_dist = dist;
            answer = defib_info[1].to_string();
        }
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{answer}");
}

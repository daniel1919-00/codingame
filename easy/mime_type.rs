use std::io;
use std::collections::HashMap;

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
    let n = parse_input!(input_line, i32); // Number of elements which make up the association table.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let q = parse_input!(input_line, i32); // Number Q of file names to be analyzed.

    let mut mime_types: HashMap<String, String> = HashMap::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let ext = inputs[0].trim().to_string(); // file extension
        if ext.len() > 10 {
            continue;
        }

        let mt = inputs[1].trim().to_string(); // MIME type.
        if mt.len() > 50 {
            continue;
        }

        if mime_types.contains_key(&ext) {
            continue;
        }
        
        mime_types.insert(ext.to_lowercase(), mt);
    }

    for i in 0..q as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let fname = input_line.trim_matches('\n').to_string(); // One file name per line.

        if fname.len() > 256 {
            continue;
        }

        let mut extension_chars: Vec<char> = Vec::new();
        let mut has_extension = false;
        for chr in fname.chars().rev() {

            if chr == '.' {
                if extension_chars.len() == 0 {
                    break;
                }

                has_extension = true;
                break;
            }

            extension_chars.push(chr.to_ascii_lowercase());
        }

        if extension_chars.len() > 10 {
            println!("UNKNOWN");
            continue;
        }

        if !has_extension {
            println!("UNKNOWN");
            continue;
        }

        let file_ext: String = extension_chars.into_iter().rev().collect();
        println!("{}", mime_types.get(&file_ext).unwrap_or(&String::from("UNKNOWN")));
    }
}

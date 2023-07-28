use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn char_to_index(char: char) -> usize {
    match char.to_ascii_uppercase() {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        _ => 26
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let width = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let height = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let text_to_render: String = input_line.trim_matches('\n').to_string();

    let mut ascii_characters: [Vec<String>; 27] = Default::default();


    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row: String = input_line.trim_matches('\n').to_string();
        
        let mut ascii_char_index = 0;
        let mut row_char_width = 0;
        let mut ascii_char_chars = String::new();

        for row_char in row.chars() {
            if row_char_width == width {
                ascii_characters[ascii_char_index].push(ascii_char_chars.clone());
                ascii_char_chars.clear();
                ascii_char_index += 1;
                row_char_width = 0;
            }

            ascii_char_chars.push(row_char);
            row_char_width += 1;
        }

        if !ascii_char_chars.is_empty() {
            ascii_characters[ascii_char_index].push(ascii_char_chars.clone());
        }
    }

    let mut answer = String::new();
    for i in 0..height as usize {
        for render_char in text_to_render.chars() {
            let ascii_char = &ascii_characters[char_to_index(render_char)];
            answer += &ascii_char[i];
        }    

        answer += "\n";
    }

    println!("{answer}", );
}

use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let mut thor_x = parse_input!(inputs[2], i32); // Thor's starting X position
    let mut thor_y = parse_input!(inputs[3], i32); // Thor's starting Y position

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        let mut action_x = "";
        let mut action_y = "";

        if light_x > thor_x {
            action_x = "E";
            thor_x += 1;
        } else if light_x < thor_x {
            action_x = "W";
            thor_x -= 1;
        }

        if light_y > thor_y {
            action_y = "S";
            thor_y += 1;
        } else if light_y < thor_y {
            action_y = "N";
            thor_y -= 1;
        }

        // A single line providing the move to be made: N NE E SE S SW W or NW
        println!("{action_y}{action_x}");
    }
}

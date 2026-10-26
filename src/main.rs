use std::io;

#[derive(Debug)]



/* 
fn change_direction(directions : new_direction){

    // if the user inputs up/down when the snake is already facing that direction do nothing.
    // so given the current direction state and the users input either move if valid or dont.

    
    match Direction {
        Directions::Up => {

        };
        Directions::Down {

        };
        Directions::Left {

        };
        Directions::Right {

        };
    }
}
*/

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn input_direction(mut input: String) -> Option<Direction> {
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim() {
                "w" | "W" => Some(Direction::Up),
                "d" | "D" => Some(Direction::Right),
                "a" | "A" => Some(Direction::Left),
                "s" | "S" => Some(Direction::Down),
                _ => {
                    println!("Invalid keypress");
                    None
                }
            }
        }
        Err(error) => {
            eprintln!("Error: {error}");
            None
        }
    }
}
// when we take an input from the user we want to print some formatted direction



fn main() {
    println!("Choose a direction");
    let input = String::new();

    match input_direction(input.clone()) {
        Some(direction) => println!("Direction: {direction:?}"),
        None => println!("No direction selected"),
    }
}

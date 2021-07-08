mod riddle1;
mod riddle2;
use riddle1::solve_riddle_1;
use riddle1::read;
use riddle2::solve_riddle_2;

fn main() {
    let mut choice = 0;
    println!("Select an option: ");
    println!("-----------------------------------------------------------");
    println!("1. Solve Riddle 1 ->");
    println!(":- Riddle: If 1/2 of 5 is 3, then what is 1/3 of 10? |");
    println!("-----------------------------------------------------------");
    println!("2. Solve Riddle 2 -> ");
    println!(":- Riddle: What is 3/7 chicken, 2/3 cat and 2/4 goat? |");
    println!("-----------------------------------------------------------");
    read(&mut choice);

    match choice {
        1 => solve_riddle_1(),
        2 => solve_riddle_2(),
        _ => println!("No Riddle selected"),
    }
}

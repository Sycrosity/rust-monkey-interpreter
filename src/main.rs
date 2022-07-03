use users::{get_current_username};

use monkey_interpreter::repl::repl;

fn main() {

    match get_current_username() {
        Some(name) => println!("Hello {:?}! This is the monkey programing language REPL.", name),
        None => panic!("user doesn't exist")
    }

    println!("Feel free to type in some commands.");

    repl();
}

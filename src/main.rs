use monkey_interpreter::repl::repl;
use users::get_current_username;

//runs a repl of monkey
fn main() {
    //check the username of the user who ran the command
    match get_current_username() {
        Some(name) => println!(
            "Hello {:?}! This is the monkey programing language REPL.",
            name
        ),
        None => panic!("user doesn't exist"),
    }

    println!("Feel free to type in some commands.");

    repl();
}

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::lexer::Lexer;
use crate::token::Token;

//creates a REPL (read, evaluate, print, loop) instance for executing monkey code.
pub fn repl() {
    let mut rl: Editor<()> = Editor::<()>::new();

    //loop until error or program is force closed.
    loop {
        let readline: Result<String, ReadlineError> = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut lex: Lexer = Lexer::new(&line);

                //[TODO] - once parser is built, parse given code and print output
                //[TODO] - store variables locally so they can be reused in the repl
                while let Some(token) = Some(lex.next_token()) {
                    if token != Token::EndOfFile {
                        println!("{:?}", token);
                    } else {
                        break;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            //match any other errors
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

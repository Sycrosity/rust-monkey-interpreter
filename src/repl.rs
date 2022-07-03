use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::token::Token;
use crate::lexer::Lexer;

pub fn repl() {

    let mut rl: Editor<()> = Editor::<()>::new();

    loop {
        
        let readline: Result<String, ReadlineError> = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut lex: Lexer = Lexer::new(&line);

                while let Some(token) = Some(lex.next_token()) {
                    if token != Token::EndOfFile {
                        println!("{:?}", token);
                    } else {
                        break;
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    };

}
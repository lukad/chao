extern crate libchao;
extern crate rustyline;

use rustyline::{DefaultEditor, error::ReadlineError};

use libchao::{Interpreter, parse};

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    rl.load_history(".chaohistory").unwrap_or_default();
    let mut interpreter = Interpreter::new();
    loop {
        let readline = rl.readline("chao> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line).unwrap();
                match parse(&line) {
                    Ok(expr) => println!("=> {}", interpreter.eval(&expr)),
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(".chaohistory").unwrap();
}

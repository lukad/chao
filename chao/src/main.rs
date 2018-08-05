extern crate libchao;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use libchao::{parse, Env};

fn main() {
    let mut rl = Editor::<()>::new();
    rl.load_history(".chaohistory").unwrap_or_default();
    let mut env = Env::new();
    loop {
        let readline = rl.readline("chao> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if let Some(expr) = parse(&line) {
                    println!("=> {}", env.eval(&expr));
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

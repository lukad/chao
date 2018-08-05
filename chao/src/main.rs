extern crate libchao;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use libchao::{Env, Expr::*};

fn main() {
    let mut rl = Editor::<()>::new();
    rl.load_history(".chaohistory").unwrap_or_default();
    let mut env = Env::new();
    let prog = List(vec![Symbol("if".to_string()), Bool(true), Int(42), Int(32)]);
    let result = env.eval(&prog);
    println!("{:?}", result);
    loop {
        let readline = rl.readline("chao> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("=> {}", line);
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

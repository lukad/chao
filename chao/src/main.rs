extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    rl.load_history(".chaohistory");
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

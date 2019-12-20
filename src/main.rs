extern crate rustyline;
extern crate serde_yaml;
extern crate confy;
use rustyline::{error::ReadlineError, Editor};

use lib_jsh::shell::Shell;

fn main() {
    let repl = Shell::from_config(confy::load("jirash").unwrap());
    println!("{:?}", repl);

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(&repl.build_prompt());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

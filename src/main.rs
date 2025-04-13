mod command;

use command::Pipeline;
use rustyline::history::DefaultHistory;
use rustyline::{Config, Editor};

fn main() {
    let config = Config::builder().build();
    let mut rl = Editor::<(), DefaultHistory>::with_config(config).unwrap();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(input) => {
                let input = input.trim_end().to_string();
                if !input.is_empty() {
                    rl.add_history_entry(&input).unwrap();
                }
                let mut pipeline = Pipeline::parse(&input);
                pipeline.execute();
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                // TODO: Handle Ctrl-C
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                // TODO: Handle Ctrl-D
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

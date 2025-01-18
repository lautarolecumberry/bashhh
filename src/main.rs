mod command;

use command::Pipeline;
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim_end();

        let mut pipeline = Pipeline::parse(input);
        pipeline.execute();
    }
}

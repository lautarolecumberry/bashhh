mod command;

use command::Pipeline;
use command::SimpleCommand;
use std::io;

fn main() {
    loop {
        print!("> ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim_end();

        let mut pipeline = Pipeline::parse(input);
        pipeline.execute();
    }
}

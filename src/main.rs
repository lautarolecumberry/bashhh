mod command;

use command::Pipeline;
use command::SimpleCommand;
use std::io;

fn main() {
    loop {
        print!("> ");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim_end();
        println!("executing {command} ... ");
        let mut s_cmd = SimpleCommand::new();
        s_cmd.push_back(command.to_string());
        let mut pipeline = Pipeline::new();
        pipeline.push_back(s_cmd);
        pipeline.execute();
    }
}

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
        s_cmd.push_back(String::from("echo"));
        s_cmd.push_back(String::from("hola"));
        let mut s_cmd2 = SimpleCommand::new();
        s_cmd2.push_back(String::from("grep"));
        s_cmd2.push_back(String::from("hola"));
        let mut s_cmd3 = SimpleCommand::new();
        s_cmd3.push_back(String::from("grep"));
        s_cmd3.push_back(String::from("hola"));
        let mut pipeline = Pipeline::new();
        pipeline.push_back(s_cmd);
        pipeline.push_back(s_cmd2);
        pipeline.push_back(s_cmd3);
        pipeline.execute();
    }
}

mod command;

use command::Pipeline;
use command::SimpleCommand;
// use std::io;

// fn main() {
//     print!("> ");
//     let mut command = String::new();
//     io::stdin()
//         .read_line(&mut command)
//         .expect("Failed to read line");
//     let command = command.trim_end();
//     println!("executing {command} ... ");
// }

fn main() {
    // let args = vec![String::from("arg1"), String::from("arg2")];
    // let out = String::from("output");
    // let input = String::from("input");

    let mut command = SimpleCommand::new();
    command.dump();
    command.push_back(String::from("arg3"));
    command.dump();
    let popped = command.pop_front();
    println!("Popped: {:?}", popped);
    command.dump();
    command.set_redir_in(String::from("new input"));
    command.set_redir_out(String::from("new output"));
    command.dump();
    println!("Is empty: {}", command.is_empty());
    command.pop_front();
    command.pop_front();
    println!("Is empty: {}", command.is_empty());
    println!("Length: {}", command.length());
    println!("Front: {:?}", command.front());
    println!("Redir in: {}", command.get_redir_in());
    println!("Redir out: {}", command.get_redir_out());
    println!("\n=========================================\n");

    let mut pipeline = Pipeline::new();
    let mut command = SimpleCommand::new();
    command.push_back(String::from("ls"));
    command.push_back(String::from("-l"));
    command.set_redir_in(String::from("ls.txt"));
    let mut command2 = SimpleCommand::new();
    command2.push_back(String::from("grep"));
    pipeline.dump();
    pipeline.push_back(command);
    pipeline.dump();
    pipeline.push_back(command2);
    pipeline.dump();
    pipeline.set_wait(false);
    pipeline.dump();
    println!("Length: {}", pipeline.length());
    println!("Is empty: {}", pipeline.is_empty());
    println!("Front: {:?}", pipeline.front());
    println!("Wait: {}", pipeline.get_wait());
    let popped = pipeline.pop_front();
    println!("Popped: {:?}", popped);
    pipeline.pop_front();
    println!("Is empty: {}", pipeline.is_empty());
    println!("Length: {}", pipeline.length());
    let mut pipeline = Pipeline::new();
    for i in 0..257 {
        let mut cmd = SimpleCommand::new();
        cmd.push_back(format!("gtk-fuse{}", i));
        pipeline.push_back(cmd);
    }
    pipeline.set_wait(false);
    pipeline.dump();
}

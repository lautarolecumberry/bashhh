mod command;

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
    let args = vec![String::from("arg1"), String::from("arg2")];
    let out = String::from("output");
    let input = String::from("input");

    let mut command = SimpleCommand::new(args, out, input);
    command.dump();
    let pushed = command.push_back(String::from("arg3"));
    println!("Pushed: {:?}", pushed);
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
}

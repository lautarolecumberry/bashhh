#[derive(PartialEq)]
#[derive(Debug)]
pub struct SimpleCommand {
    args: Vec<String>,
    out: String,
    input: String,
}

impl SimpleCommand {
    pub fn new() -> SimpleCommand {
        SimpleCommand { args: Vec::new(), out: String::new(), input: String::new() }
    }

    pub fn push_back(&mut self, arg: String) {
        self.args.push(arg);
    }

    pub fn pop_front(&mut self) -> Option<String> {
        return self.args.pop()
    }

    pub fn set_redir_in(&mut self, input: String) {
        self.input = input;
    }

    pub fn set_redir_out(&mut self, out: String) {
        self.out = out;
    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn length(&self) -> usize {
        self.args.len()
    }

    pub fn front(&self) -> Option<&String> {
        self.args.first()
    }

    pub fn get_redir_in(&self) -> &String {
        &self.input
    }

    pub fn get_redir_out(&self) -> &String {
        &self.out
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for arg in &self.args {
            result.push_str(arg);
            if arg != self.args.last().unwrap() {
                result.push(' ');
            }
        }
        result.push_str(" < ");
        result.push_str(&self.input);
        result.push_str(" > ");
        result.push_str(&self.out);
        result
    }

    pub fn dump(&self) {
        println!("SimpleCommand `{}`", self.to_string());
        println!();
    }
}

pub struct Pipeline {
    commands: Vec<SimpleCommand>,
    should_wait: bool,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline { commands: Vec::new(), should_wait: true }
    }

    pub fn push_back(&mut self, command: SimpleCommand) {
        self.commands.push(command);
    }

    pub fn pop_front(&mut self) -> Option<SimpleCommand> {
        return self.commands.pop()
    }

    pub fn set_wait(&mut self, wait: bool) {
        self.should_wait = wait;
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn length(&self) -> usize {
        self.commands.len()
    }

    pub fn front(&self) -> Option<&SimpleCommand> {
        self.commands.first()
    }

    pub fn get_wait(&self) -> bool {
        self.should_wait
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for command in &self.commands {
            result.push_str(&command.to_string());
            if command != self.commands.last().unwrap() {
                result.push_str(" | ");
            }
        }
        if self.should_wait {
            result.push_str(" &");
        }
        result
    }

    pub fn dump(&self) {
        println!("Pipeline `{}`", self.to_string());
        println!();
    }
}

pub struct SimpleCommand {
    args: Vec<String>,
    out: String,
    input: String,
}

impl SimpleCommand {
    pub fn new(args: Vec<String>, out: String, input: String) -> SimpleCommand {
        SimpleCommand { args, out, input }
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

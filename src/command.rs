use std::collections::VecDeque;
use std::process::{Child, Command, Stdio};

#[derive(PartialEq, Clone)]
pub struct SimpleCommand {
    args: VecDeque<String>,
    out: String,
    input: String,
}

impl SimpleCommand {
    pub fn new() -> SimpleCommand {
        SimpleCommand {
            args: VecDeque::new(),
            out: String::new(),
            input: String::new(),
        }
    }

    pub fn push_back(&mut self, arg: String) {
        self.args.push_back(arg);
    }

    pub fn pop_front(&mut self) -> Option<String> {
        return self.args.pop_front();
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

    // pub fn length(&self) -> usize {
    //     self.args.len()
    // }

    // pub fn front(&self) -> Option<&String> {
    //     self.args.front()
    // }

    pub fn get_redir_in(&self) -> &String {
        &self.input
    }

    pub fn get_redir_out(&self) -> &String {
        &self.out
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let len = self.args.len();
        for (i, arg) in self.args.iter().enumerate() {
            result.push_str(arg);
            if i < len - 1 {
                result.push(' ');
            }
        }
        if !self.input.is_empty() {
            result.push_str(" < ");
            result.push_str(&self.input);
        }
        if !self.out.is_empty() {
            result.push_str(" > ");
            result.push_str(&self.out);
        }
        result
    }

    fn get_args(&self) -> Vec<&str> {
        self.args.iter().map(|s| s.as_str()).collect()
    }

    pub fn execute(
        &mut self,
        in_piped: Option<Child>,
        out_piped: bool,
        should_wait: bool,
    ) -> Child {
        let cmd = self.pop_front().unwrap();
        let args = self.get_args();

        let mut command = Command::new(cmd);
        command.args(args);

        // TODO: if in file

        if let Some(mut child) = in_piped {
            command.stdin(
                child
                    .stdout
                    .take()
                    .expect("Failed to take stdout from first command"),
            );
        }

        // TODO: if out file

        if out_piped {
            command.stdout(Stdio::piped());
        }

        let mut child = command.spawn().expect("Failed to execute command");
        if should_wait {
            child.wait().expect("Failed to wait on child");
        }
        child
    }

    pub fn parse(command_str: &str) -> SimpleCommand {
        let mut command = SimpleCommand::new();
        let mut parts = command_str.split_whitespace();

        let mut input_set = false;
        let mut output_set = false;

        while let Some(part) = parts.next() {
            if part == "<" {
                if let Some(input_file) = parts.next() {
                    command.input = input_file.to_string();
                    input_set = true;
                }
            } else if part == ">" {
                if let Some(output_file) = parts.next() {
                    command.out = output_file.to_string();
                    output_set = true;
                }
            } else {
                command.args.push_back(part.to_string());
            }
        }
        command
    }
}

pub struct Pipeline {
    commands: VecDeque<SimpleCommand>,
    should_wait: bool,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            commands: VecDeque::new(),
            should_wait: true,
        }
    }

    pub fn push_back(&mut self, command: SimpleCommand) {
        self.commands.push_back(command);
    }

    pub fn pop_front(&mut self) -> Option<SimpleCommand> {
        return self.commands.pop_front();
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
        self.commands.front()
    }

    pub fn get_wait(&self) -> bool {
        self.should_wait
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        if self.commands.is_empty() {
            return result;
        }
        let last_index = self.commands.len() - 1;
        for (i, command) in self.commands.iter().enumerate() {
            result.push_str(&command.to_string());
            if i < last_index {
                result.push_str(" | ");
            }
        }
        if !self.get_wait() && !self.commands.is_empty() {
            result.push_str(" &");
        }
        result
    }

    pub fn execute(&mut self) {
        if self.is_empty() {
            return;
        }

        let mut previous_child = None;

        while let Some(mut command) = self.pop_front() {
            let out_piped = !self.is_empty();
            previous_child = Some(command.execute(previous_child, out_piped, self.get_wait()));
        }
    }

    pub fn parse(input: &str) -> Pipeline {
        let mut pipeline = Pipeline::new();

        let command_strs: Vec<String> = input
            .split('|')
            .map(str::trim)
            .map(|s| s.to_string())
            .collect();

        let default_command = String::from("");
        let last_command = command_strs.last().unwrap_or(&default_command);

        if last_command.ends_with("&") {
            pipeline.should_wait = false;
        }

        for (i, command_str) in command_strs.iter().enumerate() {
            let mut command_str = command_str.to_string();

            if i == command_strs.len() - 1 && command_str.ends_with("&") {
                command_str = command_str.trim_end_matches("&").to_string();
            }

            let command = SimpleCommand::parse(&command_str);
            pipeline.commands.push_back(command);
        }

        pipeline
    }
}

#[cfg(test)]
mod simple_command_tests {
    use super::*;

    #[test]
    fn test_length_null() {
        let command = SimpleCommand::new();
        assert_eq!(command.length(), 0);
    }

    #[test]
    fn test_front_null() {
        let command = SimpleCommand::new();
        assert!(command.front().is_none());
    }

    #[test]
    fn test_front_empty() {
        let scmd = SimpleCommand::new();
        assert!(scmd.front().is_none());
    }

    #[test]
    fn test_new_is_empty() {
        let scmd = SimpleCommand::new();
        assert!(scmd.is_empty());
        assert_eq!(scmd.length(), 0);
    }

    #[test]
    fn test_adding_emptying() {
        let mut scmd = SimpleCommand::new();
        for i in 0..257 {
            assert_eq!(i == 0, scmd.is_empty());
            scmd.push_back("123".to_string());
        }
        for _ in 0..257 {
            assert!(!scmd.is_empty());
            scmd.pop_front();
        }
        assert!(scmd.is_empty());
    }

    #[test]
    fn test_adding_emptying_length() {
        let mut scmd = SimpleCommand::new();
        for i in 0..257 {
            assert_eq!(i, scmd.length());
            scmd.push_back("123".to_string());
        }
        for i in (1..=257).rev() {
            assert_eq!(i, scmd.length());
            scmd.pop_front();
        }
        assert_eq!(scmd.length(), 0);
    }

    #[test]
    fn test_fifo() {
        let mut scmd = SimpleCommand::new();
        let mut strings = Vec::new();
        for i in 0..257 {
            strings.push(i.to_string());
        }
        for s in &strings {
            scmd.push_back(s.clone());
        }
        for s in &strings {
            let front = scmd.front().unwrap();
            println!("{} == {}", front, s);
            assert_eq!(front, s);
            let popped = scmd.pop_front();
            println!("{}", popped.unwrap());
        }
    }

    #[test]
    fn test_front_idempotent() {
        let mut scmd = SimpleCommand::new();
        scmd.push_back("123".to_string());
        for _ in 0..257 {
            assert_eq!(scmd.front().unwrap(), "123");
        }
    }

    #[test]
    fn test_front_is_back() {
        let mut scmd = SimpleCommand::new();
        scmd.push_back("123".to_string());
        assert_eq!(scmd.front().unwrap(), "123");
    }

    #[test]
    fn test_front_is_not_back() {
        let mut scmd = SimpleCommand::new();
        scmd.push_back("123".to_string());
        scmd.push_back("456".to_string());
        assert_ne!(scmd.front().unwrap(), "456");
    }

    #[test]
    fn test_redir() {
        let mut scmd = SimpleCommand::new();
        scmd.set_redir_in("123".to_string());
        scmd.set_redir_out("456".to_string());
        assert_ne!(scmd.get_redir_in(), scmd.get_redir_out());
        scmd.set_redir_out("123".to_string());
        assert_eq!(scmd.get_redir_in(), scmd.get_redir_out());
    }

    #[test]
    fn test_independent_redirs() {
        let mut scmd = SimpleCommand::new();
        scmd.set_redir_in("123".to_string());
        assert_eq!(scmd.get_redir_in(), "123");
        assert_eq!(scmd.get_redir_out(), "");
        scmd.set_redir_in(String::new());
        assert_eq!(scmd.get_redir_in(), "");
        assert_eq!(scmd.get_redir_out(), "");
        scmd.set_redir_out("456".to_string());
        assert_eq!(scmd.get_redir_in(), "");
        assert_eq!(scmd.get_redir_out(), "456");
        scmd.set_redir_in("123".to_string());
        assert_eq!(scmd.get_redir_in(), "123");
        assert_eq!(scmd.get_redir_out(), "456");
    }

    #[test]
    fn test_to_string_empty() {
        let scmd = SimpleCommand::new();
        assert_eq!(scmd.to_string(), "");
    }

    #[test]
    fn test_to_string() {
        let mut scmd = SimpleCommand::new();
        let mut strings = Vec::new();
        for i in 0..257 {
            strings.push(i.to_string());
        }
        for i in 0..255 {
            scmd.push_back(strings[i].clone());
        }
        scmd.set_redir_in(strings[255].clone());
        scmd.set_redir_out(strings[256].clone());
        let result = scmd.to_string();
        let mut last_pos = 0;
        for i in 0..257 {
            if i < 255 {
                let pos = result.find(&strings[i]).unwrap();
                assert!(pos >= last_pos);
                last_pos = pos;
            } else if i == 255 {
                let pos = result.find(&strings[i]).unwrap();
                let redir_pos = result.find('<').unwrap();
                assert!(pos > redir_pos);
            } else {
                let pos = result.find(&strings[i]).unwrap();
                let redir_pos = result.find('>').unwrap();
                assert!(pos > redir_pos);
            }
        }
    }
}

#[cfg(test)]
mod pipeline_tests {
    use super::*;

    #[test]
    fn test_push_back_null() {
        let mut pipeline = Pipeline::new();
        pipeline.push_back(SimpleCommand::new());
    }

    #[test]
    fn test_pop_front_null() {
        let mut pipeline = Pipeline::new();
        pipeline.pop_front();
    }

    #[test]
    fn test_pop_front_empty() {
        let mut pipeline = Pipeline::new();
        pipeline.pop_front();
    }

    #[test]
    fn test_set_wait_null() {
        let mut pipeline = Pipeline::new();
        pipeline.set_wait(false);
    }

    #[test]
    fn test_is_empty_null() {
        let pipeline = Pipeline::new();
        assert!(pipeline.is_empty());
    }

    #[test]
    fn test_length_null() {
        let pipeline = Pipeline::new();
        assert_eq!(pipeline.length(), 0);
    }

    #[test]
    fn test_front_null() {
        let pipeline = Pipeline::new();
        assert!(pipeline.front().is_none());
    }

    #[test]
    fn test_front_empty() {
        let pipeline = Pipeline::new();
        assert!(pipeline.front().is_none());
    }

    #[test]
    fn test_get_wait_null() {
        let pipeline = Pipeline::new();
        assert!(pipeline.get_wait());
    }

    #[test]
    fn test_to_string_null() {
        let pipeline = Pipeline::new();
        assert_eq!(pipeline.to_string(), "");
    }

    #[test]
    fn test_adding_emptying() {
        let mut pipeline = Pipeline::new();
        assert!(pipeline.is_empty());
        for _ in 0..257 {
            pipeline.push_back(SimpleCommand::new());
            assert!(!pipeline.is_empty());
        }
        for _ in 0..257 {
            assert!(!pipeline.is_empty());
            pipeline.pop_front();
        }
        assert!(pipeline.is_empty());
    }

    #[test]
    fn test_adding_emptying_length() {
        let mut pipeline = Pipeline::new();
        for i in 0..257 {
            assert_eq!(pipeline.length(), i);
            pipeline.push_back(SimpleCommand::new());
        }
        for i in (0..257).rev() {
            assert_eq!(pipeline.length(), i + 1);
            pipeline.pop_front();
        }
        assert_eq!(pipeline.length(), 0);
    }

    #[test]
    fn test_fifo() {
        let mut pipeline = Pipeline::new();
        let mut commands = Vec::new();
        for _ in 0..257 {
            let cmd = SimpleCommand::new();
            commands.push(cmd);
        }
        for cmd in &commands {
            pipeline.push_back(cmd.clone());
        }
        for cmd in &commands {
            assert_eq!(pipeline.front().unwrap(), cmd);
            pipeline.pop_front();
        }
    }

    #[test]
    fn test_front_idempotent() {
        let mut pipeline = Pipeline::new();
        let cmd = SimpleCommand::new();
        pipeline.push_back(cmd.clone());
        for _ in 0..257 {
            assert_eq!(pipeline.front().unwrap(), &cmd);
        }
    }

    #[test]
    fn test_front_is_back() {
        let mut pipeline = Pipeline::new();
        let cmd = SimpleCommand::new();
        pipeline.push_back(cmd.clone());
        assert_eq!(pipeline.front().unwrap(), &cmd);
    }

    #[test]
    fn test_front_is_not_back() {
        let mut pipeline = Pipeline::new();
        let mut cmd0 = SimpleCommand::new();
        cmd0.push_back(String::from("123"));
        let mut cmd1 = SimpleCommand::new();
        cmd1.push_back(String::from("456"));
        pipeline.push_back(cmd0.clone());
        pipeline.push_back(cmd1.clone());
        assert_ne!(pipeline.front().unwrap(), &cmd1);
    }

    #[test]
    fn test_wait() {
        let mut pipeline = Pipeline::new();
        pipeline.set_wait(true);
        assert!(pipeline.get_wait());
        pipeline.set_wait(false);
        assert!(!pipeline.get_wait());
    }

    #[test]
    fn test_to_string_empty() {
        let pipeline = Pipeline::new();
        assert_eq!(pipeline.to_string(), "");
    }

    #[test]
    fn test_to_string() {
        let mut pipeline = Pipeline::new();
        for _ in 0..257 {
            let mut cmd = SimpleCommand::new();
            cmd.push_back("gtk-fuse".to_string());
            pipeline.push_back(cmd);
        }
        pipeline.set_wait(false);
        let result = pipeline.to_string();
        let pipe_count = result.matches('|').count();
        assert_eq!(pipe_count, 256);
        assert!(result.ends_with('&'));
    }
}

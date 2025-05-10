use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Helper;
use std::fs;

pub struct FilenameCompleter;

impl Completer for FilenameCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let prefix = &line[start..pos];

        // Get files and directories in the current directory
        let entries = fs::read_dir(".")
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect::<Vec<String>>();

        // Filter entries that start with the prefix
        let matches: Vec<Pair> = entries
            .iter()
            .filter(|entry| entry.starts_with(prefix))
            .map(|entry| Pair {
                display: entry.clone(),
                replacement: entry.clone(),
            })
            .collect();

        Ok((start, matches))
    }
}

impl Helper for FilenameCompleter {}
impl Hinter for FilenameCompleter {
    type Hint = String;
}
impl Highlighter for FilenameCompleter {}
impl Validator for FilenameCompleter {}

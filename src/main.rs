mod command;
mod completer;
mod utils;

use command::Pipeline;
use completer::FilenameCompleter;
use rustyline::{Config, Editor, Result};
use utils::get_history_path;

fn main() -> Result<()> {
    let config = Config::builder().auto_add_history(true).build();
    let history = rustyline::sqlite_history::SQLiteHistory::open(config, &get_history_path())?;
    let mut rl: Editor<FilenameCompleter, _> = Editor::with_history(config, history)?;
    rl.set_helper(Some(FilenameCompleter));

    loop {
        let input = rl.readline("> ")?;
        let mut pipeline = Pipeline::parse(&input);
        pipeline.execute();
    }
}

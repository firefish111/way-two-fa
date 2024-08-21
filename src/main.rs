use std::io::{self, stdout}; 

// ratatui
use ratatui::{
  backend::CrosstermBackend,
  crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
  },
  Terminal,
};

mod acc;
mod ui;
mod parse;

use ui::App;
use parse::file::CsvParser;

fn main() -> io::Result<()> {
  stdout().execute(EnterAlternateScreen)?;
  enable_raw_mode()?;
  let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
  term.clear()?;

  let cs = CsvParser {
    filename: String::from("keys.csv"),
  };

  let mut app = App::new(&cs);

  let end = app.run(&mut term);

  stdout().execute(LeaveAlternateScreen)?;
  disable_raw_mode()?;
  end
}

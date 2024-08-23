//! way2fa - A basic TUI 2FA applet

use std::{env, fs, io::{self, stdout}, path::PathBuf};

// ratatui
use ratatui::{
  backend::CrosstermBackend,
  crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
  },
  Terminal,
};

// possible directories
use dirs::*;

mod acc;
mod ui;
mod parse;

use ui::App;
use parse::{file::CsvParser, AccList};

/// Any result can be cast to this using `?`. Typedef'd as a quick shorthand
type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Insulation layer - used to stop errors from exiting the program prematurely.
///
/// While inside this function, terminal is in raw mode.
/// Exiting before the disable_raw_mode call (in `main`) will leave the terminal completely screwed.
/// We therefore do anything that spits out prompts to the user OUTSIDE here and either BEFORE or AFTER the disable_raw_mode call
fn insulation(al: &impl AccList) -> GenericResult<()> {
  /*
    WARNING: TERMINAL BREAKAGE
  */

  let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
  term.clear()?;

  let mut app = App::new(al)?;

  app.run(&mut term)?;
  Ok(()) // no errors
}

const DIR_NAME: &str = "way2fa";
const DEFAULT_FILENAME: &str = "keys.csv";

fn main() -> GenericResult<()> {
  let argv = env::args().skip(1).collect::<Vec<String>>();

  let mut confpath = match config_dir() { 
    Some(pth) => pth,
    None if argv.is_empty() => panic!("No config directory, and no other file given."),
    _ => {
      panic!("WARNING: No config directory");
      PathBuf::new()
    },
  };
  confpath.push(DIR_NAME);
  confpath.push(DEFAULT_FILENAME);

  if !confpath.exists() {
    fs::create_dir(confpath.parent().unwrap())?;
    fs::File::create_new(confpath.clone())?;
  }

  let cs = CsvParser::new(if argv.is_empty() {
    confpath
  } else {
    PathBuf::from(argv[0].clone()) // i know its technically argv[1], but i trimmed the first away earlier, so deal with it
  });

  // normal terminal i/o must cease until terminal is out of raw again

  stdout().execute(EnterAlternateScreen)?;
  enable_raw_mode()?;

  let any_errors = insulation(&cs);

  stdout().execute(LeaveAlternateScreen)?;
  disable_raw_mode()?;
  any_errors?; // errors escape the insulation here

  // from here, we may resume normal operations

  Ok(())
}

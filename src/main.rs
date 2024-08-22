//! way2fa - A basic TUI 2FA applet

use std::{env, io::{self, stdout}};

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
use parse::{file::CsvParser, AccList};

/// Insulation layer - used to stop errors from exiting the program prematurely.
///
/// While inside this function, terminal is in raw mode.
/// Exiting before the disable_raw_mode call (in `main`) will leave the terminal completely screwed.
/// We therefore do anything that spits out prompts to the user OUTSIDE here and either BEFORE or AFTER the disable_raw_mode call
fn insulation(al: &impl AccList) -> Result<(), Box<dyn std::error::Error>> {
  /*
    WARNING: TERMINAL BREAKAGE
  */

  let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
  term.clear()?;

  let mut app = App::new(al)?;

  app.run(&mut term)?;
  Ok(()) // no errors
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cs = CsvParser {
    filename: String::from("keys.csv"),
  };

  // normal terminal i/o must cease until terminal is out of raw again

  stdout().execute(EnterAlternateScreen)?;
  enable_raw_mode()?;

  let any_errors = insulation(&cs);

  stdout().execute(LeaveAlternateScreen)?;
  disable_raw_mode()?;
  any_errors?;

  // from here, we may resume normal operations

  Ok(())
}

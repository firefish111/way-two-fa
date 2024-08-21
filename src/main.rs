use data_encoding::BASE32_NOPAD;
use std::{env, io::{self, stdout, Write}}; 

// ratatui
use ratatui::{
  backend::CrosstermBackend,
  crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
  },
  Terminal,
};

mod acc;
mod ui;
mod parse;
use acc::Account;
use ui::App;


fn main() -> io::Result<()> {
  stdout().execute(EnterAlternateScreen)?;
  enable_raw_mode()?;
  let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
  term.clear()?;

  let mut app = App::new();
  // TODO: csv or secret parsing
  let code = "2FASTEST"; //env::args().skip(1).collect::<Vec<String>>().join("").to_uppercase();
  app.accs.push(Account {
    name: String::from("test"),
    acc_id: Some(String::from("personREAL")),
    key: BASE32_NOPAD.decode(&code.to_string().into_bytes()).unwrap(),
    interv: 30, // 30 seconds
  });

  app.accs.push(Account {
    name: String::from("test2"),
    acc_id: Some(String::from("impostorFAKE")),
    key: BASE32_NOPAD.decode(&"WAAAA234".to_string().into_bytes()).unwrap(),
    interv: 30, // 30 seconds
  });

  let end = app.run(&mut term);

  stdout().execute(LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}

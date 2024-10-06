//! TUI creation - uses the Ratatui library to create the interface

use std::{time, io::{self, Stdout}};

use crate::{acc::Account, parse::{AccList, DataSrc}};

use ratatui::{
  backend::CrosstermBackend,
  crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
  },
  Terminal,
  prelude::*,
  widgets::{*, block::{Position, Title}},
  symbols::border,
  style::Stylize,
};

/// typedef the terminal to `Tty` for QoL
pub type Tty = Terminal<CrosstermBackend<Stdout>>;

/// The current state of the app.
/// 
/// - `quitting` - whether the app will exit next frame
/// - `is_peek` - whether the next tick's codes are displaying
/// - `is_new` - whether the new code window is displaying
/// - `acc_src` - the source file of the accounts
/// - `accs` - list of accounts to display 2FA codes for
pub struct App {
  quitting: bool, 
  is_peek: bool,
  is_new: bool,
  acc_src: DataSrc<String>,
  pub accs: Vec<Account>,
}

use anyhow::{Context, Result};

impl App {
  /// Creates new app, takes an account fetcher as an argument to fetch accounts
  pub fn new(inp: &impl AccList) -> Result<Self> {
    Ok(Self {
      quitting: false,
      is_peek: false,
      is_new: false,
      acc_src: inp.get_src(),
      accs: inp.get_accs().context("Failed to get account list")?,
    })
  }

  /// Renders frame of UI.
  /// It's abstracted into a method to placate borrow checker
  fn render_frame(&mut self, frame: &mut Frame) {
    frame.render_widget(self, frame.area());
  }

  /// Where the app runs - each frame is drawn in this loop.
  pub fn run(&mut self, term: &mut Tty) -> Result<()> {
    while !self.quitting {
      term.draw(|frame| self.render_frame(frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  /// Handle keyboard events.
  /// Keyreleases are basically impossible as most ttys hide them, so only keypresses are detected
  pub fn handle_events(&mut self) -> Result<()> {
    // check if it has been 25ms, to make it non blocking
    if event::poll(std::time::Duration::from_millis(25))? /* 25ms = 40/s */ {
      match event::read()? {
        Event::Key(kev) if kev.kind == KeyEventKind::Press => {
          // manage keydown
          if self.is_new {
            match kev.code {
              KeyCode::Esc => self.is_new = false,
              _ => {}
            }
          } else {
            match kev.code {
              KeyCode::Char('q') => self.quitting = true,
              KeyCode::Char('p') => self.is_peek = !self.is_peek,
              KeyCode::Char('n') => self.is_new = true,
              _ => {}
            }
          }
        },
        _ => {}
      }
    }

    Ok(())
  }
}

/// Width of the border of the main box
const BORDER_WIDTH: u16 = 2_u16;

/// Width of a single code
const CODE_WIDTH: u16 = 9_u16;

/// Minimum padding either side of the progress bar
const PADDING: (u16, u16) = (4_u16, 1_u16); // padding

/// widgets are just lots of components, therefore the whole application is just a big widget
/// also has to be implemented for `&App` to once again placate borrow checker
impl Widget for &mut App {
  /// Meat of the app - render one frame
  fn render(mut self, area: Rect, buf: &mut Buffer) {
    // first thing, layouts
    let layts = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
          Constraint::Min(self.accs.len() as u16 + 2), // 2 for borders
          Constraint::Length(if self.is_new { 10 } else { 0 }),
      ])
      .split(area);

    // current time
    let time = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).expect("Before 1970").as_secs();

    let titl = Title::from(Line::from(vec![
        " ".into(),
        " way2fa ".light_yellow().on_red().bold(),
        " - My OTPs".dark_gray(),
        " ".into(),
      ]))
      .alignment(Alignment::Left);

    let loc_titl = Title::from(Line::from(vec![
        " ".into(),
        format!(" {} ", self.acc_src.unwrap()).light_cyan().bg(if let DataSrc::Msg(_) = self.acc_src { Color::Red } else { Color::Yellow }).bold(),
        " ".into(),
      ]))
      .alignment(Alignment::Center);

    // a bit lengthy
    let instrs = Title::from(Line::from(vec![
        " ".into(),
        " ".on_blue(),
        "Q".light_yellow().on_blue().bold().underlined(),
        "uit ".light_red().on_blue().bold(),
        " | ".into(),
        " ".bg(if self.is_peek { Color::Cyan } else { Color::Blue }),
        "P".light_yellow().bold().underlined().bg(if self.is_peek { Color::Cyan } else { Color::Blue }),
        "eek ".light_red().bold().bg(if self.is_peek { Color::Cyan } else { Color::Blue }),
        " | ".into(),
        " ".bg(if self.is_new { Color::Cyan } else { Color::Blue }),
        "N".light_yellow().bold().underlined().bg(if self.is_new { Color::Cyan } else { Color::Blue }),
        "ew ".light_red().bold().bg(if self.is_new { Color::Cyan } else { Color::Blue }),
        " ".into(),
      ]))
      .position(Position::Bottom)
      .alignment(Alignment::Center);

    let mut blk = Block::bordered()
      .title(titl)
      .title(loc_titl)
      .title(instrs)
      .padding(Padding::horizontal(1))
      .border_set(border::DOUBLE);

    // ONPEEK
    if self.is_peek {
      blk = blk.title(Title::from(Line::from(vec![
        " ".into(),
        " Next Code ".light_green().on_blue().bold(),
        " ".into(),
      ])).alignment(Alignment::Right));
    }

    let mut para = Vec::new();
    let mut progs = Vec::new();

    for ac in &self.accs {
      let interv = ac.interv.unwrap_or(30); // default is 30secs
      let secs = time % interv;
      // actual key
      let ky = ac.gen_key(time / interv);

      let topush = vec![
        ac.name.clone().light_cyan().bold(),
        match ac.acc_id {
          Some(ref unm) => format!(" <@{}>", unm),
          None => String::new(),
        }.light_blue(),
        ": ".into(),
        format!("{:0>3} {:0>3}", ky / 1_000, ky % 1000).bold(),
      ];

      // ONPEEK
      if self.is_peek {
        let peeky = ac.gen_key((time / interv) + 1);
        let peek_where = Rect {
          x: area.width - CODE_WIDTH - PADDING.1 - BORDER_WIDTH, // +1 for padding
          y: (para.len() + 1) as u16 + area.y,
          width: CODE_WIDTH,
          height: 1,
        };
        (format!(" {:0>3} {:0>3} ", peeky / 1_000, peeky % 1000).light_yellow().italic())
          .render(peek_where, buf);
      }
      

      let prog = LineGauge::default()
        .filled_style(Style::default().fg(match interv - secs {
            0..=5 => Color::LightRed,
            _ => Color::LightMagenta
          }))
        .unfilled_style(Style::default().fg(Color::Magenta))
        .label(format!("{:0>2}s", (interv - secs)).fg(match interv - secs {
            11.. => Color::LightGreen,
            5..=10 => Color::Yellow,
            0..5 => Color::LightRed,
          }).bold())
        .ratio((secs as f64) / (interv as f64))
        .line_set(symbols::line::DOUBLE);
      
      progs.push(prog);
      para.push(Line::from(topush));
    }

    // now row number
    let text_len = para.iter().cloned().map(|ln| String::from(ln).len() as u16).max().unwrap_or_default();

    for (rn, gbar) in progs.iter().enumerate() {
      let progbar_where = Rect {
        x: text_len + area.x + PADDING.0 + BORDER_WIDTH, // +1 for padding
        y: (rn + 1) as u16 + area.y,
        width: area.width.saturating_sub(
          text_len + PADDING.0 + PADDING.1 + 2*BORDER_WIDTH + if self.is_peek { CODE_WIDTH + 2 } else { 0 }
        ),
        height: 1,
      };
      gbar.render(progbar_where, buf);
    }

    Paragraph::new(Text::from(para))
      .block(blk)
      .render(layts[0], buf);
  }
}

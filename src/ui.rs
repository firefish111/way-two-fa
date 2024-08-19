use std::{time, io::{self, Stdout}};

use crate::{acc::Account, INTERVAL};

use ratatui::{
  backend::CrosstermBackend,
  crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  },
  Terminal,
  prelude::*,
  widgets::*,
  symbols::border,
  style::Stylize,
};

pub type Tty = Terminal<CrosstermBackend<Stdout>>;

pub struct App {
  quitting: bool, 
  is_peek: bool,
  pub accs: Vec<Account>,
}

impl App {
  pub fn new() -> Self {
    Self {
      quitting: false,
      is_peek: false,
      accs: Vec::new(),
    }
  }

  // abstracted into a method to placate borrow checker
  fn render_frame(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.area());
  }

  pub fn run(&mut self, term: &mut Tty) -> io::Result<()> {
    while !self.quitting {
      term.draw(|frame| self.render_frame(frame));
      //self.handle_events()?;
    }
    Ok(())
  }
}

// widgets are just lots of components, therefore the whole application is just a big widget
// also has to be implemented for reference to it to once again placate borrow checker
impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    // current time
    let time = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).expect("Before 1970").as_secs();
    let secs = time % INTERVAL;

    let blk = Block::bordered()
      .padding(Padding::horizontal(1))
      .border_set(border::DOUBLE);

    let mut para = Vec::new();

    for ac in &self.accs {
      // actual key
      let ky = ac.gen_key(time / INTERVAL);

      let topush = Line::from(vec![
        ac.name.clone().light_cyan().bold(),
        match ac.acc_id {
          Some(ref unm) => format!(" <@{}>", unm),
          None => String::new(),
        }.light_blue(),
        ": ".into(),
        format!("{:0>3} {:0>3}", ky / 1_000, ky % 1000).bold(),
      ]);

      para.push(topush.clone());
      // now row number
      let row_n = para.len() as u16;
      let text_len = String::from(topush).len() as u16;
      const BORDER_WIDTH: u16 = 2_u16;
      const PADDING: (u16, u16) = (4_u16, 1_u16); // padding
      let progbar_where = Rect {
        x: text_len + area.y + PADDING.0 + BORDER_WIDTH, // +1 for padding
        y: row_n + area.x,
        width: area.width - text_len - PADDING.0 - PADDING.1 - 2*BORDER_WIDTH,
        height: 1,
      };
      
//      para.push(Line::from(format!("{}", progbar_where)));
      let prog = LineGauge::default()
        .filled_style(Style::default().fg(match secs {
            25..=30 => Color::LightRed,
            _ => Color::LightMagenta
          }))
        .unfilled_style(Style::default().fg(Color::Magenta))
        .label(format!("{:0>2}s", (INTERVAL - secs)).fg(match secs {
            25..=30 => Color::LightRed,
            20..=24 => Color::Yellow,
            0..=19 => Color::LightGreen,
            _ => Color::Cyan
          }).bold())
        .ratio((secs as f64) / (INTERVAL as f64))
        .line_set(symbols::line::DOUBLE)
        .render(progbar_where, buf);
    }

    Paragraph::new(Text::from(para))
      .block(blk)
      .render(area, buf);
  }
}

use std::{time, io::{self, Stdout}};

use crate::{acc::Account, parse::AccList};

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

pub type Tty = Terminal<CrosstermBackend<Stdout>>;

pub struct App {
  quitting: bool, 
  is_peek: bool,
  pub accs: Vec<Account>,
}

impl App {
  pub fn new(inp: &impl AccList) -> Self {
    Self {
      quitting: false,
      is_peek: false,
      accs: inp.get_accs().unwrap(),
    }
  }

  // abstracted into a method to placate borrow checker
  fn render_frame(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.area());
  }

  pub fn run(&mut self, term: &mut Tty) -> io::Result<()> {
    while !self.quitting {
      term.draw(|frame| self.render_frame(frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  pub fn handle_events(&mut self) -> io::Result<()> {
    // check if it has been 25ms, to make it non blocking
    if event::poll(std::time::Duration::from_millis(25))? /* 25ms = 40/s */ {
      match event::read()? {
        Event::Key(kev) if kev.kind == KeyEventKind::Press => {
          // manage keydown
          match kev.code {
            KeyCode::Char('q') => self.quitting = true,
            KeyCode::Char('p') => self.is_peek = !self.is_peek,
            _ => {}
          }
        },
        _ => {}
      }
    }

    Ok(())
  }
}

const BORDER_WIDTH: u16 = 2_u16;
const CODE_WIDTH: u16 = 9_u16;
const PADDING: (u16, u16) = (4_u16, 1_u16); // padding

// widgets are just lots of components, therefore the whole application is just a big widget
// also has to be implemented for reference to it to once again placate borrow checker
impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    // current time
    let time = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).expect("Before 1970").as_secs();

    let titl = Title::from(Line::from(vec![
        " ".into(),
        " way2fa ".light_yellow().on_red().bold(),
        " - My Codes".dark_gray(),
        " ".into(),
      ]))
      .alignment(Alignment::Left);

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
        " ".on_blue(),
        "N".light_yellow().on_blue().bold().underlined(),
        "ew ".light_red().on_blue().bold(),
        " ".into(),
      ]))
      .position(Position::Bottom)
      .alignment(Alignment::Center);

    let mut blk = Block::bordered()
      .title(titl)
      .title(instrs)
      .padding(Padding::horizontal(1))
      .border_set(border::DOUBLE);

    // ONPEEK
    if self.is_peek {
      blk = blk.title(Title::from(Line::from(vec![
        " ".into(),
        " Next Code ".light_cyan().on_yellow().bold(),
        " ".into(),
      ])).alignment(Alignment::Right));
    }

    let mut para = Vec::new();
    let mut progs = Vec::new();

    for ac in &self.accs {
      let secs = time % ac.interv;
      // actual key
      let ky = ac.gen_key(time / ac.interv);

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
        let peeky = ac.gen_key((time / ac.interv) + 1);
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
        .filled_style(Style::default().fg(match secs {
            25..=30 => Color::LightRed,
            _ => Color::LightMagenta
          }))
        .unfilled_style(Style::default().fg(Color::Magenta))
        .label(format!("{:0>2}s", (ac.interv - secs)).fg(match secs {
            25..=30 => Color::LightRed,
            20..=24 => Color::Yellow,
            0..=19 => Color::LightGreen,
            _ => Color::Cyan,
          }).bold())
        .ratio((secs as f64) / (ac.interv as f64))
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
        width: area.width - text_len - PADDING.0 - PADDING.1 - 2*BORDER_WIDTH - if self.is_peek { CODE_WIDTH + 2 } else { 0 },
        height: 1,
      };
      gbar.render(progbar_where, buf);
    }

    Paragraph::new(Text::from(para))
      .block(blk)
      .render(area, buf);
  }
}

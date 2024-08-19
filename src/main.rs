use data_encoding::BASE32_NOPAD;
use std::{time, thread, env, io, io::Write}; 
use colored::{Colorize, Color};

mod acc;
use acc::Account;

fn split_key(ky: u32) -> String {
  format!("{:0>3} {:0>3}", ky / 1_000, ky % 1000)
}

fn main() -> io::Result<()> {
  const INTERVAL: u64 = 30; // 30 seconds

  let code = env::args().skip(1).collect::<Vec<String>>().join("").to_uppercase();
  let acct = Account {
    name: String::from("test"),
    acc_id: Some(String::from("personREAL")),
    key: BASE32_NOPAD.decode(&code.to_string().into_bytes()).unwrap(),
  };

  let next_exec = time::Duration::from_secs(1); // every 30 secs

  let mut peek = true;

  loop {
    let time = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).expect("Before 1970").as_secs();
    let left = 30 - time % 30;
    print!("{}{}: {}\t\t{}\t\t{:0>2}\r",
      acct.name,
      match acct.acc_id {
        Some(ref unm) => format!(" <@{}>", unm),
        None => String::new(),
      }.bright_blue(),
      split_key(acct.gen_key(time / INTERVAL)).bold(),
      if peek { split_key(acct.gen_key(time / INTERVAL + 1)) } else { String::new() }.truecolor(64, 64, 64),
      left.to_string().color(match left {
        11..=30 => Color::BrightGreen,
        6..=10 => Color::Yellow,
        0..=5 => Color::BrightRed,
        _ => Color::Magenta, /* in theory unreachable */
      }),
    );
    io::stdout().flush()?;

    thread::sleep(next_exec);
  };

  Ok(())
}

use data_encoding::BASE32_NOPAD;
use std::{time, thread, env, io, io::Write}; 

mod acc;
use acc::Account;

fn main() -> io::Result<()> {
  const INTERVAL: u64 = 30; // 30 seconds

  let code = env::args().skip(1).collect::<Vec<String>>().join("").to_uppercase();
  let acct = Account {
    name: String::from("test"),
    acc_id: None,
    key: BASE32_NOPAD.decode(&code.to_string().into_bytes()).unwrap(),
  };

  let next_exec = time::Duration::from_secs(1); // every 30 secs

  loop {
    let time = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).expect("Before 1970").as_secs();
    let otp = acct.gen_key(time / INTERVAL);
    print!("{:0>3} {:0>3}\t\t[{:0>2}]\r", otp / 1_000, otp % 1_000, 30 - (time % 30));
    io::stdout().flush()?;

    thread::sleep(next_exec);
  };

  Ok(())
}

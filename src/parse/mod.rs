pub mod file;

use crate::acc::Account;

pub trait AccList {
  fn get_accs(&self) -> Vec<Account>;
}

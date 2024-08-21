use crate::acc::Account;
use super::AccList;

struct CsvParser;

impl AccList for CsvParser {
  fn get_accs(&self) -> Vec<Account> {
    todo!();
  }
}

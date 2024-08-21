use crate::acc::Account;
use super::AccList;

use csv::Reader;
use std::error::Error;

pub struct CsvParser {
  pub filename: String,
}

impl AccList for CsvParser {
  fn get_accs(&self) -> Result<Vec<Account>, Box<dyn Error>> {
    let mut dat = Reader::from_path(self.filename.clone())?;

    // converting Iterator<Result> to Result<Vec>
    let ret: Result<Vec<Account>, csv::Error> = dat.deserialize().collect();

    // errors need to be in box, so we just cast the error, and rewrap in ok
    Ok(ret?)
  }
}

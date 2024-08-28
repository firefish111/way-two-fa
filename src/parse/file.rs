//! CSV parser

use crate::{acc::Account, GenericResult};
use super::AccList;

use csv::Reader;
use std::{error::Error, path::PathBuf};

/// Reads 2FA keys from a CSV file
pub struct CsvParser {
  filename: PathBuf,
}

impl AccList for CsvParser {
  // doc'd in trait
  fn get_accs(&self) -> GenericResult<Vec<Account>> {
    let mut dat = Reader::from_path(self.filename.clone())?;

    // converting Iterator<Result> to Result<Vec>
    let ret: Result<Vec<Account>, csv::Error> = dat.deserialize().collect();

    // errors need to be boxed, so we just cast the error, and rewrap in ok
    Ok(ret?)
  }

  fn get_src(&self) -> GenericResult<String> {
    self.filename.file_name().map(
      |s| s.to_os_string().into_string().unwrap()
    ).ok_or("<no source available>".into())
  }

  fn write_accs(&self, to_write: Vec<Account>) {
    todo!();
  }
}

impl CsvParser {
  /// Creates a new CSV parser
  ///
  /// also checks if the file in the struct exists or not,
  /// as the file not found errors raised by csv::Reader arent user-friendly
  pub fn new(pt: PathBuf) -> Self {
    if !pt.exists() {
      panic!("Cannot open keyfile \"{}\": no such file or directory", pt.display());
    };

    Self {
      filename: pt,
    }
  }
}

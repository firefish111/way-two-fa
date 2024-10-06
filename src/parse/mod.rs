//! Module containing code for interfacing with storage of 2FA keys 

pub mod file;

use crate::acc::Account;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use data_encoding::BASE32_NOPAD;
use anyhow::Result;

/// serde base32 plugin, used to deserialise the key into Vec<u8>
pub fn base32_plugin<'de, D>(deser: D) -> Result<Vec<u8>, D::Error>
  where D: Deserializer<'de> {
  
  // adding a serde plugin
  String::deserialize(deser)
    .and_then(|code| BASE32_NOPAD.decode(&code.into_bytes())
    .map_err(|err| serde::de::Error::custom(err.to_string())))
}

pub enum DataSrc<T> {
  Msg(T),
  Filename(T),
}

impl<T> DataSrc<T> {
  pub fn unwrap(&self) -> &T {
    match self {
      Self::Msg(x) => x,
      Self::Filename(x) => x,
    }
  }
}

/// Trait for all possible backends that can obtain an Account list
pub trait AccList {
  /// Retrieve accounts from storage
  fn get_accs(&self) -> Result<Vec<Account>>;

  /// Returns a string detailing the source of the data (to go on the titlebar)
  fn get_src(&self) -> DataSrc<String>;

  /// Write accounts to storage
  fn write_accs(&self, to_write: Vec<Account>);
}

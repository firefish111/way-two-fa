//! Module containing code for interfacing with storage of 2FA keys 

pub mod file;

use crate::acc::Account;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use data_encoding::BASE32_NOPAD;

/// serde base32 plugin, used to deserialise the key into Vec<u8>
pub fn base32_plugin<'de, D>(deser: D) -> Result<Vec<u8>, D::Error>
  where D: Deserializer<'de> {
  
  // adding a serde plugin
  String::deserialize(deser)
    .and_then(|code| BASE32_NOPAD.decode(&code.into_bytes()).map_err(|err| serde::de::Error::custom(err.to_string())))
}

/// Trait for all possible backends that can obtain an Account list
pub trait AccList {
  /// Retrieve accounts from storage
  fn get_accs(&self) -> Result<Vec<Account>, Box<dyn std::error::Error>>;

  /// Write accounts to storage
  fn write_accs(&self, to_write: Vec<Account>);
}

pub mod file;

use crate::acc::Account;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use data_encoding::BASE32_NOPAD;

// serde base32
pub fn base32_plugin<'de, D>(deser: D) -> Result<Vec<u8>, D::Error>
  where D: Deserializer<'de> {
  
  // adding a serde plugin
  String::deserialize(deser)
    .and_then(|code| BASE32_NOPAD.decode(&code.into_bytes()).map_err(|err| serde::de::Error::custom(err.to_string())))
//        base64::decode(&string).map_err(|err| serde::de::Error::custom(err.to_string()))
}

pub trait AccList {
  fn get_accs(&self) -> Result<Vec<Account>, Box<dyn std::error::Error>>;
}

//! 2FA Code generation functions

use sha1::Sha1;
use hmac::{Hmac, Mac};

use serde::Deserialize;
use crate::parse::base32_plugin;

type Hasher = Hmac<Sha1>;

/// Struct for a 2FA account (a single entry)
#[derive(Debug, Deserialize)]
pub struct Account {
  pub name: String,
  pub acc_id: Option<String>,

  #[serde(deserialize_with = "base32_plugin")]
  pub key: Vec<u8>,
  pub interv: Option<u64>, // most often is 30
}

impl Account {
  /// Generate a specific six-digit code. (exactly which is `nth`)
  /// `nth` is typically `CurrUnixTime/30`, or one more than that if peeking.
  pub fn gen_key(&self, nth: u64) -> u32 {
    let mut codegen = Hasher::new_from_slice(&self.key).expect("HMAC can take key of any size");

    codegen.update(&nth.to_be_bytes());

    let total_hmac: [u8; 20] = codegen.finalize().into_bytes().into();
    let ix: usize = (total_hmac[19] & 0xf) as usize;

    (u32::from_be_bytes(total_hmac[ix..ix+4].try_into().unwrap()) & 0x7f_ff_ff_ff) % 1_000_000
  }
}

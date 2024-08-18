use sha1::Sha1;
use hmac::{Hmac, Mac};

type Hasher = Hmac<Sha1>;

pub struct Account {
  pub name: String,
  pub acc_id: Option<String>,
  pub key: Vec<u8>,
}

impl Account {
  pub fn gen_key(&self, nth: u64) -> u32 {
    let mut codegen = Hasher::new_from_slice(&self.key).expect("HMAC can take key of any size");

    codegen.update(&nth.to_be_bytes());

    let total_hmac: [u8; 20] = codegen.finalize().into_bytes().into();
    let ix: usize = (total_hmac[19] & 0xf) as usize;

    (u32::from_be_bytes(total_hmac[ix..ix+4].try_into().unwrap()) & 0x7f_ff_ff_ff) % 1_000_000
  }
}

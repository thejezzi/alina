use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum CharacterSet {
  Standard,
  UrlSafe,
  Crypt,
  Bcrypt,
  ImapMutf7,
  BinHex,
}

impl CharacterSet {
  pub fn as_base64_alphabet(&self) -> base64::alphabet::Alphabet {
    match self {
      CharacterSet::Standard => base64::alphabet::STANDARD,
      CharacterSet::UrlSafe => base64::alphabet::URL_SAFE,
      CharacterSet::Crypt => base64::alphabet::CRYPT,
      CharacterSet::Bcrypt => base64::alphabet::BCRYPT,
      CharacterSet::ImapMutf7 => base64::alphabet::IMAP_MUTF7,
      CharacterSet::BinHex => base64::alphabet::BIN_HEX,
    }
  }
  pub fn create_engine(&self) -> base64::engine::fast_portable::FastPortable {
    base64::engine::fast_portable::FastPortable::from(
      &self.as_base64_alphabet(),
      base64::engine::fast_portable::NO_PAD)
  }
}
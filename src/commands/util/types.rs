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
  pub fn as_base64_config(&self) -> base64::Config {
    match self {
      CharacterSet::Standard => base64::STANDARD,
      CharacterSet::UrlSafe => base64::URL_SAFE,
      CharacterSet::Crypt => base64::CRYPT,
      CharacterSet::Bcrypt => base64::BCRYPT,
      CharacterSet::ImapMutf7 => base64::IMAP_MUTF7,
      CharacterSet::BinHex => base64::BINHEX,
    }
  }
}
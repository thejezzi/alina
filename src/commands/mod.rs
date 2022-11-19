pub mod code;
pub mod util;
pub mod config;

use clap::Subcommand;

use self::code::CodeArgs;
use self::util::UtilArgs;
use self::config::ConfigArgs;

#[derive(Debug, Clone, Subcommand)]
pub enum CommandType {
  /// Manage all your code
  Code(CodeArgs),
  /// Use several util commands like base64 encoding ...
  Util(UtilArgs),
  /// Change paths for code dir and template path and other settings
  Config(ConfigArgs),
}
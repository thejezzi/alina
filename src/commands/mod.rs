pub mod code;
pub mod util;

use clap::Subcommand;

use self::code::CodeArgs;
use self::util::UtilArgs;

#[derive(Debug, Clone, Subcommand)]
pub enum CommandType {
  /// Manage all your code
  Code(CodeArgs),
  /// Use several util commands like base64 encoding ...
  Util(UtilArgs)
}
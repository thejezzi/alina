pub mod code;
pub mod config;
pub mod note;
pub mod util;

use clap::Subcommand;

use self::code::CodeArgs;
use self::config::ConfigArgs;
use self::note::NoteArgs;
use self::util::UtilArgs;

#[derive(Debug, Clone, Subcommand)]
pub enum CommandType {
    /// Manage all your code
    Code(CodeArgs),
    /// Use several util commands like base64 encoding ...
    Util(UtilArgs),
    /// Change paths for code dir and template path and other settings
    Config(ConfigArgs),
    /// Manage your notes
    Note(NoteArgs),
}

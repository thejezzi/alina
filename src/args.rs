use crate::commands::CommandType;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command_type: Option<CommandType>,

    #[clap(short, long, default_value = "false")]
    /// Show current version number
    pub version: bool,
}

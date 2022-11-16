use clap::Parser;
use crate::commands::CommandType;

#[derive(Debug, Parser)]
pub struct HurrixArgs {
  #[clap(subcommand)]
  pub command_type: CommandType,
}
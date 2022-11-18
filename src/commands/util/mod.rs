pub mod base64;
pub mod types;

use clap::Parser;
use self::base64::UtilBase64Args;

#[derive(Debug, Clone, Parser)]
pub struct UtilArgs {
    #[clap(subcommand)]
    pub subcommand: Option<UtilSubCommand>,
}

#[derive(Debug, Clone, Parser)]
pub enum UtilSubCommand {
    /// Encode or decode base64 in different text formats
    Base64(UtilBase64Args),
}

pub fn exec(args: &UtilArgs) {
  // destructure the args
  let UtilArgs { subcommand } = args;
  // match the subcommand
  match subcommand {
      Some(UtilSubCommand::Base64(new_args)) => base64::exec(new_args),
      None => (),
  };
}
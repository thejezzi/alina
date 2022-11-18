pub mod base64;
pub mod string;
pub mod types;

use self::base64::UtilBase64Args;
use self::string::UtilStringArgs;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct UtilArgs {
    #[clap(subcommand)]
    pub subcommand: Option<UtilSubCommand>,
}

#[derive(Debug, Clone, Parser)]
pub enum UtilSubCommand {
    /// Encode or decode base64 in different text formats
    Base64(UtilBase64Args),
    String(UtilStringArgs),
}

pub fn exec(args: &UtilArgs) {
    // destructure the args
    let UtilArgs { subcommand } = args;
    // match the subcommand
    match subcommand {
        Some(UtilSubCommand::Base64(the_args)) => base64::exec(the_args),
        Some(UtilSubCommand::String(the_args)) => string::exec(the_args),
        None => (),
    };
}

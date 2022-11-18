use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct UtilStringArgs {
    #[clap(subcommand)]
    pub subcommand: Operations,
}

#[derive(Debug, Clone, Parser)]
pub enum Operations {
    /// Get the length of a string
    Length(LengthArgs),
}

/// main function for the string command
pub fn exec(args: &UtilStringArgs) {
    let UtilStringArgs { subcommand } = args;

    match subcommand {
        Operations::Length(length_args) => length(length_args),
    }
}

#[derive(Debug, Clone, Parser)]
pub struct LengthArgs {
    /// The string to get the length of
    pub string: String,
}

fn length(args: &LengthArgs) {
    let LengthArgs { string } = args;

    println!("{}", string.len());
}
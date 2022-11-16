pub mod args;
pub mod commands;
pub mod lib;

use args::HurrixArgs;
use clap::Parser;
use commands::CommandType;

fn main() {
    let args = HurrixArgs::parse();

    match args.command_type {
        CommandType::Test(test_args) => commands::test::exec(&test_args),
    }
}

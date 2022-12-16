extern crate base64;

pub mod args;
pub mod commands;

use args::Args;
use clap::Parser;
use commands::CommandType;

fn main() {
    let args = Args::parse();

    if args.version {
        println!("Alina v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let command_type = match args.command_type {
        Some(command_type) => command_type,
        _ => {
            println!("No command specified. Use --help for more information.");
            use clap::CommandFactory;
            let mut cmd = Args::command();
            cmd.print_help().expect("Failed to print help");
            return;
        }
    };

    match command_type {
        CommandType::Code(code_args) => commands::code::exec(&code_args),
        CommandType::Util(util_args) => commands::util::exec(&util_args),
        CommandType::Config(config_args) => commands::config::exec(&config_args),
        CommandType::Note(note_args) => commands::note::exec(&note_args),
    }
}

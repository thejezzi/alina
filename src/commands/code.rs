use std::path::Path;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct CodeArgs {
    #[clap(subcommand)]
    pub subcommand: Option<CodeSubCommand>,
}

#[derive(Debug, Clone, Parser)]
pub enum CodeSubCommand {
    /// Create a new project in rust
    New(CodeNewArgs),
}

pub fn exec(args: &CodeArgs) {
    // destructure the args
    let CodeArgs { subcommand } = args;
    // match the subcommand
    match subcommand {
        Some(CodeSubCommand::New(new_args)) => new(new_args),
        None => (),
    };
}

#[derive(Debug, Clone, Parser)]
pub struct CodeNewArgs {
    /// The name of the project
    pub name: String,

    #[clap(short, long)]
    pub lang: Option<String>,

    #[clap(short, long)]
    pub template: Option<String>,
}

fn new(args: &CodeNewArgs) {
    use clipboard::ClipboardContext;
    use clipboard::ClipboardProvider;

    let mut cl_ctx: ClipboardContext =
        ClipboardProvider::new().expect("Failed to get clipboard context");

    // destructure the args
    let CodeNewArgs {
        name,
        lang,
        template,
    } = args;
    // do something with the args
    let _path = Path::new("C:\\test");
    cl_ctx
        .set_contents(_path.to_str().unwrap().to_string())
        .expect("Failed to set clipboard");
    ferrisprint!(
        "Created a new project\nwith the name {}.\nCopied path to clipboard",
        name
    );
}

fn list() {
    unimplemented!("list");
}

fn remove() {
    unimplemented!("remove");
}

fn export() {
    unimplemented!("export");
}

fn config() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

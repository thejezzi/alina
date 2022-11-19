use clap::Parser;

use crate::lib;

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
    pub name: Option<String>,

    #[clap(short, long, default_value = "rust")]
    pub lang: String,

    #[clap(short, long)]
    pub template: Option<String>,
    
    #[clap(short, long, default_value="false")]
    pub random: bool,

    #[clap(short, long, default_value="false")]
    pub with_vscode: bool,
}

fn new(args: &CodeNewArgs) {
    

    // destructure the args
    let CodeNewArgs {
        name: _,
        lang: _,
        template: _,
        random,
        with_vscode
    } = args;

    let cnf = lib::config::load_config();

    if *random {
        println!("Generating random rust project ...");

        // generate random 6 character string
        let random_string = lib::util::random_string(6);

        // create new rust project with random name in code directory
        let mut code_dir = cnf.code_dir;
        
        // add rust/random string to code directory and append random string
        code_dir.push("rust");
        code_dir.push("random");
        code_dir.push(&random_string);

        // run cargo command
        let cargo_command = format!("cargo new {}", code_dir.to_str().unwrap());
        std::process::Command::new("cmd")
            .args(&["/C", &cargo_command])
            .output()
            .expect("Failed to execute command");
        
        // inform user that project was created
        // TODO: maybe add a better message to clarify what was created and where
        println!("Done!");

        // Convert code_dir to string
        let code_dir_str = match code_dir.to_str() {
            Some(s) => s.to_string(),
            _ => String::from(""),
        };
        // and copy it to clipboard so the user access the directory easily
        lib::util::copy_to_clipboard(code_dir_str);

        // open vscode if requested makes cd-ing unnecessary
        if *with_vscode {
            let mut vscode_command = String::from("code ");
            vscode_command.push_str(code_dir.to_str().unwrap());
            std::process::Command::new("cmd")
                .args(&["/C", &vscode_command])
                .output()
                .expect("Failed to execute command");
        }

        return;
    }
    
      


}

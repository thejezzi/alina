use clap::Parser;

use alina::ops::Languages;

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

    #[clap(short, long, value_enum, default_value_t=Languages::Rust)]
    pub lang: Languages,

    #[clap(short, long)]
    pub template: Option<String>,

    #[clap(short, long, default_value = "false")]
    pub random: bool,

    #[clap(short, long, default_value = "false")]
    pub with_vscode: bool,
}

fn new(args: &CodeNewArgs) {
    // destructure the args
    let CodeNewArgs {
        name: _,
        lang,
        template: _,
        random,
        with_vscode,
    } = args;

    let cnf = alina::config::load_config();
    let mut code_dir = cnf.code_dir;
    code_dir.push(lang.to_string());

    if *random {
        println!("Generating random {} project ...", lang);

        // generate random 6 character string
        let random_string = alina::util::random_string(6);
        println!("Random name is: {}", random_string);

        // add rust/random string to code directory and append random string
        code_dir.push("random");
        code_dir.push(&random_string);
    }

    // if no name was provided, ask for one
    let _name = match &args.name {
        Some(s) => s.to_string(),
        None => {
            let mut name = alina::util::random_string(6);
            if !*random {
                let msg = vec![
                    "No name was provided!\n",
                    "\n",
                    "If this was done on purpose consider to\n",
                    "quit and use the '-r' flag to generate a\n",
                    "random name for your project. You can\n",
                    "continue by providing a name for your project.\n",
                ];
                println!("{}", msg.join(""));
                // Ask if the user he wants quit or not
                let a_continue = alina::util::ask_yes_no("Do you want to continue? [y/n]");
                if !a_continue {
                    println!("Quitting ...");
                    std::process::exit(0);
                }
                println!("Please enter a name for the project:");
                std::io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
            }
            name
        }
    };

    // run cargo command
    let cargo_command = format!("cargo new {}", code_dir.to_str().unwrap());
    let output = std::process::Command::new("cmd")
        .args(["/C", &cargo_command])
        .output()
        .expect("Failed to execute command");

    println!("cargo: {}", String::from_utf8_lossy(&output.stdout));

    // inform user that project was created
    // TODO: maybe add a better message to clarify what was created and where
    println!("Done!");

    // Convert code_dir to string
    let code_dir_str = match code_dir.to_str() {
        Some(s) => s.to_string(),
        _ => String::from(""),
    };
    // and copy it to clipboard so the user access the directory easily
    alina::util::copy_to_clipboard(code_dir_str);

    // open vscode if requested makes cd-ing unnecessary
    if *with_vscode {
        let mut vscode_command = String::from("code ");
        vscode_command.push_str(code_dir.to_str().unwrap());
        std::process::Command::new("cmd")
            .args(["/C", &vscode_command])
            .output()
            .expect("Failed to execute command");
    }
}

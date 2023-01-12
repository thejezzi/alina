use chrono::{DateTime, Utc};
use clap::Parser;
use colored::*;
use std::fs;

#[derive(Debug, Clone, Parser)]
pub struct NoteArgs {
    #[clap(subcommand)]
    pub subcommand: Option<NoteSubCommand>,
}

#[derive(Debug, Clone, Parser)]
pub enum NoteSubCommand {
    /// Create a new note
    New(NoteNewArgs),
    /// List all notes
    List,
    /// Open a note
    Open,
    /// Delete a note
    Delete,
    /// Edit a note
    Edit(NoteEditArgs),
}

pub fn exec(args: &NoteArgs) {
    // destructure the args
    let NoteArgs { subcommand } = args;
    // match the subcommand
    match subcommand {
        Some(NoteSubCommand::New(new_args)) => new(new_args),
        Some(NoteSubCommand::List) => list(),
        Some(NoteSubCommand::Open) => log_unimplemented(),
        Some(NoteSubCommand::Delete) => log_unimplemented(),
        Some(NoteSubCommand::Edit(args)) => edit(args),
        None => (),
    };
}

#[derive(Debug, Clone, Parser)]
pub struct NoteNewArgs {
    /// The name of the note
    pub name: Option<String>,

    #[clap(short, long, default_value = "false")]
    pub copy_to_clipboard: bool,
}

fn new(args: &NoteNewArgs) {
    // destructure the args
    let NoteNewArgs {
        name,
        copy_to_clipboard,
    } = args;

    if name.is_none() {
        println!("You didn't provide a name for the note");
        match dialoguer::Confirm::new()
            .with_prompt("Do you want to continue?")
            .interact()
        {
            Ok(bool) => {
                if !bool {
                    println!("{}", "Aborting ...".red());
                    return;
                }
            }
            Err(_) => {
                println!("{}", "Aborting ...".red());
                return;
            }
        }
    }

    let name: String = match name {
        Some(name) => name.to_string(),
        None => {
            let name = dialoguer::Input::<String>::new()
                .with_prompt("Please enter a name for the note")
                .interact()
                .expect("Failed to get input");
            name
        }
    };

    let cnf = alina::config::load_config();
    let mut note_dir = *cnf.code_dir;
    note_dir.push("notes");
    let note_file_name = format!("{}.md", name);
    note_dir.push(note_file_name);
    // test if the file exists
    if note_dir.exists() {
        println!("Note already exists");
        return;
    }
    fs::File::create(note_dir.clone()).expect("Failed to create note");

    let note_dir_str = note_dir
        .to_str()
        .expect("Failed to convert note dir to string");

    println!("{}", "Note created at:");
    println!();
    println!("   {}", note_dir_str.yellow());
    if *copy_to_clipboard {
        alina::util::copy_to_clipboard(note_dir_str.to_string());
        println!("   {}", "Copied to clipboard".truecolor(169, 169, 169));
    }
    println!();

    // Ask if user wants to edit the note
    let edit = dialoguer::Confirm::new()
        .with_prompt("Do you want to edit the note?")
        .interact()
        .expect("Failed to get input");

    // TODO: implement an option where the user can specify the editor in the config
    // TODO: If the user didn't specify an editor in the config, search for the default editor
    // TODO: If no default editor was found, search for common editors like vim, nano, helix, notepad, gedit, etc.
    // TODO: If the user didn't specify an editor in the config, ask the user which editor to use

    if edit {
        println!("{}", "Opening note in editor ...".green());
        alina::util::open_file_in_editor(&note_dir);
    }
}

fn list() {
    let cnf = alina::config::load_config();
    let mut note_dir = *cnf.code_dir;
    note_dir.push("notes");

    let note_files = fs::read_dir(note_dir).expect("Failed to read notes directory");
    for note_file in note_files {
        let note_file = note_file.expect("Failed to read note file");
        let note_file_name = note_file.file_name();
        let updated_at = note_file
            .metadata()
            .expect("Failed to read note file metadata")
            .modified()
            .expect("Failed to read note file modified date");
        let updated_at_datetime: DateTime<Utc> = updated_at.into();
        let note_file_name = note_file_name
            .to_str()
            .expect("Failed to convert note file name to string");
        println!(
            "{} {} :: {}",
            updated_at_datetime
                .format("%d.%m.%Y")
                .to_string()
                .black()
                .on_bright_cyan(),
            updated_at_datetime
                .format("%H:%M")
                .to_string()
                .black()
                .on_bright_yellow(),
            note_file_name
        );
    }
}

#[derive(Debug, Clone, Parser)]
pub struct NoteEditArgs {
    /// The name of the note
    pub name: Option<String>,
}

pub fn edit(args: &NoteEditArgs) {
    let NoteEditArgs { name } = args;

    if let None = name {
        println!("Name not provided");
        let cnf = alina::config::load_config();
        let mut note_dir = *cnf.code_dir;
        note_dir.push("notes");
        let files: Vec<String> = fs::read_dir(note_dir)
            .expect("Failed to read notes directory")
            .map(|entry| {
                let entry = entry.expect("Failed to read note file");
                let note_file_name = entry.file_name();
                let note_file_name = note_file_name
                    .to_str()
                    .expect("Failed to convert note file name to string");
                note_file_name.to_string()
            })
            .collect();
        dialoguer::Select::new()
            .with_prompt("Please select a note to edit")
            .items(&files)
            .interact()
            .expect("Failed to get input");

        // let file_name_chosen = files[result].clone();
        return;
    }
}

pub fn log_unimplemented() {
    println!("This feature is not yet implemented");
}

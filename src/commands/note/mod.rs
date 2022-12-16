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
    Edit,
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
        Some(NoteSubCommand::Edit) => log_unimplemented(),
        None => (),
    };
}

#[derive(Debug, Clone, Parser)]
pub struct NoteNewArgs {
    /// The name of the note
    pub name: Option<String>,
}

fn new(args: &NoteNewArgs) {
    // destructure the args
    let NoteNewArgs { name } = args;

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

pub fn log_unimplemented() {
    println!("This feature is not yet implemented");
}

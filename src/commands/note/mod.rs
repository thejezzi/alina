use std::fs;
use clap::Parser;

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
        Some(NoteSubCommand::List) => log_unimplemented(),
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

    let cnf = alina::config::load_config();
    let mut note_dir = *cnf.code_dir;
    note_dir.push("notes");
    if let Some(name) = name {
        let note_file_name = format!("{}.md", name);
        note_dir.push(note_file_name);
    }
    println!("Creating note at {:?}", note_dir);
    // test if the file exists
    if note_dir.exists() {
        println!("Note already exists");
        return;
    }
    fs::File::create(note_dir).expect("Failed to create note");
}

pub fn log_unimplemented() {
    println!("This feature is not yet implemented");
}
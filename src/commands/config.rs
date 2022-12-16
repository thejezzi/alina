use clap::Parser;
use colored::*;
use std::str::FromStr;

use alina::{self, config::save_config};

#[derive(Debug, Clone, Parser)]
pub struct ConfigArgs {
    #[clap(subcommand)]
    pub subcommand: Operations,
}

#[derive(Debug, Clone, Parser)]
pub enum Operations {
    /// Lists the current config
    List,
    /// Change the default configuration
    Set(SetArgs),
    /// Removes the config file and therefore resets the config to default
    Reset,
}

pub fn exec(args: &ConfigArgs) {
    // destructure the args
    let ConfigArgs { subcommand } = args;
    // match the subcommand
    match subcommand {
        Operations::List => list(),
        Operations::Set(the_args) => set(the_args),
        Operations::Reset => reset(),
    };
}

#[derive(Debug, Clone, Parser)]
pub struct SetArgs {
    /// Takes key value pairs to set the config like so , key=value
    pub key_value: String,
}

fn set(args: &SetArgs) {
    let SetArgs { key_value } = args;
    let mut cnf = alina::config::load_config();
    let key_value_vec = key_value.split('=').collect::<Vec<&str>>();
    let key = key_value_vec[0];
    let value = key_value_vec[1];

    use alina::config::ops::create_cnf_path;

    match key {
        "code_dir" => cnf.code_dir = create_cnf_path(value),
        "template_dir" => cnf.template_dir = create_cnf_path(value),
        "editor" => {
            cnf.editor = alina::config::Editor::from_str(value).expect("Failed to parse editor")
        }
        "editor_path" => cnf.editor_path = Some(value.to_string()),
        _ => {
            println!();
            println!(" \"{}\" is not a valid key.", key.red());
            println!();
            print_possible_keys();
            return;
        }
    }
    println!("Setting {} to {}", key, value);
    save_config(cnf).expect("Failed to save config");
}

fn list() {
    let cnf = alina::config::load_config();
    print_config(&cnf);
}

fn reset() {
    alina::config::reset_config().expect("Failed to reset config");
    println!("Config was reset to default");
}

fn print_possible_keys() {
    let cnf_iter = alina::config::Config::new().iter();
    println!("Possible keys are:");
    for (key, _) in cnf_iter {
        println!(" - {}", key.green());
    }
}

fn print_config(cnf: &alina::config::Config) {
    let cnf_iterator = cnf.iter();
    let longest_word: usize = cnf_iterator
        .clone()
        .map(|(key, _)| key.len())
        .max()
        .expect("Failed to get longest word");

    for (key, value) in cnf_iterator {
        let spaces = " ".repeat(longest_word - key.len() + 2);
        println!("{}:{}{}", key.green(), spaces, value.blue());
    }
}

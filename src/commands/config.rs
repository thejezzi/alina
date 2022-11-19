use clap::Parser;

use crate::lib::{self, config::save_config};

#[derive(Debug, Clone, Parser)]
pub struct ConfigArgs {
    #[clap(subcommand)]
    pub subcommand: Operations,
}

#[derive(Debug, Clone, Parser)]
pub enum Operations {
    /// Change the default configuration
    Set(SetArgs),
    /// Removes the config file and therefore resets the config to default
    Reset
}

pub fn exec(args: &ConfigArgs) {
    // destructure the args
    let ConfigArgs { subcommand } = args;
    // match the subcommand
    match subcommand {
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
    let mut cnf = lib::config::load_config();
    let key_value_vec = key_value.split('=').collect::<Vec<&str>>();
    let key = key_value_vec[0];
    let value = key_value_vec[1];

    println!("Setting {} to {}", key, value);

    use lib::config::ops::create_cnf_path;

    match key {
        "code_dir" => cnf.code_dir = create_cnf_path(value),
        "template_dir" => cnf.template_dir = create_cnf_path(value),
        _ => println!("Invalid key"),
    }

    save_config(cnf).expect("Failed to save config");
}

fn reset () {
    lib::config::reset_config().expect("Failed to reset config");
    println!("Config was reset to default");
}


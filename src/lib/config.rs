#![allow(non_snake_case)]
use std::fs;

extern crate serde_yaml;
use super::consts;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Config {
    pub code_dir: Box<PathBuf>,
    pub template_dir: Box<PathBuf>,
}

/// Loads the config file if it exists, otherwise returns the default config
pub fn load_config() -> Config {
    // test if .alina.yaml exists in home directory
    // if not create it with default values
    // if yes load it

    //test if file exists
    let home = dirs::home_dir().unwrap();
    let config_path = home.join(".alina.yaml");
    let config_exists = config_path.exists();

    if !config_exists {
        return std_config();
    }
    read_config()
}

/// Returns the default config whereas the code directory is a path to a code \
/// folder in the home directory same goes for the template directory.
///
pub fn std_config() -> Config {
    // get home directory
    let current_home_dir = dirs::home_dir().expect("Failed to get home directory");

    // create path to code directory
    let mut code_dir = Box::new(current_home_dir.to_owned());
    code_dir.push(consts::CODE_DIR_NAME);

    // create path to template directory
    let mut template_dir = Box::new(current_home_dir);
    template_dir.push(consts::TEMPLATE_DIR_NAME);

    // create config struct
    Config {
        code_dir,
        template_dir,
    }
}

/// If the user wishes to change the default config, this function can be used \
/// to write a new config file to the home directory.
pub fn create_config() -> Result<(), Box<dyn Error>> {
    // get home directory
    let current_home_dir = dirs::home_dir().expect("Failed to get home directory");

    // create path to config file
    let config_path = current_home_dir.join(".alina.yaml");

    // get the default config values
    let config = std_config();

    // write config to file
    let config_string = serde_yaml::to_string(&config)?;
    fs::write(config_path, config_string)?;

    Ok(())
}

/// Reads the config file and parses it into yaml which is then deserialized \
/// into a Config struct.
pub fn read_config() -> Config {
    let home = dirs::home_dir().unwrap();
    let config_path = home.join(".alina.yaml");
    let config_exists = config_path.exists();

    if !config_exists {
        return std_config();
    }

    let config_content = fs::read_to_string(config_path).expect("Failed to read config file");
    let config: Config =
        serde_yaml::from_str(&config_content).expect("Failed to parse config file");
    config
}

/// Takes a Config struct and writes it to the config file. Is used to update \
/// the config file.
pub fn update_config(config: Config) -> Result<(), Box<dyn Error>> {
    let home = dirs::home_dir().unwrap();
    let config_path = home.join(".alina.yaml");
    let config_exists = config_path.exists();

    if !config_exists {
        return Err("Config file does not exist".into());
    }

    let config_string = serde_yaml::to_string(&config)?;
    fs::write(config_path, config_string)?;

    Ok(())
}

pub fn save_config(config: Config) -> Result<(), Box<dyn Error>> {
    let home = dirs::home_dir().unwrap();
    let config_path = home.join(".alina.yaml");
    let config_exists = config_path.exists();

    if !config_exists {
        return create_config();
    }
    update_config(config)?;

    Ok(())
}

pub fn reset_config() -> Result<(), Box<dyn Error>> {
    let home = dirs::home_dir().unwrap();
    let config_path = home.join(".alina.yaml");
    let config_exists = config_path.exists();

    if !config_exists {
        println!("Config file does not exist");
        return Ok(());
    }
    fs::remove_file(config_path)?;

    Ok(())
}

pub mod ops {
    use std::path::PathBuf;

    /// Converts a string to a PathBuf allocated on the heap
    pub fn create_cnf_path(path: &str) -> Box<PathBuf> {
        let cnf_path = Box::new(PathBuf::from(path));
        // Check if path is valid
        if !cnf_path.exists() {
            panic!("Path does not exist");
        }
        cnf_path
    }
}

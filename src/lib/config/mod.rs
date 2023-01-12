#![allow(non_snake_case)]
use std::fs;

extern crate serde_yaml;
use super::consts;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

pub mod editor;
use editor::Editor;

/// This struct is used to represent the config file. It is used to define the default directories
/// and the default editor. The editor path is only used if the editor is set to "Other" and then
/// the user can define the path to the editor.
///
/// !IMPORTANT! If you define a new field you also have to add it to the iterator implementation \
/// !IMPORTANT! and it also has to have a default value in the new function aaand it has to \
/// !IMPORTANT! implement the Display trait and the FromStr trait if it is not a string.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub code_dir: Box<PathBuf>,
    pub template_dir: Box<PathBuf>,
    pub editor: Editor,
    pub editor_path: Option<String>,
}

/// This iterator struct is only used to implement the iterator trait for the config struct because
/// we need an index to tell where we are in the iterator. Pretty obvious.
#[derive(Clone)]
pub struct ConfigIterator {
    index: usize,
    config: Config,
}

impl Config {
    pub fn new() -> Config {
        Config {
            code_dir: Box::new(PathBuf::from(consts::CODE_DIR_NAME)),
            template_dir: Box::new(PathBuf::from(consts::TEMPLATE_DIR_NAME)),
            editor: Editor::None,
            editor_path: None,
        }
    }
    pub fn iter(&self) -> ConfigIterator {
        ConfigIterator {
            index: 0,
            config: self.clone(),
        }
    }
}

/// The iterator implementation for the config struct is used to print out all keys and values so
/// the user can see what he or she is able to define. Better than git which just tells you to read
/// the documentation online.
impl Iterator for ConfigIterator {
    type Item = (&'static str, String);

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                Some((
                    "code_dir",
                    self.config
                        .code_dir
                        .to_str()
                        .expect("Failed to convert the code dir to string")
                        .to_string(),
                ))
            }
            1 => {
                self.index += 1;
                Some((
                    "template_dir",
                    self.config
                        .template_dir
                        .to_str()
                        .expect("Failed to convert the template dir to string")
                        .to_string(),
                ))
            }
            2 => {
                self.index += 1;
                Some(("editor", self.config.editor.to_string()))
            }
            3 => {
                self.index += 1;
                match &self.config.editor_path {
                    Some(path) => Some(("editor_path", path.to_string())),
                    None => ("editor_path", String::from("none")).into(),
                }
            }
            _ => None,
        }
    }
}

/// Loads the config file if it exists, otherwise returns the default config
pub fn load_config() -> Config {
    // test if .alina.yaml exists in home directory
    // if not create it with default values
    // if yes load it

    //test if file exists
    let home = dirs::home_dir().expect("Homedir should be defined but is apparently empty.");
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

    let editor = Editor::None;

    let editor_path = None;

    // create config struct
    Config {
        code_dir,
        template_dir,
        editor,
        editor_path,
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

    /// Checks if editor is defined in config file
    pub fn check_if_editor_defined() -> bool {
        let cnf = super::load_config();
        if cnf.editor == super::Editor::None {
            return false;
        }
        true
    }
}

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// This enum is used to represent the different editors that can be defined by default.
/// But if anyone uses a different editor there is also a "Other" option whereas the user can define the path to the editor
/// which will then be used to open the files.
#[derive(PartialEq, Eq, Debug, EnumString, Serialize, Deserialize, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Editor {
    Atom,
    Emacs,
    Gedit,
    Helix,
    Nano,
    NotePad,
    Sublime,
    Vim,
    VSCode,

    Other,
    None,
}

/// This is a custom implementation of the Display trait for the Editor enum. \
/// This is needed to be able to print the enum to the console and convert it to a string.
impl Display for Editor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Editor::Atom => write!(f, "atom"),
            Editor::Emacs => write!(f, "emacs"),
            Editor::Gedit => write!(f, "gedit"),
            Editor::Helix => write!(f, "helix"),
            Editor::Nano => write!(f, "nano"),
            Editor::NotePad => write!(f, "notepad"),
            Editor::Sublime => write!(f, "sublime"),
            Editor::Vim => write!(f, "vim"),
            Editor::VSCode => write!(f, "vscode"),
            Editor::Other => write!(f, "other"),
            Editor::None => write!(f, "none"),
        }
    }
}

impl Editor {

    /// This function returns the name of the executable of the respctive editor as a static string.\
    /// This is needed to be able to open the editor with the default command.
    /// 
    /// # Example
    /// ```
    /// use crate::config::editor::Editor;
    /// 
    /// let editor = Editor::Atom;
    /// assert_eq!(editor.as_command(), "atom");
    /// ```
    /// \
    /// .
    pub fn as_command(&self) -> &'static str {
        use Editor::*;
        match *self {
            Atom => "atom",
            Emacs => "emacs",
            Gedit => "gedit",
            Helix => "hx",
            Nano => "nano",
            NotePad => "notepad",
            Sublime => "sublime",
            Vim => "vim",
            VSCode => "code",
            Other => "other",

            _ => "none",
        }
    }
}

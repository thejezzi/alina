use clap::{Parser, ValueEnum};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, ValueEnum, Parser)]
pub enum Languages {
    Rust,
    Javascript
}

impl Display for Languages {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Languages::Rust => write!(f, "rust")?,
            Languages::Javascript => write!(f, "javascript")?,
        };
        Ok(())
    }
}
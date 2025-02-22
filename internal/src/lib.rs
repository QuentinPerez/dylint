pub mod cargo;

mod command;
pub use command::*;

pub mod env;

pub mod examples;

mod filename;
pub use filename::{library_filename, parse_path_filename};

#[cfg(feature = "git2")]
mod git;
#[cfg(feature = "git2")]
pub use git::*;

pub mod packaging;

pub mod paths;

pub mod rustup;

mod sed;
pub use sed::find_and_replace;

pub mod testing;

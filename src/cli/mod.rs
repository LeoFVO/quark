mod build;

use crate::cli::build::BuildCommand;
use clap::{Parser, Subcommand};

/// CLI related errors
#[derive(Debug)]
pub enum Error {
}

/// A common result type for our CLI.
pub type Result<T> = std::result::Result<T, Error>;

/// `Handler` is a trait that should be implemented for each of our commands.
///
/// It defines the contract & the input / output of a command execution.
pub trait Handler {
    /// Executes the command handler.
    ///
    /// Every command should take no argument, has it is built at runtime with these arguments.
    /// Also, a command must always return a `Result<()>`.
    fn handler(&self) -> crate::Result<()>;
}

#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct Cli {
    /// Container bundle
    #[clap(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    /// Get the command used by the user.
    ///
    /// For example, if the user executes the command `build`,
    /// we dynamically return the command so the `main` can
    /// execute it.
    pub fn command(self) -> Box<dyn Handler> {
        match self.command {
            Command::Build(cmd) => Box::new(cmd),
        }
    }
}

/// The enumeration of our commands.
///
/// Each of our commands should be listed in this enumeration with the following format :
/// CommandName(CommandHandler)
///
/// Example:
///
/// You want to add the `list` command:
///
/// List(ListCommand)
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Build a quardle
    Build(BuildCommand),
}
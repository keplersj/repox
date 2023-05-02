use clap::{CommandFactory, Parser};
use miette::{Diagnostic, Result};
use repox::command::{
    init::{self, run_init},
    sync::{self, run_sync},
    Command,
};
use thiserror::Error;

/// Work-in-Progress drop-in replacement for Google's gerrit repo tool
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Error, Diagnostic)]
enum CLIError {
    #[error("An error occurred while running the init command")]
    #[diagnostic(code(repox::main::init))]
    InitError(#[from] init::InitError),

    #[error("An error occurred while running the sync command")]
    #[diagnostic(code(repox::main::sync))]
    SyncError(#[from] sync::SyncError),

    #[error("The executed command has not been implemented: {0:#?}")]
    #[diagnostic(code(repox::main::command_unimplemented))]
    // Command Boxed at the advice of clippy
    UnimplementedCommand(Box<Command>),
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match args.command {
        Command::Init(args) => Ok(run_init(*args).map_err(CLIError::InitError)?),
        Command::Sync(args) => Ok(run_sync(args).map_err(CLIError::SyncError)?),
        Command::Version => {
            let version = Args::command().render_long_version();
            println!("{version}");

            Ok(())
        }
        command => Err(CLIError::UnimplementedCommand(Box::from(command)).into()),
    }
}

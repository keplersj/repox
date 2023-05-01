use clap::{CommandFactory, Parser};
use miette::{Diagnostic, Result};
use repox::command::{init::run_init, Command};
use thiserror::Error;

/// Work-in-Progress drop-in replacement for Google's gerrit repo tool
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(code(repox::main))]
enum CLIError {
    #[error("The executed command has not been implemented: {0:#?}")]
    UnimplementedCommand(Command),
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match args.command {
        Command::Init(args) => run_init(args),
        Command::Version => {
            let version = Args::command().render_long_version();
            println!("{version}");

            Ok(())
        }
        command => Err(CLIError::UnimplementedCommand(command).into()),
    }
}

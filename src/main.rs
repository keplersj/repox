use clap::Parser;
use miette::Result;
use repox::command::{init::run_init, Command};

/// Work-in-Progress drop-in replacement for Google's gerrit repo tool
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match args.command {
        Command::Init(args) => run_init(args),
        Command::Sync(_args) => todo!(),
        Command::Upload(_args) => todo!(),
        Command::Diff(_args) => todo!(),
        Command::Download(_args) => todo!(),
        Command::ForAll(_args) => todo!(),
        Command::Prune(_args) => todo!(),
        Command::Start(_args) => todo!(),
        Command::Status(_args) => todo!(),
        _ => todo!(),
    }
}

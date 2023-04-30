use clap::{CommandFactory, Parser};
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
        Command::Version => {
            let version = Args::command().render_long_version();
            println!("{}", version);

            Ok(())
        }
        _ => todo!(),
    }
}

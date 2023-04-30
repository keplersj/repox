use clap::Args;

#[derive(Args, Debug)]
pub struct DiffArgs {
    projects: Option<Vec<String>>,
}

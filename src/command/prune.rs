use clap::Args;

#[derive(Args, Debug)]
pub struct PruneArgs {
    projects: Option<Vec<String>>,
}

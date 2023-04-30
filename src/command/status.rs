use clap::Args;

#[derive(Args, Debug)]
pub struct StatusArgs {
    projects: Option<Vec<String>>,
}

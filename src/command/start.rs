use clap::Args;

#[derive(Args, Debug)]
pub struct StartArgs {
    branch_name: String,
    projects: Option<Vec<String>>,
}

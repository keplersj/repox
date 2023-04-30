use clap::Args;

#[derive(Args, Debug)]
pub struct ForAllArgs {
    projects: Option<Vec<String>>,
}

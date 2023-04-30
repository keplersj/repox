use clap::Args;

#[derive(Args, Debug)]

pub struct SyncArgs {
    projects: Option<Vec<String>>,
}

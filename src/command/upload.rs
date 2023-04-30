use clap::Args;

#[derive(Args, Debug)]
pub struct UploadArgs {
    projects: Option<Vec<String>>,
}

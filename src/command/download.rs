use clap::Args;

#[derive(Args, Debug)]
pub struct DownloadArgs {
    target: String,
    change: String,
}

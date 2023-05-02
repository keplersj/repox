use clap::Args;
use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Args, Debug)]

pub struct SyncArgs {
    projects: Option<Vec<String>>,
}

#[derive(Debug, Error, Diagnostic)]
pub enum SyncError {}

pub fn run_sync(args: SyncArgs) -> Result<(), SyncError> {
    Ok(())
}

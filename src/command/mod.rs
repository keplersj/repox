pub mod diff;
pub mod download;
pub mod for_all;
pub mod init;
pub mod prune;
pub mod start;
pub mod status;
pub mod sync;
pub mod upload;

use self::{
    diff::DiffArgs, download::DownloadArgs, for_all::ForAllArgs, init::InitArgs, prune::PruneArgs,
    start::StartArgs, status::StatusArgs, sync::SyncArgs, upload::UploadArgs,
};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize a repo client checkout in the current directory
    // Arguments boxed at the advice of clippy
    Init(Box<InitArgs>),

    /// Update working tree to the latest revision
    Sync(SyncArgs),

    /// Upload changes for code review
    Upload(UploadArgs),

    /// Show changes between commit and working tree
    Diff(DiffArgs),

    /// Download and checkout a change
    Download(DownloadArgs),

    /// Run a shell command in each project
    ForAll(ForAllArgs),

    /// Prune (delete) already merged topics
    Prune(PruneArgs),

    /// Start a new branch for development
    Start(StartArgs),

    /// Show the working tree status
    Status(StatusArgs),

    /// Permanently abandon a development branch
    Abandon,
    /// View current topic branches
    Branch,
    /// View current topic branches
    Branches,
    /// Checkout a branch for development
    Checkout,
    /// Cherry-pick a change.
    CherryPick,
    /// Manifest diff utility
    DiffManifests,
    /// Delete a GITC Client.
    GitcDelete,
    /// Initialize a GITC Client.
    GitcInit,
    /// Print lines matching a pattern
    Grep,
    /// Get info on the manifest branch, current branch or unmerged branches
    Info,
    /// List projects and their associated directories
    List,
    /// Manifest inspection utility
    Manifest,
    /// Display overview of unmerged project branches
    Overview,
    /// Update repo to the latest version
    SelfUpdate,
    /// Update working tree to the latest known good revision
    SmartSync,
    /// Stage file(s) for commit
    Stage,
    /// Display the version of repox
    Version,
}

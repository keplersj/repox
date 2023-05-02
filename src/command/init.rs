use crate::manifest::Manifest;
use clap::Args;
use miette::{Diagnostic, Result};
use quick_xml::{de::from_str, DeError};
use std::fs::read_to_string;
use thiserror::Error;
use tracing::{info, info_span};

/// Initialize a repo client checkout in the current directory
///
/// # Description
///
/// The 'repo init' command is run once to install and initialize repo. The latest
/// repo source code and manifest collection is downloaded from the server and is
/// installed in the .repo/ directory in the current working directory.
///
/// When creating a new checkout, the manifest URL is the only required setting. It
/// may be specified using the --manifest-url option, or as the first optional
/// argument.
///
/// The optional -b argument can be used to select the manifest branch to checkout
/// and use. If no branch is specified, the remote's default branch is used. This is
/// equivalent to using -b HEAD.
///
/// The optional -m argument can be used to specify an alternate manifest to be
/// used. If no manifest is specified, the manifest default.xml will be used.
///
/// If the --standalone-manifest argument is set, the manifest will be downloaded
/// directly from the specified --manifest-url as a static file (rather than setting
/// up a manifest git checkout). With --standalone-manifest, the manifest will be
/// fully static and will not be re-downloaded during subsesquent `repo init` and
/// `repo sync` calls.
///
/// The --reference option can be used to point to a directory that has the content
/// of a --mirror sync. This will make the working directory use as much data as
/// possible from the local reference directory when fetching from the server. This
/// will make the sync go a lot faster by reducing data traffic on the network.
///
/// The --dissociate option can be used to borrow the objects from the directory specified with the --reference option only to reduce network transfer, and stop
/// borrowing from them after a first clone is made by making necessary local copies
/// of borrowed objects.
///
/// The --no-clone-bundle option disables any attempt to use $URL/clone.bundle to
/// bootstrap a new Git repository from a resumeable bundle file on a content
/// delivery network. This may be necessary if there are problems with the local
/// Python HTTP client or proxy configuration, but the Git binary works.
///
/// # Switching Manifest Branches
///
/// To switch to another manifest branch, `repo init -b otherbranch` may be used in
/// an existing client. However, as this only updates the manifest, a subsequent
/// `repo sync` (or `repo sync -d`) is necessary to update the working directory
/// files.
#[derive(Args, Debug)]
pub struct InitArgs {
    //Logging options
    /// show all output
    #[arg(short = 'v', long, default_value_t = false)]
    verbose: bool,
    /// show all output
    #[arg(short = 'q', long, default_value_t = false)]
    quiet: bool,

    // Manifest options
    /// manifest repository location
    #[arg(short = 'u', long)]
    manifest_url: String,
    /// manifest branch or revision (use HEAD for default)
    #[arg(short = 'b', long, default_value = "HEAD")]
    manifest_branch: String,
    /// initial manifest file
    #[arg(short = 'm', long, default_value = "default.xml")]
    manifest_path: String,
    /// restrict manifest projects to ones with specified
    /// group(s) [default|all|G1,G2,G3|G4,-G5,-G6]
    #[arg(short = 'g', long)]
    groups: Option<Vec<String>>,
    /// restrict manifest projects to ones with a specified
    /// platform group [auto|all|none|linux|darwin|...]
    #[arg(short = 'p', long)]
    platform: Option<Vec<String>>,
    /// sync any submodules associated with the manifest repo
    #[arg(long, default_value_t = true)]
    submodules: bool,
    /// download the manifest as a static file rather then
    /// create a git checkout of the manifest repo
    #[arg(long, default_value_t = false)]
    standalone_manifest: bool,
    /// create a shallow clone of the manifest repo with given
    /// depth (0 for full clone); see git clone
    #[arg(long, default_value_t = 0)]
    manifest_depth: usize,

    // Manifest (only) checkout options
    /// fetch only current manifest branch from server (default)
    #[arg(short = 'c', long, default_value_t = true)]
    current_branch: bool,
    /// fetch all manifest branches from server
    #[arg(long)]
    no_current_branch: Option<bool>,
    /// fetch tags in the manifest
    #[arg(long)]
    tags: Option<bool>,
    /// don't fetch tags in the manifest
    #[arg(long)]
    no_tags: Option<bool>,
    // Checkout modes
    /// create a replica of the remote repositories rather than a client working directory
    #[arg(long)]
    mirror: Option<bool>,
    /// checkout an archive instead of a git repository for each project. See git archive.
    #[arg(long)]
    archive: Option<bool>,
    /// use git-worktree to manage projects
    #[arg(long)]
    worktree: Option<bool>,

    // Project checkout optimizations
    /// use git-worktree to manage projects
    #[arg(long)]
    reference: Option<String>,
    /// dissociate from reference mirrors after clone
    #[arg(long)]
    dissociate: Option<bool>,
    /// create a shallow clone with given depth; see git clone
    #[arg(long)]
    depth: Option<usize>,
    /// perform partial clone (https://git-scm.com/docs/gitrepository-layout#_code_partialclone_code)
    #[arg(long)]
    partial_clone: Option<bool>,
    /// disable use of partial clone (https://git-scm.com/docs/gitrepository-layout#_code_partialclone_code)
    #[arg(long)]
    no_partial_clone: Option<bool>,
    /// exclude the specified projects (a comma-delimited project names) from partial clone (https://git-scm.com/docs/gitrepository-layout#_code_partialclone_code)
    #[arg(long)]
    partial_clone_exclude: Option<String>,
    /// filter for use with --partial-clone [default:blob:none]
    #[arg(long)]
    clone_filter: Option<String>,
    /// use the manifest superproject to sync projects; implies -c
    #[arg(long)]
    use_superproject: Option<bool>,
    /// disable use of manifest superprojects
    #[arg(long)]
    no_use_superproject: Option<bool>,
    /// enable use of /clone.bundle on HTTP/HTTPS (default if not --partial-clone)
    #[arg(long)]
    clone_bundle: Option<bool>,
    /// disable use of /clone.bundle on HTTP/HTTPS (default if --partial-clone)
    #[arg(long)]
    no_clone_bundle: Option<bool>,
    /// enable Git LFS support
    #[arg(long)]
    git_lfs: Option<bool>,
    /// disable Git LFS support
    #[arg(long)]
    no_git_lfs: Option<bool>,

    // repo Version options
    /// repo repository location ($REPO_URL)
    #[arg(long)]
    repo_url: Option<String>,
    /// repo branch or revision ($REPO_REV)
    #[arg(long)]
    repo_rev: Option<String>,
    /// do not verify repo source code
    #[arg(long)]
    no_repo_verify: Option<bool>,

    // Other options
    /// Always prompt for name/e-mail
    #[arg(long)]
    config_name: Option<bool>,

    // Multi-manifest:
    /// operate starting at the outermost manifest
    #[arg(long)]
    outer_manifest: Option<bool>,
    /// do not operate on outer manifests
    #[arg(long)]
    no_outer_manifest: Option<bool>,
    /// only operate on this (sub)manifest
    #[arg(long)]
    this_manifest_only: Option<bool>,
    /// don't operate on this manifest and its submanifests
    #[arg(long)]
    no_this_manifest_only: Option<bool>,
    /// operate on this manifest and its submanifests
    #[arg(long)]
    all_manifests: Option<bool>,
}

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(code(repox::command::init))]
pub enum InitError {
    #[error("Could not read manifest file")]
    ManifestReadError(#[source] std::io::Error),

    #[error("An error occurred initializing gix's interrupt handler")]
    GixInterruptInitError(#[source] std::io::Error),

    #[error("An error occurred while creating a destination directory")]
    CreateDirectoryError(#[source] std::io::Error),

    #[error(transparent)]
    XmlDeserializationError(#[from] DeError),

    #[error(transparent)]
    GixUrlParseError(#[from] gix::url::parse::Error),

    #[error(transparent)]
    GixCloneError(#[from] gix::clone::Error),

    #[error(transparent)]
    GixFetchError(#[from] gix::clone::fetch::Error),

    #[error(transparent)]
    GixCheckoutError(#[from] gix::clone::checkout::main_worktree::Error),

    #[error(transparent)]
    GixRemoteError(#[from] gix::remote::find::existing::Error),
}

pub fn run_init(args: InitArgs) -> Result<(), InitError> {
    let manifest_contents =
        read_to_string(args.manifest_path).map_err(InitError::ManifestReadError)?;

    let manifest: Manifest = from_str(&manifest_contents)?;

    gix::interrupt::init_handler(|| {}).map_err(InitError::GixInterruptInitError)?;

    for project in manifest.projects() {
        let _project_span = info_span!("Checking out project", name = project.name).entered();

        let remote = manifest
            .remotes()
            .into_iter()
            .find(|remote| remote.name == project.remote.clone().unwrap())
            .unwrap();

        info!("Project remote {:#?}", remote);

        let repo_url = format!("{}/{}", remote.fetch, project.name);
        info!("Repo URL: {repo_url}");
        let dst = project.path.unwrap();
        info!("Destination: {dst}");

        std::fs::create_dir_all(&dst).map_err(InitError::CreateDirectoryError)?;
        info!("Destination Created: {dst}");
        let url = gix::url::parse(repo_url.as_str().into())?;
        info!("Git URL: {:#?}", url);

        info!("Url: {:?}", url.to_bstring());
        let mut prepare_clone = gix::prepare_clone(url, &dst)?;

        let clone_span = info_span!("Cloning {repo_url:?} into {dst:?}...").entered();
        let (mut prepare_checkout, _) = prepare_clone
            .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;
        clone_span.exit();

        let checkout_span = info_span!(
            "Checking out project",
            dest = ?prepare_checkout.repo().work_dir().expect("should be there")
        )
        .entered();

        let (repo, _) = prepare_checkout
            .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;

        checkout_span.exit();

        let remote = repo
            .find_default_remote(gix::remote::Direction::Fetch)
            .expect("always present after clone")?;

        info!(
            "Default remote: {} -> {}",
            remote
                .name()
                .expect("default remote is always named")
                .as_bstr(),
            remote
                .url(gix::remote::Direction::Fetch)
                .expect("should be the remote URL")
                .to_bstring(),
        );
    }

    Ok(())
}

use clap::Parser;
use miette::{IntoDiagnostic, Result};
use quick_xml::de::from_str;
use repox::manifest::Manifest;
use std::{fs::read_to_string, path::PathBuf};
use tracing::{info, info_span};

/// Work-in-Progress drop-in replacement for Google's gerrit repo tool
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    manifest_path: PathBuf,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let manifest_contents = read_to_string(args.manifest_path).into_diagnostic()?;

    let manifest: Manifest = from_str(&manifest_contents).into_diagnostic()?;

    gix::interrupt::init_handler(|| {}).into_diagnostic()?;

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

        std::fs::create_dir_all(&dst).into_diagnostic()?;
        info!("Destination Created: {dst}");
        let url = gix::url::parse(repo_url.as_str().into()).into_diagnostic()?;
        info!("Git URL: {:#?}", url);

        info!("Url: {:?}", url.to_bstring());
        let mut prepare_clone = gix::prepare_clone(url, &dst).into_diagnostic()?;

        let clone_span = info_span!("Cloning {repo_url:?} into {dst:?}...").entered();
        let (mut prepare_checkout, _) = prepare_clone
            .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
            .into_diagnostic()?;
        clone_span.exit();

        let checkout_span = info_span!(
            "Checking out project",
            dest = ?prepare_checkout.repo().work_dir().expect("should be there")
        )
        .entered();

        let (repo, _) = prepare_checkout
            .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
            .into_diagnostic()?;

        checkout_span.exit();

        let remote = repo
            .find_default_remote(gix::remote::Direction::Fetch)
            .expect("always present after clone")
            .into_diagnostic()?;

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

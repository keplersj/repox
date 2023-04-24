use clap::Parser;
use miette::{Context, IntoDiagnostic, Result};
use quick_xml::de::from_str;
use repox::manifest::Manifest;
use std::{fs::read_to_string, path::PathBuf};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// Work-in-Progress drop-in replacement for Google's gerrit repo tool
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    manifest_path: PathBuf,
}

fn main() -> Result<()> {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .into_diagnostic()
        .context("setting default subscriber failed")?;

    let args = Args::parse();

    let manifest_contents = read_to_string(args.manifest_path).into_diagnostic()?;

    let manifest: Manifest = from_str(&manifest_contents).into_diagnostic()?;

    gix::interrupt::init_handler(|| {}).into_diagnostic()?;

    for project in manifest.projects() {
        println!("Project: {:#?}", project);

        let remote = manifest
            .remotes()
            .into_iter()
            .find(|remote| remote.name == project.remote.clone().unwrap())
            .unwrap();

        println!("Remote: {:#?}", remote);

        let repo_url = format!("{}/{}", remote.fetch, project.name);
        println!("Repo URL: {repo_url}");
        let dst = project.path.unwrap();
        println!("Destination: {dst}");

        std::fs::create_dir_all(&dst).into_diagnostic()?;
        println!("Destination Created: {dst}");
        let url = gix::url::parse(repo_url.as_str().into()).into_diagnostic()?;
        println!("Git URL: {:#?}", url);

        println!("Url: {:?}", url.to_bstring());
        let mut prepare_clone = gix::prepare_clone(url, &dst).into_diagnostic()?;

        println!("Cloning {repo_url:?} into {dst:?}...");
        let (mut prepare_checkout, _) = prepare_clone
            .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
            .into_diagnostic()?;

        println!(
            "Checking out into {:?} ...",
            prepare_checkout.repo().work_dir().expect("should be there")
        );

        let (repo, _) = prepare_checkout
            .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
            .into_diagnostic()?;
        println!(
            "Repo cloned into {:?}",
            repo.work_dir().expect("directory pre-created")
        );

        let remote = repo
            .find_default_remote(gix::remote::Direction::Fetch)
            .expect("always present after clone")
            .into_diagnostic()?;

        println!(
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

        println!("{}\n\n", "==========");
    }

    Ok(())
}

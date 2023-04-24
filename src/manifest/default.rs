use serde::Deserialize;

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-default)
#[derive(Debug, Clone, Deserialize)]
pub(super) struct Default {
    /// Name of a previously defined remote element.
    /// Project elements lacking a remote attribute of their own will use this remote.
    #[serde(rename = "@remote")]
    remote: Option<String>,

    /// Name of a Git branch (e.g. master or refs/heads/master).
    /// Project elements lacking their own revision attribute will use this revision.
    #[serde(rename = "@revision")]
    revision: Option<String>,

    /// Name of a Git branch (e.g. master).
    /// Project elements not setting their own dest-branch will inherit this value.
    /// If this value is not set, projects will use revision by default instead.
    #[serde(rename = "@dest-branch")]
    dest_branch: Option<String>,

    /// Name of the Git ref in which a sha1 can be found.
    /// Used when syncing a revision locked manifest in -c mode to avoid having to sync the entire ref space.
    /// Project elements not setting their own upstream will inherit this value.
    #[serde(rename = "@upstream")]
    upstream: Option<String>,

    /// Number of parallel jobs to use when synching.
    #[serde(rename = "@sync-j")]
    sync_j: Option<String>,

    /// Set to true to only sync the given Git branch (specified in the revision attribute) rather than the whole ref space.
    /// Project elements lacking a sync-c element of their own will use this value.
    #[serde(rename = "@sync-c")]
    sync_c: Option<String>,

    /// Set to true to also sync sub-projects.
    #[serde(rename = "@sync-s")]
    sync_s: Option<String>,

    /// Set to false to only sync the given Git branch (specified in the revision attribute) rather than the other ref tags.
    #[serde(rename = "@sync-tags")]
    sync_tags: Option<String>,
}

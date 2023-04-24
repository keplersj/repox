use serde::Deserialize;

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-remote)
#[derive(Debug, Clone, Deserialize)]
pub struct Remote {
    /// A short name unique to this manifest file.
    /// The name specified here is used as the remote name in each project's .git/config,
    ///     and is therefore automatically available to commands like git fetch, git remote, git pull and git push.
    #[serde(rename = "@name")]
    pub name: String,

    /// The alias, if specified, is used to override name to be set as the remote name in each project's .git/config.
    /// Its value can be duplicated while attribute name has to be unique in the manifest file.
    /// This helps each project to be able to have same remote name which actually points to different remote url.
    #[serde(rename = "@alias")]
    alias: Option<String>,

    /// The Git URL prefix for all projects which use this remote.
    /// Each project's name is appended to this prefix to form the actual URL used to clone the project.
    #[serde(rename = "@fetch")]
    pub fetch: String,

    /// The Git “push” URL prefix for all projects which use this remote.
    /// Each project's name is appended to this prefix to form the actual URL used to “git push” the project.
    /// This attribute is optional; if not specified then “git push” will use the same URL as the fetch attribute.
    #[serde(rename = "@pushurl")]
    pushurl: Option<String>,

    /// Hostname of the Gerrit server where reviews are uploaded to by repo upload.
    /// This attribute is optional; if not specified then repo upload will not function.
    #[serde(rename = "@review")]
    review: Option<String>,

    /// Name of a Git branch (e.g. master or refs/heads/master).
    /// Remotes with their own revision will override the default revision.    
    #[serde(rename = "@revision")]
    revision: Option<String>,
}

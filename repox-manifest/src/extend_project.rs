use serde::Deserialize;

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-extend_project)
#[derive(Debug, Clone, Deserialize)]
pub(super) struct ExtendProject {
    #[serde(rename = "@name")]
    name: String,

    /// If specified, limit the change to projects checked out at the specified path, rather than all projects with the given name.
    #[serde(rename = "@path")]
    path: Option<String>,

    /// List of additional groups to which this project belongs.
    /// Same syntax as the corresponding element of project.
    #[serde(rename = "@groups")]
    groups: Option<String>,

    /// If specified, overrides the revision of the original project.
    /// Same syntax as the corresponding element of project.
    #[serde(rename = "@revision")]
    revision: Option<String>,

    /// If specified, overrides the remote of the original project.
    /// Same syntax as the corresponding element of project.
    #[serde(rename = "@remote")]
    remote: Option<String>,
}

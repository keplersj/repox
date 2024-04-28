use serde::Deserialize;

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-remove_project)
#[derive(Debug, Clone, Deserialize)]
pub(super) struct RemoveProject {
    #[serde(rename = "@name")]
    name: String,
}

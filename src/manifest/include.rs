use serde::Deserialize;

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-include)
#[derive(Debug, Clone, Deserialize)]
pub(super) struct Include {
    /// the manifest to include, specified relative to the manifest repository's root.
    #[serde(rename = "@name")]
    name: String,
}

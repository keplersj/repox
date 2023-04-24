use serde::Deserialize;

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-manifest_server)
#[derive(Debug, Clone, Deserialize)]
pub(super) struct ManifestServer {
    #[serde(rename = "@url")]
    url: String,
}

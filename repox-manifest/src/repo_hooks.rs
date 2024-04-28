use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RepoHooks {
    #[serde(rename = "@in-project")]
    in_project: String,
    #[serde(rename = "@enabled-list")]
    enabled_list: String,
}

pub mod default;
pub mod extend_project;
pub mod include;
pub mod manifest_server;
pub mod notice;
pub mod project;
pub mod remote;
pub mod remove_project;
pub mod repo_hooks;

use self::{
    extend_project::ExtendProject, include::Include, manifest_server::ManifestServer,
    notice::Notice, project::Project, remote::Remote, remove_project::RemoveProject,
    repo_hooks::RepoHooks,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-manifest) for more.
pub struct Manifest {
    notice: Option<Notice>,

    /// One or more remote elements may be specified.
    /// Each remote element specifies a Git URL shared by one or more projects and (optionally) the Gerrit review server those projects upload changes through.
    remote: Option<Vec<Remote>>,

    /// At most one default element may be specified.
    /// Its remote and revision attributes are used when a project element does not specify its own remote or revision attribute.
    default: Option<self::default::Default>,

    /// At most one manifest-server may be specified.
    /// The url attribute is used to specify the URL of a manifest server, which is an XML RPC service.
    ///
    /// The manifest server should implement the following RPC methods:
    ///
    /// ```sh
    /// GetApprovedManifest(branch, target)
    /// ```
    ///
    /// Return a manifest in which each project is pegged to a known good revision for the current branch and target.
    /// This is used by repo sync when the --smart-sync option is given.
    ///
    /// The target to use is defined by environment variables TARGET_PRODUCT and TARGET_BUILD_VARIANT.
    /// These variables are used to create a string of the form $TARGET_PRODUCT-$TARGET_BUILD_VARIANT, e.g. passion-userdebug.
    /// If one of those variables or both are not present, the program will call GetApprovedManifest without the target parameter and the manifest server should choose a reasonable default target.
    ///
    /// ```sh
    /// GetManifest(tag)
    /// ```
    ///
    /// Return a manifest in which each project is pegged to the revision at the specified tag.
    /// This is used by repo sync when the --smart-tag option is given.
    #[serde(rename = "manifest-server")]
    manifest_server: Option<ManifestServer>,

    /// Deletes the named project from the internal manifest table, possibly allowing a subsequent project element in the same manifest file to replace the project with a different source.
    ///
    /// This element is mostly useful in a local manifest file, where the user can remove a project, and possibly replace it with their own definition.
    #[serde(rename = "remove-project")]
    remove_project: Option<Vec<RemoveProject>>,

    /// One or more project elements may be specified.
    /// Each element describes a single Git repository to be cloned into the repo client workspace.
    /// You may specify Git-submodules by creating a nested project.
    /// Git-submodules will be automatically recognized and inherit their parent's attributes, but those may be overridden by an explicitly specified project element.
    project: Option<Vec<Project>>,

    /// Modify the attributes of the named project.
    ///
    /// This element is mostly useful in a local manifest file, to modify the attributes of an existing project without completely replacing the existing project definition.
    /// This makes the local manifest more robust against changes to the original manifest.
    #[serde(rename = "extend-project")]
    extend_project: Option<Vec<ExtendProject>>,

    #[serde(rename = "repo-hooks")]
    repo_hooks: Option<RepoHooks>,

    /// This element provides the capability of including another manifest file into the originating manifest.
    /// Normal rules apply for the target manifest to include - it must be a usable manifest on its own.
    include: Option<Vec<Include>>,
}

impl Manifest {
    pub fn projects(&self) -> Vec<Project> {
        self.project.clone().unwrap_or_default()
    }

    pub fn remotes(&self) -> Vec<Remote> {
        self.remote.clone().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::Manifest;
    use insta::assert_debug_snapshot;
    use quick_xml::de::from_str;

    #[test]
    fn test_serialized_sample() {
        let manifest_contents = include_str!("../../samples/imx-6.1.1-1.0.0.xml");

        let parsed: Manifest = from_str(manifest_contents).unwrap();

        assert_debug_snapshot!(parsed);
    }
}

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Notice {}

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

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-default)
#[derive(Debug, Clone, Deserialize)]
struct Default {
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

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-manifest_server)
#[derive(Debug, Clone, Deserialize)]
struct ManifestServer {
    #[serde(rename = "@url")]
    url: String,
}

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-project)
#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    /// Zero or more annotation elements may be specified as children of a project element.
    /// Each element describes a name-value pair that will be exported into each project's environment during a ‘forall’ command, prefixed with REPO__.
    /// In addition, there is an optional attribute “keep” which accepts the case insensitive values “true” (default) or “false”.
    /// This attribute determines whether or not the annotation will be kept when exported with the manifest subcommand.
    annotation: Option<Vec<Annotation>>,

    project: Option<Vec<Project>>,

    /// Zero or more copyfile elements may be specified as children of a project element.
    /// Each element describes a src-dest pair of files; the “src” file will be copied to the “dest” place during repo sync command.
    ///
    /// “src” is project relative, “dest” is relative to the top of the tree.
    /// Copying from paths outside of the project or to paths outside of the repo client is not allowed.
    ///
    /// “src” and “dest” must be files.
    /// Directories or symlinks are not allowed.
    /// Intermediate paths must not be symlinks either.
    ///
    /// Parent directories of “dest” will be automatically created if missing.
    copyfile: Option<Vec<Copyfile>>,

    /// It's just like copyfile and runs at the same time as copyfile but instead of copying it creates a symlink.
    ///
    /// The symlink is created at “dest” (relative to the top of the tree) and points to the path specified by “src” which is a path in the project.
    ///
    /// Parent directories of “dest” will be automatically created if missing.
    ///
    /// The symlink target may be a file or directory, but it may not point outside of the repo client.
    linkfile: Option<Vec<LinkFile>>,

    /// A unique name for this project.
    /// The project‘s name is appended onto its remote’s fetch URL to generate the actual URL to configure the Git remote with.
    ///
    /// The URL gets formed as:
    ///
    /// ```sh
    /// ${remote_fetch}/${project_name}.git
    /// ```
    ///
    /// where ${remote_fetch} is the remote‘s fetch attribute and ${project_name} is the project’s name attribute.
    /// The suffix “.git” is always appended as repo assumes the upstream is a forest of bare Git repositories.
    /// If the project has a parent element, its name will be prefixed by the parent's.
    ///
    /// The project name must match the name Gerrit knows, if Gerrit is being used for code reviews.
    #[serde(rename = "@name")]
    pub name: String,

    /// An optional path relative to the top directory of the repo client where the Git working directory for this project should be placed.
    /// If not supplied the project name is used.
    /// If the project has a parent element, its path will be prefixed by the parent's.
    #[serde(rename = "@path")]
    pub path: Option<String>,

    /// Name of a previously defined remote element.
    /// If not supplied the remote given by the default element is used.
    #[serde(rename = "@remote")]
    pub remote: Option<String>,

    /// Name of the Git branch the manifest wants to track for this project.
    /// Names can be relative to refs/heads (e.g. just “master”) or absolute (e.g. “refs/heads/master”).
    /// Tags and/or explicit SHA-1s should work in theory, but have not been extensively tested.
    /// If not supplied the revision given by the remote element is used if applicable, else the default element is used.
    #[serde(rename = "@revision")]
    pub revision: Option<String>,

    /// Name of a Git branch (e.g. master).
    /// When using repo upload, changes will be submitted for code review on this branch.
    /// If unspecified both here and in the default element, revision is used instead.
    #[serde(rename = "@dest-branch")]
    pub dest_branch: Option<String>,

    /// List of groups to which this project belongs, whitespace or comma separated.
    /// All projects belong to the group “all”, and each project automatically belongs to a group of its name:name and path:path.
    /// E.g. for , that project definition is implicitly in the following manifest groups: default, name:monkeys, and path:barrel-of.
    /// If you place a project in the group “notdefault”, it will not be automatically downloaded by repo.
    /// If the project has a parent element, the name and path here are the prefixed ones.
    #[serde(rename = "@groups")]
    pub groups: Option<String>,

    /// Set to true to only sync the given Git branch (specified in the revision attribute) rather than the whole ref space.
    #[serde(rename = "@sync-c")]
    sync_c: Option<String>,

    /// Set to true to also sync sub-projects.
    #[serde(rename = "@sync-s")]
    sync_s: Option<String>,

    #[serde(rename = "@sync-tags")]
    sync_tags: Option<String>,

    /// Name of the Git ref in which a sha1 can be found.
    /// Used when syncing a revision locked manifest in -c mode to avoid having to sync the entire ref space.
    #[serde(rename = "@upstream")]
    upstream: Option<String>,

    /// Set the depth to use when fetching this project.
    /// If specified, this value will override any value given to repo init with the --depth option on the command line.
    #[serde(rename = "@clone-depth")]
    clone_depth: Option<String>,

    /// Set to true to force this project to create the local mirror repository according to its path attribute (if supplied) rather than the name attribute.
    /// This attribute only applies to the local mirrors syncing, it will be ignored when syncing the projects in a client working directory.
    #[serde(rename = "@force-path")]
    force_path: Option<String>,
}

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-extend_project)
#[derive(Debug, Clone, Deserialize)]
struct ExtendProject {
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

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-annotation)
#[derive(Debug, Clone, Deserialize)]
struct Annotation {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@keep")]
    keep: String,
}

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-copyfile)
#[derive(Debug, Clone, Deserialize)]
struct Copyfile {
    #[serde(rename = "@src")]
    src: String,
    #[serde(rename = "@dest")]
    dest: String,
}

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-linkfile)
#[derive(Debug, Clone, Deserialize)]
struct LinkFile {
    #[serde(rename = "@src")]
    src: String,
    #[serde(rename = "@dest")]
    dest: String,
}

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-remove_project)
#[derive(Debug, Clone, Deserialize)]
struct RemoveProject {
    #[serde(rename = "@name")]
    name: String,
}

/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-include)
#[derive(Debug, Clone, Deserialize)]
struct Include {
    /// the manifest to include, specified relative to the manifest repository's root.
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RepoHooks {
    #[serde(rename = "@in-project")]
    in_project: String,
    #[serde(rename = "@enabled-list")]
    enabled_list: String,
}

#[derive(Debug, Clone, Deserialize)]
/// See [Google's documentation](https://gerrit.googlesource.com/git-repo/+/master/docs/manifest-format.md#Element-manifest) for more.
pub struct Manifest {
    notice: Option<Notice>,

    /// One or more remote elements may be specified.
    /// Each remote element specifies a Git URL shared by one or more projects and (optionally) the Gerrit review server those projects upload changes through.
    remote: Option<Vec<Remote>>,

    /// At most one default element may be specified.
    /// Its remote and revision attributes are used when a project element does not specify its own remote or revision attribute.
    default: Option<Default>,

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
    use crate::manifest::Manifest;
    use insta::assert_debug_snapshot;
    use quick_xml::de::from_str;

    #[test]
    fn test_serialized_sample() {
        let manifest_contents = include_str!("../../samples/imx-6.1.1-1.0.0.xml");

        let parsed: Manifest = from_str(manifest_contents).unwrap();

        assert_debug_snapshot!(parsed);
    }
}

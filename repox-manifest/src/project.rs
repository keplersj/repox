use serde::Deserialize;

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

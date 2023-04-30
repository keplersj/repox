# repox

A (work in progress) drop-in replacement for [Google's gerrit repo management tool](https://gerrit.googlesource.com/git-repo), powered by [gitoxide](https://github.com/Byron/gitoxide).

## Mission

The goal of this project is to create a tool that serves as a drop in replacement for `repo`, written in Rust with an emphasis on performance, safety, and a solid user experience.

## Status

While this project is an active work in progres, the following has been planned/implemented:

Repo Management Backend:

- [ ] Manifest Parser
  - [x] Basic Functionality Implemented using [quick-xml](https://github.com/tafia/quick-xml) and [serde](https://serde.rs/)
  - [ ] Manifest includes
- [ ] Git Management

CLI Parity:

- [x] [Full parity](https://source.android.com/docs/setup/create/repo#help) with the `repo help` command
- [ ] [Full parity](https://source.android.com/docs/setup/create/repo#init) with the`repo init` command
  - [x] Partial functionality currently implemented (Reads project and remotes directly from a provided manifest on disc)
- [ ] [Full parity](https://source.android.com/docs/setup/create/repo#sync) with the `repo sync` command
  - [x] Partial functionality currently implemented in `run_init()` (Checks out projects from remotes using a provided manifest on disc)
- [ ] [Full parity](https://source.android.com/docs/setup/create/repo#upload) with the `repo upload` command
- [ ] [Full parity](https://source.android.com/docs/setup/create/repo#diff) with the `repo diff` command
- [ ] [Full parity](https://source.android.com/docs/setup/create/repo#download) with the `repo download` command
- [ ] [Full parity](https://source.android.com/docs/setup/create/repo#forall) with the `repo forall` command
- [ ] [Full parity](https://source.android.com/docs/setup/create/repo#prune) with the `repo prune` commmand
- [ ] [Full parity](https://source.android.com/docs/setup/create/repo#start) with the `repo start` command
- [ ] [Full parity](https://source.android.com/docs/setup/create/repo#status) with the `repo status` command
- [ ] Full parity with the `repo version` command
  - [x] Prints out version information prepared by clap.

## License

Copyright 2023 [Kepler Sticka-Jones](https://keplersj.com). All Rights Reserved.

workspace(name = "turbofish.rs")

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

git_repository(
    name = "io_bazel_rules_rust",
    # commit = "f32695dcd02d9a19e42b9eb7f29a24a8ceb2b858",
    branch = "master",
    remote = "https://github.com/bazelbuild/rules_rust.git",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

# https://bazelbuild.github.io/rules_rust/
# rust_repositories(version = "1.49.0", edition="2018", rustfmt_version = "1.49.0")
rust_repositories(version = "nightly", iso_date = "2021-01-11", edition="2018")

# https://docs.rs/crate/cargo-raze/0.0.19
# load("//cargo:crates.bzl", "raze_fetch_remote_crates")

# raze_fetch_remote_crates()

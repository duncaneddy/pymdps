# Contributing

## Release Process

The release process for `pymdps` is as follows:

1. Update the version number in `Cargo.toml` and `pyproject.toml`.
1. Update the changelog in `CHANGELOG.md`.
1. Commit the changes.
1. Copy the contents of `CHANGELOG.md` to the release notes on GitHub.
1. Create a new release on GitHub with the appropriate version number and release notes.
    1. Create a new tag with the version number. **DO NOT** prefix the tag with `v`. (e.g. use `1.0.0` instead of `v1.0.0`)
    1. Name the release `vX.Y.Z` where `X.Y.Z` is the version number.
    1. Copy the contents of `CHANGELOG.md` to the release notes.
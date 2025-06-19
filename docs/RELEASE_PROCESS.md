# Release Process

This project tags releases via Git and GitHub Actions. Follow these steps when preparing a new version.

1. **Update Version Numbers**
   - Edit `Cargo.toml` and set the new `version`.
   - Update the version in `docs/cgip.1` and `scripts/build_deb.sh`.
   - Run `cargo build --release` to update `Cargo.lock`.
2. **Write Release Notes**
   - Add a markdown file under `notes/` named after the version (e.g. `notes/0.5.0.md`).
   - Summarize new features, fixes and maintenance items.
3. **Commit Changes**
   - Commit all version bumps and notes.
4. **Tag the Release**
   - Run `make tag-release-dry` to verify the tag.
   - Run `make tag-release` to create and push the git tag. This triggers GitHub Actions to build and upload assets.
5. **Publish Packages (optional)**
   - `make tarball-publish` or `make deb-publish` will build artifacts and upload them to the GitHub release.

These steps ensure the version is consistent and the release automation runs correctly.

# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_Nothing yet._

## [1.0.1] - 2026-11-11

### Added

- Add `--version` to the CLI.
- Add `CHANGELOG.md`, with release notes extracted by `scripts/release.py` for each GitHub Release.
- Add `scripts/doctor.py`, `scripts/release.py`, `formula.rb.example`, `scoop.json.example` and `.github/dependabot.yml` from the Liferay Rust tool template (v0.7.0).
- Document the release process in a Releasing section of `README.md`.

### Changed

- Align with the Liferay Rust tool template v0.7.0 release infrastructure.
- Reject release tags that aren't annotated or don't match the `Cargo.toml` version in `.github/workflows/release.yml`, derive binary and asset names from `Cargo.toml`, and package release assets as `.tar.gz` / `.zip` archives.
- Pin GitHub Actions to commit SHAs.
- Run the template's placeholder check (`scripts/doctor.py`) in the pre-commit hook.
- Update `.gemini/prompts/update-distribution-channels.md` to name the formula and manifest after the binary, keep the example templates, and only publish on a tool's first release (the tap and bucket auto-update after that).
- Restructure `README.md` to the template standard and correct the documented restart command and post-reset behaviour.

### Removed

- Remove the unused `src/core` and `src/utils` template modules, which were never compiled into the binary.

### Security

- Bump `openssl` to 0.10.82 and `openssl-sys` to 0.9.118.
- Bump `rand` to 0.8.8.

## [1.0.0] - 2026-03-16

### Added

- Add the `reset` command, which drops and recreates the `public` schema of a Liferay Cloud environment's database via `lcp shell`.
- Discover the running instance automatically from `lcp shell` output, and accept a project ID with or without an environment suffix.
- Automate the shell session with `expect` when available, falling back to printing the command for manual use.
- Prompt for the database password and require confirmation before resetting, unless `--force` is passed.
- Restart the service after an automated reset.
- Add release builds for Linux, macOS (Intel and Apple Silicon) and Windows, and document Homebrew, Scoop, WSL and source installation.

<!-- markdownlint-disable MD049 -->

---

_Last Updated: 2026-11-11_ | _Last Reviewed: 2026-11-11_

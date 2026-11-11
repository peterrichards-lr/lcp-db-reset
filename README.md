# Liferay Cloud Database Reset Tool (`lcp-db-reset`)

[![GitHub Release](https://img.shields.io/github/v/release/peterrichards-lr/lcp-db-reset)](https://github.com/peterrichards-lr/lcp-db-reset/releases)
[![License](https://img.shields.io/github/license/peterrichards-lr/lcp-db-reset)](LICENSE)
[![Rust CI](https://github.com/peterrichards-lr/lcp-db-reset/actions/workflows/rust.yml/badge.svg)](https://github.com/peterrichards-lr/lcp-db-reset/actions/workflows/rust.yml)

A cross-platform Rust CLI that resets the database schema of a Liferay Cloud (LCP) environment. It finds the running instance for you, then runs `DROP SCHEMA public CASCADE; CREATE SCHEMA public;` with `psql` inside an `lcp shell` session, so Liferay can rebuild its database from scratch on the next start.

## Features

- **Safe:** Fails fast if you aren't authenticated with `lcp`, and asks for explicit confirmation before wiping the database (unless `--force` is passed).
- **Smart:** Discovers the running instance of the service automatically and detects whether your project ID already includes an environment suffix.
- **Secure:** Prompts for the database password with hidden input instead of leaving it in your shell history (unless you pass it with `--password`).
- **Automated:** When `expect` is installed, it "types" the command into the interactive `lcp shell` and restarts the service afterwards, with no manual steps on macOS, Linux and WSL.
- **Cross-Platform:** Native binaries for macOS (Intel and Apple Silicon), Linux and Windows.

## Prerequisites

- The Liferay Cloud CLI (`lcp`), logged in with `lcp login`.
- `expect` for full automation (installed by default on most macOS and Linux systems). Without it, the tool opens `lcp shell` and prints the command for you to paste.

## Installation

### Homebrew (macOS / Linux)

```bash
brew tap peterrichards-lr/tap
brew install lcp-db-reset
```

### Scoop (Windows)

```powershell
scoop bucket add peterrichards-lr https://github.com/peterrichards-lr/scoop-bucket
scoop install lcp-db-reset
```

### Windows Subsystem for Linux (WSL)

Install [Homebrew on Linux](https://docs.brew.sh/Homebrew-on-Linux) inside your WSL distribution, then install the tool with Homebrew:

```bash
brew tap peterrichards-lr/tap
brew install lcp-db-reset
```

For full automation, make sure `expect` is installed in your WSL distribution:

```bash
# Ubuntu/Debian example
sudo apt update && sudo apt install expect -y
```

### Manual Download

Download the archive for your platform from the [GitHub Releases](https://github.com/peterrichards-lr/lcp-db-reset/releases) page, extract it, and put `lcp-db-reset` on your `PATH`.

### Build from Source

With a Rust toolchain installed:

```bash
cargo install --path .
```

## Usage

```bash
lcp-db-reset --help
lcp-db-reset reset --help
lcp-db-reset --version
```

### Common Patterns

The project ID can be given in several forms:

```bash
# Option 1: Project and environment separately
lcp-db-reset reset -p my-project -e dev

# Option 2: Full LCP project ID
lcp-db-reset reset -p my-project-dev

# Option 3: Project only, no hyphen (you will be prompted for the environment)
lcp-db-reset reset -p myproject
```

Without `--environment`, a project ID that contains a hyphen is treated as a full `project-environment` ID, so the environment prompt only appears for project IDs without a hyphen.

### `reset` Options

- `-p, --project <PROJECT>`: The project ID (e.g. `lfrprj`) or full ID (e.g. `lfrprj-dev`). Required.
- `-e, --environment <ENVIRONMENT>`: The environment (e.g. `dev`, `uat`, `prd`).
- `-s, --service <SERVICE>`: The service name [default: `liferay`].
- `-i, --instance <INSTANCE>`: A specific instance ID (optional, found automatically if missing).
- `-P, --password <PASSWORD>`: The database password (optional, prompted securely if missing).
- `--force`: Skip the confirmation prompt.

### What It Does

1. Checks that you're logged in (`lcp auth token`).
2. Finds the running instance of the service from the `lcp shell` instance list, unless `--instance` is given.
3. Prompts for the database password (unless `--password` is given) and asks for confirmation (unless `--force` is given).
4. With `expect` installed, runs the `psql` reset inside `lcp shell`, then runs `lcp restart` for the service.
   Without `expect`, prints the `psql` command and opens `lcp shell` for you to paste it; the restart is then up to you.

## Releasing

1. Bump `version` in `Cargo.toml` and run `cargo build` to update `Cargo.lock`.
2. Move the `[Unreleased]` entries in `CHANGELOG.md` into a new `## [x.y.z] - YYYY-MM-DD` section.
3. Commit, then create an annotated tag and push it:
   ```bash
   python3 scripts/release.py --tag vX.Y.Z
   git push origin vX.Y.Z
   ```
   The release workflow fails if the tag isn't annotated, doesn't match `Cargo.toml`, or has no `CHANGELOG.md` section.
4. For a tool's first release, add it to the distribution channels by following `.gemini/prompts/update-distribution-channels.md`. After that, the Homebrew tap and Scoop bucket pick up new releases automatically within a couple of hours.

## Disclaimer

**This tool is provided "as-is", without warranty of any kind. It executes `DROP SCHEMA public CASCADE; CREATE SCHEMA public;` and is DESTRUCTIVE and IRREVERSIBLE: every table and all data in the `public` schema is deleted. Back up anything you need, double-check the project and environment, and use it with extreme caution and at your own risk.**

After the reset, the `liferay` service must be restarted so it can recreate its tables and indexes on the fresh schema. The tool does this automatically when `expect` is available. Otherwise, restart it from the LCP console or the CLI:

```bash
lcp restart -p my-project-dev -s liferay
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

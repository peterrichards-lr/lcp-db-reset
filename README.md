# Liferay Cloud Database Reset Tool (`lcp-db-reset`)

[![Rust CI](https://github.com/peterrichards-lr/lcp-db-reset/actions/workflows/rust.yml/badge.svg)](https://github.com/peterrichards-lr/lcp-db-reset/actions/workflows/rust.yml)
[![Latest Release](https://img.shields.io/github/v/tag/peterrichards-lr/lcp-db-reset?label=version)](https://github.com/peterrichards-lr/lcp-db-reset/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, cross-platform Rust CLI utility to securely and safely reset the database schema of your Liferay Cloud (LCP) environments. It automates finding instance IDs and executing non-interactive PostgreSQL commands inside the `lcp shell`.

## Features

- **Safe:** Fails fast if you aren't authenticated with `lcp`. Requires explicit confirmation before wiping the database.
- **Smart:** Automatically discovers your running Liferay instances and intelligently detects if your project ID already contains an environment suffix.
- **Secure:** Uses hidden prompts for your database password instead of keeping it in your shell history (unless passed via flag).
- **Automated:** Uses `expect` to "type" commands into the interactive `lcp shell` for 100% hands-free execution on macOS/Linux.
- **Cross-Platform:** Native binaries for macOS (Intel/ARM), Linux, and Windows.

## Installation

### macOS / Linux (Homebrew)

```bash
brew tap peterrichards-lr/homebrew-tap
brew install lcp-db-reset
```

_Note: For full automation, ensure `expect` is installed (default on most macOS/Linux systems)._

### Windows (Scoop)

```powershell
scoop bucket add peterrichards-lr https://github.com/peterrichards-lr/scoop-bucket
scoop install lcp-db-reset
```

### Windows Subsystem for Linux (WSL)

The tool works perfectly in WSL! For the best experience (100% automation), ensure `expect` is installed in your WSL distribution:

```bash
# Ubuntu/Debian example
sudo apt update && sudo apt install expect -y
```

### Manual Download

Download the pre-compiled executable for your OS from the [GitHub Releases](https://github.com/peterrichards-lr/lcp-db-reset/releases) page.

### From Source

If you have Rust installed, you can build from source:

```bash
cargo install --path .
```

## Usage

The tool is smart enough to handle project IDs in multiple formats:

```bash
# Option 1: Split project and environment
lcp-db-reset reset -p my-project -e dev

# Option 2: Full LCP project ID
lcp-db-reset reset -p my-project-dev

# Option 3: Interactive (will prompt for environment if missing)
lcp-db-reset reset -p my-project
```

### Options:

- `-p, --project <PROJECT>`: The project ID (e.g., `lfrprj`) or full ID (e.g., `lfrprj-dev`)
- `-e, --environment <ENVIRONMENT>`: The environment (e.g., `dev`, `uat`, `prd`)
- `-s, --service <SERVICE>`: The service name [default: `liferay`]
- `-i, --instance <INSTANCE>`: Specific instance ID (optional, found automatically if missing)
- `-P, --password <PASSWORD>`: Database password (optional, prompted securely if missing)
- `--force`: Skip confirmation prompt

## Disclaimer

This tool executes `DROP SCHEMA public CASCADE; CREATE SCHEMA public;`. It is **DESTRUCTIVE**.

**After resetting the database, you MUST restart the `liferay` service** to allow it to recreate the necessary tables and indexes on a fresh schema. You can do this via the LCP console or the CLI:

```bash
lcp restart -p my-project -e dev -s liferay
```

Use with extreme caution.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

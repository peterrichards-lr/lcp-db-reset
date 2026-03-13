# Liferay Cloud Database Reset Tool (`lcp-db-reset`)

A streamlined, cross-platform CLI tool to securely and safely reset the database schema of your Liferay Cloud (LCP) environments. It automates finding instance IDs and executing non-interactive PostgreSQL commands inside the `lcp shell`.

## Features

- **Safe:** Fails fast if you aren't authenticated with `lcp`. Requires explicit confirmation before wiping the database.
- **Smart:** Automatically discovers your running Liferay instances.
- **Secure:** Uses hidden prompts for your database password instead of keeping it in your shell history (unless passed via flag).
- **Automated:** Bypasses interactive `psql` prompts entirely by piping commands directly into the container.

## Installation

You can download the pre-compiled executable for your OS from the [Releases](#) page.

Alternatively, if you have Rust installed, you can build from source:
```bash
cargo install --path .
```

*(Note: Homebrew and Scoop support can be set up via the included Gemini prompts after your first release!)*

## Usage

```bash
# Basic usage (will prompt for password and confirmation)
lcp-db-reset reset --project my-project-id --environment dev

# Fully automated usage (for CI/CD or scripts)
lcp-db-reset reset -p my-project-id -e dev -s liferay --password "my-secret-pass" --force
```

### Options:
- `-p, --project <PROJECT>`: The project ID (e.g., lfr-prj)
- `-e, --environment <ENVIRONMENT>`: The environment (e.g., dev, uat, prd)
- `-s, --service <SERVICE>`: The service name [default: liferay]
- `-i, --instance <INSTANCE>`: Specific instance ID (optional, found automatically if missing)
- `-P, --password <PASSWORD>`: Database password (optional, prompted if missing)
- `--force`: Skip confirmation prompt

## Disclaimer

This tool executes `DROP SCHEMA public CASCADE; CREATE SCHEMA public;`. It is DESTRUCTIVE. Use with extreme caution.

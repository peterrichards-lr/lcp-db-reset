# Liferay Rust Tool Template

A modular template for building high-performance, cross-platform CLI tools for Liferay DXP, Liferay Cloud (LXC), and Client Extensions.

## New in v0.2.0

- **Project Discovery:** Robust logic to detect Workspace, Liferay Cloud, and Client Extension roots.
- **Git Integration:** Built-in wrappers for common Git operations (add, commit, push).
- **Features Support:** Optional support for Web Scraping (`web`) and YAML processing (`yaml`).
- **Distribution:** Includes a Homebrew formula example for easy macOS distribution.

## Features

- **Cross-Platform:** GitHub Actions pre-configured for Windows, Linux, and macOS (ARM/Intel).
- **Liferay Aware:** Logic for path resolution and product version detection from `gradle.properties`.
- **Modern CLI:** Built on `clap` for a professional command-line experience.

## Project Structure

```plaintext
.
├── .github/workflows/release.yml # Multi-OS CI/CD
├── src/
│   ├── main.rs          # Command routing
│   ├── core/
│   │   ├── mod.rs       # Core traits
│   │   └── env.rs       # Project discovery logic
│   ├── utils/
│   │   ├── mod.rs       # Utility re-exports
│   │   ├── git.rs       # Git wrappers
│   │   └── xml.rs       # Recursive XML logic
│   └── cli.rs           # Command definitions
├── formula.rb.example   # Example for Homebrew distribution
├── .gitignore           # Tracks Cargo.lock for reliable CI
├── Cargo.toml           # Feature-based dependencies
└── LICENSE (MIT)
```

## Getting Started

1. Click **"Use this template"** on GitHub.
2. Update the `name` and `description` in `Cargo.toml`.
3. Enable features in `Cargo.toml` if needed:
   ```toml
   [dependencies]
   lfr-tool = { path = ".", features = ["web", "yaml"] }
   ```
4. Update `artifact_name` in `.github/workflows/release.yml`.
5. Customize subcommands in `src/main.rs`.
6. Push a tag (e.g., `v1.0.0`) to trigger an automated release.

## Development

```bash
# Build locally
cargo build

# Run with arguments
cargo run -- --help
```

## Distribution (macOS, Linux, Windows)

To avoid "Unidentified Developer" warnings on macOS and ensure a secure, user-level installation on Windows, we recommend building from source via **Homebrew** or **Scoop**. 

### Automated Distribution via Gemini

This template includes an automated prompt for Gemini CLI to handle updating your Homebrew tap and Scoop bucket repositories with new releases.

When you create a new GitHub release, you can simply ask Gemini:

```bash
"Please execute the steps in .gemini/prompts/update-distribution-channels.md to update my distribution repositories"
```

Gemini will automatically:
1. Extract metadata from `Cargo.toml`.
2. Calculate the SHA256 hash of the release tarball.
3. Generate and write the Homebrew formula (`formula.rb.example`) and Scoop manifest (`scoop.json.example`).
4. Commit and push the updates to your local `homebrew-tap` and `scoop-bucket` repositories.

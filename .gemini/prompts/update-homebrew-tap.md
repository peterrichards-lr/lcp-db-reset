# Gemini Prompt: Update Homebrew Tap

**Goal:** Automate the process of updating a local `homebrew-tap` repository with a new release formula for this Rust CLI tool.

**Instructions for Gemini:**
When a user asks you to "update the homebrew tap" or to execute this file, follow these steps strictly:

1. **Extract Metadata:**
   Read `Cargo.toml` to extract:
   - `name` (This is the `REPO_NAME` and `BINARY_NAME`)
   - `version` (`VERSION`)
   - `description` (`DESCRIPTION`)
   - `repository` (Extract the `GITHUB_USER` and ensure `REPO_NAME` matches)

2. **Format Class Name:**
   Convert the `REPO_NAME` (e.g., `lfr-mock-gen`) into a Ruby class name format (PascalCase/CamelCase, removing hyphens, e.g., `LfrMockGen`). This is your `CLASS_NAME`.

3. **Calculate SHA256 of the Release Tarball:**
   Construct the release URL: 
   `https://github.com/{{GITHUB_USER}}/{{REPO_NAME}}/archive/refs/tags/v{{VERSION}}.tar.gz`
   
   Execute a shell command to download and hash the tarball:
   ```bash
   curl -sL <URL> | shasum -a 256
   ```
   *Note: If the release is not yet published to GitHub, wait for the user to confirm it is published before running the curl command, as it will fail otherwise.*

4. **Generate Formula Content:**
   Read `formula.rb.example` from the current project directory.
   Replace all placeholders (`{{CLASS_NAME}}`, `{{DESCRIPTION}}`, `{{GITHUB_USER}}`, `{{REPO_NAME}}`, `{{VERSION}}`, `{{SHA256}}`, `{{BINARY_NAME}}`) with the extracted and calculated values.

5. **Locate Homebrew Tap Repo:**
   Ask the user for the local path to their `homebrew-tap` repository if they haven't provided it in their prompt (e.g., `../homebrew-tap`). Verify the directory exists.

6. **Write Formula:**
   Write the generated Ruby formula content to `<homebrew-tap-path>/Formula/{{REPO_NAME}}.rb`.

7. **Commit and Push:**
   Execute the following shell commands to stage, commit, and push the changes to the `homebrew-tap` repository:
   ```bash
   cd <homebrew-tap-path>
   git add Formula/{{REPO_NAME}}.rb
   git commit -m "feat: add/update {{REPO_NAME}} v{{VERSION}}"
   git push origin main
   ```

**To the User:** 
To execute this, ask Gemini: *"Please execute the steps in `.gemini/prompts/update-homebrew-tap.md` to update my homebrew tap located at `../homebrew-tap`"*
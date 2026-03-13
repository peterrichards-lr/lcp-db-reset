mod cli;

use crate::cli::{App, AppCommands};
use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{Confirm, Password};
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let args = App::parse();

    match args.command {
        AppCommands::Reset {
            project,
            environment,
            service,
            instance,
            password,
            force,
        } => {
            reset_db(project, environment, service, instance, password, force)?;
        }
    }

    Ok(())
}

fn reset_db(
    project: String,
    environment: String,
    service: String,
    instance: Option<String>,
    password: Option<String>,
    force: bool,
) -> Result<()> {
    // 1. Find Instance ID if not provided
    let instance_id = if let Some(i) = instance {
        i
    } else {
        println!("No instance ID provided. Searching for running instances...");
        find_instance(&project, &environment, &service)?
    };

    println!("Using instance: {}", instance_id);

    // 2. Get Password if not provided
    let db_password = if let Some(p) = password {
        p
    } else {
        Password::new()
            .with_prompt(format!(
                "Enter database password for {}/{}/{}",
                project, environment, service
            ))
            .interact()?
    };

    // 3. Confirmation
    if !force {
        println!(
            "\nWARNING: This will wipe the 'public' schema in the database for project: {} env: {}!",
            project, environment
        );
        println!("This operation is DESTRUCTIVE and IRREVERSIBLE.");
        if !Confirm::new()
            .with_prompt("Do you want to continue?")
            .default(false)
            .interact()?
        {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    // 4. Construct and Execute commands via lcp shell
    // We'll use a sequence of commands sent to lcp shell's stdin.
    // The user's manual process is:
    // psql -U dxpcloud -d lportal
    // (Enter password)
    // DROP SCHEMA public CASCADE;
    // CREATE SCHEMA public;

    // However, if we can do it in one shot it's better.
    // We'll try to use PGPASSWORD to avoid interactive prompt from psql inside lcp shell.
    // Note: database host is typically 'database' in LCP.

    let sql_commands = "DROP SCHEMA public CASCADE; CREATE SCHEMA public;";
    // We use double quotes for the command to handle single quotes in the SQL.
    let psql_cmd = format!(
        "export PGPASSWORD='{}' && psql -U dxpcloud -h database -d lportal -c \"{}\"",
        db_password, sql_commands
    );

    println!("Resetting database schema... (Executing psql inside lcp shell)");

    let mut child = Command::new("lcp")
        .arg("shell")
        .arg("-p")
        .arg(&project)
        .arg("-e")
        .arg(&environment)
        .arg("-s")
        .arg(&service)
        .arg("-i")
        .arg(&instance_id)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn lcp shell. Is lcp CLI installed?")?;

    {
        let mut stdin = child
            .stdin
            .take()
            .context("Failed to open stdin of lcp shell")?;
        // Send the command and then exit the shell
        writeln!(stdin, "{}", psql_cmd)?;
        writeln!(stdin, "exit")?;
    }

    let status = child.wait()?;
    if status.success() {
        println!("\nDatabase schema reset successfully!");
    } else {
        return Err(anyhow::anyhow!(
            "lcp shell exited with error status: {}",
            status
        ));
    }

    Ok(())
}

fn find_instance(project: &str, environment: &str, service: &str) -> Result<String> {
    // Attempt with lcp list --detailed
    let output = Command::new("lcp")
        .arg("list")
        .arg("-p")
        .arg(project)
        .arg("-e")
        .arg(environment)
        .arg("-s")
        .arg(service)
        .arg("--detailed")
        .output()
        .context("Failed to run lcp list")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(id) = parse_instance_id(&stdout, service) {
            return Ok(id);
        }
    }

    // Fallback to lcp log (as suggested by user tip)
    println!("Could not find instance via 'lcp list', trying 'lcp log'...");
    let output = Command::new("lcp")
        .arg("log")
        .arg("-p")
        .arg(project)
        .arg("-e")
        .arg(environment)
        .arg("-s")
        .arg(service)
        .output()
        .context("Failed to run lcp log")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(id) = parse_instance_id(&stdout, service) {
            return Ok(id);
        }
    }

    Err(anyhow::anyhow!(
        "Could not find a running instance ID for service: {}. Please specify it manually with -i.",
        service
    ))
}

fn parse_instance_id(input: &str, service: &str) -> Option<String> {
    // Basic heuristic: look for words starting with service name and followed by a dash and hex/hash.
    // e.g., liferay-777c...
    for word in input.split_whitespace() {
        // Clean the word from brackets, quotes, etc.
        let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '-');
        if clean_word.starts_with(service) && clean_word.len() > service.len() + 5 {
            // Check if it has a dash and some suffix
            let parts: Vec<&str> = clean_word.split('-').collect();
            if parts.len() >= 2 && parts[0] == service {
                return Some(clean_word.to_string());
            }
        }
    }
    None
}

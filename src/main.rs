mod cli;

use crate::cli::{App, AppCommands};
use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{Confirm, Input, Password};
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let args = App::parse();

    // Fail fast if not logged in
    check_lcp_auth()?;

    match args.command {
        AppCommands::Reset {
            project,
            environment,
            password,
            force,
            service,
            instance,
        } => {
            reset_db(project, environment, password, force, service, instance)?;
        }
    }

    Ok(())
}

fn check_lcp_auth() -> Result<()> {
    let output = Command::new("lcp")
        .arg("auth")
        .arg("token")
        .output()
        .context("Failed to run lcp CLI. Is it installed?")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "You are not logged into Liferay Cloud (lcp). Please run 'lcp login' first."
        ));
    }
    Ok(())
}

fn reset_db(
    project: String,
    environment: Option<String>,
    password: Option<String>,
    force: bool,
    service: String,
    instance: Option<String>,
) -> Result<()> {
    // Determine the full project ID (project-environment)
    let full_project_id = if let Some(env) = environment {
        // If environment is explicitly provided, ensure we don't double-append it
        if project.ends_with(&format!("-{}", env)) {
            project
        } else {
            format!("{}-{}", project, env)
        }
    } else if project.contains('-') {
        // Heuristic: If it contains a hyphen, assume it's already a full project-environment ID
        project
    } else {
        // If no environment provided and no hyphen found, prompt for it
        let env: String = Input::new()
            .with_prompt(format!(
                "Enter environment for project '{}' (e.g. dev, uat, prd)",
                project
            ))
            .interact_text()?;
        format!("{}-{}", project, env)
    };

    // 1. Find Instance ID from logs if not provided
    let instance_id = if let Some(id) = instance {
        println!("Using provided instance: {}", id);
        id
    } else {
        println!(
            "Discovering running instances for project: {} service: {}...",
            full_project_id, service
        );
        let id = find_instance(&full_project_id, &service)?;
        println!("Found instance: {}", id);
        id
    };

    // 2. Get Password if not provided
    let db_password = if let Some(p) = password {
        p
    } else {
        Password::new()
            .with_prompt(format!(
                "Enter database password for project: {}",
                full_project_id
            ))
            .interact()?
    };

    // 3. Confirmation
    if !force {
        println!(
            "\nWARNING: This will wipe the 'public' schema in the database for project: {}!",
            full_project_id
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

    // 4. Provide the command and start interactive shell
    let escaped_password = db_password.replace("'", "'\\''");
    let sql_commands = "DROP SCHEMA public CASCADE; CREATE SCHEMA public;";

    let psql_cmd = format!(
        "export PGPASSWORD='{}' && psql -U dxpcloud -d lportal -c \"{}\"",
        escaped_password, sql_commands
    );

    println!("Resetting database schema...");

    // Check if 'expect' is available for full automation
    let expect_available = Command::new("expect")
        .arg("-v")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if expect_available {
        println!("Automating shell interaction using 'expect'...");

        // Escape quotes and dollar signs for Tcl/Expect
        let escaped_for_expect = psql_cmd.replace("\"", "\\\"").replace("$", "\\$");

        let script = format!(
            r#"#!/usr/bin/expect -f
set timeout -1
spawn lcp shell -p {} -s {} -i {}
expect "$ "
send -- "{}\r"
expect "$ "
send "exit\r"
expect eof
"#,
            full_project_id, service, instance_id, escaped_for_expect
        );

        let mut temp_file = tempfile::NamedTempFile::new().context("Failed to create temp file")?;
        write!(temp_file, "{}", script).context("Failed to write expect script")?;

        let mut child = Command::new("expect")
            .arg(temp_file.path())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to spawn expect.")?;

        let status = child.wait()?;
        if status.success() {
            println!("\nDatabase schema reset successfully via automated shell!");

            println!("Restarting service '{}' to recreate schema...", service);
            let restart_status = Command::new("lcp")
                .arg("restart")
                .arg("-p")
                .arg(&full_project_id)
                .arg("-s")
                .arg(&service)
                .status()
                .context("Failed to run lcp restart")?;

            if restart_status.success() {
                println!("Service restart initiated successfully.");
            } else {
                println!("Warning: Service restart failed or was cancelled.");
            }
        } else {
            return Err(anyhow::anyhow!(
                "Automated shell (expect) exited with error status: {}",
                status
            ));
        }
    } else {
        println!("\n--- MANUAL ACTION REQUIRED ---");
        println!(
            "The lcp CLI does not support non-interactive commands, and 'expect' is not installed."
        );
        println!("Once the shell opens, please COPY and PASTE the following command:\n");
        println!("{}\n", psql_cmd);
        println!("------------------------------\n");

        let mut child = Command::new("lcp")
            .arg("shell")
            .arg("-p")
            .arg(&full_project_id)
            .arg("-s")
            .arg(&service)
            .arg("-i")
            .arg(&instance_id)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to spawn lcp shell.")?;

        let status = child.wait()?;
        if status.success() {
            println!("\nInteractive shell closed.");
        } else {
            return Err(anyhow::anyhow!(
                "lcp shell exited with error status: {}",
                status
            ));
        }
    }

    Ok(())
}

fn find_instance(project: &str, service: &str) -> Result<String> {
    // Run lcp shell without an instance ID to trigger the instance list
    // We pipe "exit" into it so it terminates immediately after showing the list
    let mut child = Command::new("lcp")
        .arg("shell")
        .arg("-p")
        .arg(project)
        .arg("-s")
        .arg(service)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to run lcp shell to list instances")?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "exit");
    }

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    if let Some(id) = parse_instance_id_from_list(&stdout, service) {
        return Ok(id);
    }

    Err(anyhow::anyhow!(
        "Could not find a running instance ID for service: {} in project: {}. \n\
        Output from lcp shell:\n{}",
        service,
        project,
        stdout
    ))
}

fn parse_instance_id_from_list(input: &str, service: &str) -> Option<String> {
    // Look for lines like "1      liferay-5887dc6695-98w4g      Running"
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // A valid line usually has [#] [InstanceName] [Status]
        if parts.len() >= 3 {
            let instance = parts[1];
            let status = parts[2];
            if instance.starts_with(service) && status.to_lowercase() == "running" {
                return Some(instance.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instance_id_from_list() {
        let input = "
#      Instance                      Status       
1      liferay-5887dc6695-98w4g      Running      
";
        assert_eq!(
            parse_instance_id_from_list(input, "liferay"),
            Some("liferay-5887dc6695-98w4g".to_string())
        );
    }
}

mod cli;

use crate::cli::{App, AppCommands};
use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{Confirm, Password};
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let args = App::parse();

    // Fail fast if not logged in
    check_lcp_auth()?;

    match args.command {
        AppCommands::Reset {
            project,
            password,
            force,
            service,
        } => {
            reset_db(project, password, force, service)?;
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

fn reset_db(project: String, password: Option<String>, force: bool, service: String) -> Result<()> {
    // 1. Find Instance ID from logs
    println!(
        "Fetching logs for project: {} service: {} to find instance ID...",
        project, service
    );
    let instance_id = find_instance(&project, &service)?;
    println!("Found instance: {}", instance_id);

    // 2. Get Password if not provided
    let db_password = if let Some(p) = password {
        p
    } else {
        Password::new()
            .with_prompt(format!("Enter database password for project: {}", project))
            .interact()?
    };

    // 3. Confirmation
    if !force {
        println!(
            "\nWARNING: This will wipe the 'public' schema in the database for project: {}!",
            project
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
    let escaped_password = db_password.replace("'", "'\\''");
    let sql_commands = "DROP SCHEMA public CASCADE; CREATE SCHEMA public;";

    let psql_cmd = format!(
        "export PGPASSWORD='{}' && psql -U dxpcloud -h database -d lportal -c \"{}\"",
        escaped_password, sql_commands
    );

    println!("Resetting database schema... (Executing psql inside lcp shell)");

    let mut child = Command::new("lcp")
        .arg("shell")
        .arg("-p")
        .arg(&project)
        .arg("-s")
        .arg(&service)
        .arg("-i")
        .arg(&instance_id)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn lcp shell.")?;

    {
        let mut stdin = child
            .stdin
            .take()
            .context("Failed to open stdin of lcp shell")?;

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

fn find_instance(project: &str, service: &str) -> Result<String> {
    let output = Command::new("lcp")
        .arg("log")
        .arg("-p")
        .arg(project)
        .arg("-s")
        .arg(service)
        .output()
        .context("Failed to run lcp log")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!(
            "Failed to fetch logs from {} on {}: {}",
            project,
            service,
            stderr
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Some(id) = parse_instance_id(&stdout, service) {
        return Ok(id);
    }

    Err(anyhow::anyhow!(
        "Could not find a running instance ID for service: {}. Please ensure it is running.",
        service
    ))
}

fn parse_instance_id(input: &str, service: &str) -> Option<String> {
    // Look for instance ID in the format [service-xxxxx-xxxxx]
    for line in input.lines().rev() {
        if let Some(start) = line.find('[') {
            if let Some(end) = line.find(']') {
                let instance = &line[start + 1..end];
                if instance.starts_with(service) && instance.len() > service.len() + 5 {
                    return Some(instance.to_string());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instance_id_from_logs() {
        let input = "Mar 13, 09:20:06.156  build-5 [liferay-7d7fbb6cb5-tb6dc] [tomcat] WARNING ...";
        assert_eq!(
            parse_instance_id(input, "liferay"),
            Some("liferay-7d7fbb6cb5-tb6dc".to_string())
        );
    }
}

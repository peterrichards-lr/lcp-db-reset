mod cli;
mod core;
mod utils;

use crate::cli::{App, AppCommands};
use crate::core::{LiferayProject, ProjectType, Workspace};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = App::parse();

    let ws = LiferayProject {
        current_dir: std::env::current_dir().unwrap_or_default(),
    };

    match args.command {
        AppCommands::Env { target } => {
            let root = ws.find_root()?;
            let project_type = ws.detect_type(&root);

            println!("Root: {:?}", root);

            let project_desc = match project_type {
                ProjectType::LiferayWorkspace => "Liferay Workspace (Traditional)",
                ProjectType::LiferayCloud => "Liferay Cloud (LXC/DXP Cloud)",
                ProjectType::ClientExtension => "Liferay Client Extension",
                ProjectType::Unknown => "Unknown project structure",
            };
            println!("Project Type: {}", project_desc);

            if let Some(version) = ws.get_liferay_version(&root) {
                println!("Liferay Version: {}", version);
            }

            if let Ok(tomcat_path) = ws.find_tomcat(&root) {
                println!("Tomcat Directory: {:?}", tomcat_path);
            }

            if let Some(t) = target {
                println!("Checking environment for: {}", t);
            }

            // Example using the XML utility
            let _ = utils::find_elements_by_name;
        }
        AppCommands::Data { force } => {
            println!("Data operation initiated (Force={})", force);
        }
    }

    Ok(())
}

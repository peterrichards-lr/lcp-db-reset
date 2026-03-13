use std::fs;
use std::path::{Path, PathBuf};

pub trait Workspace {
    /// Finds the root of the Liferay project (DXP, LXC, or Client Extension)
    fn find_root(&self) -> anyhow::Result<PathBuf>;

    /// Detects the type of Liferay project
    fn detect_type(&self, root: &Path) -> ProjectType;

    /// Returns the Liferay version if detectable (e.g. from gradle.properties)
    fn get_liferay_version(&self, root: &Path) -> Option<String>;

    /// Specifically for local DXP: Finds the Tomcat directory inside 'bundles'
    fn find_tomcat(&self, root: &Path) -> anyhow::Result<PathBuf>;
}

#[derive(Debug, PartialEq)]
pub enum ProjectType {
    LiferayWorkspace,
    LiferayCloud,
    ClientExtension,
    Unknown,
}

pub struct LiferayProject {
    pub current_dir: PathBuf,
}

impl Workspace for LiferayProject {
    fn find_root(&self) -> anyhow::Result<PathBuf> {
        let mut path = self.current_dir.clone();
        loop {
            // Liferay Workspace (Traditional)
            if path.join("bundles").exists()
                || path.join("gradle.properties").exists() && path.join("modules").exists()
            {
                return Ok(path);
            }

            // Liferay Cloud
            if path.join("liferay").exists() || path.join("webserver").exists() {
                return Ok(path);
            }

            // Client Extension
            if path.join("client-extension.yaml").exists() {
                return Ok(path);
            }

            if !path.pop() {
                break;
            }
        }
        anyhow::bail!("Liferay project root not found.")
    }

    fn detect_type(&self, root: &Path) -> ProjectType {
        if root.join("liferay").exists() && root.join("webserver").exists() {
            ProjectType::LiferayCloud
        } else if root.join("client-extension.yaml").exists() {
            ProjectType::ClientExtension
        } else if root.join("bundles").exists() || root.join("gradle.properties").exists() {
            ProjectType::LiferayWorkspace
        } else {
            ProjectType::Unknown
        }
    }

    fn get_liferay_version(&self, root: &Path) -> Option<String> {
        // Search in liferay/gradle.properties (LXC) or root gradle.properties (Workspace)
        let paths = vec![
            root.join("liferay").join("gradle.properties"),
            root.join("gradle.properties"),
        ];

        for path in paths {
            if let Ok(content) = fs::read_to_string(path) {
                for line in content.lines() {
                    if line.starts_with("liferay.workspace.product=") {
                        let product = line.split('=').nth(1)?.trim();
                        // Basic mapping
                        if product.contains("7.4") || product.starts_with("dxp-202") {
                            return Some("7.4".to_string());
                        }
                        if product.contains("7.3") {
                            return Some("7.3".to_string());
                        }
                        if product.contains("7.2") {
                            return Some("7.2".to_string());
                        }
                        if product.contains("7.1") {
                            return Some("7.1".to_string());
                        }
                    }
                }
            }
        }
        None
    }

    fn find_tomcat(&self, root: &Path) -> anyhow::Result<PathBuf> {
        let bundles = root.join("bundles");
        if !bundles.exists() {
            anyhow::bail!("No 'bundles' directory found in root: {:?}", root);
        }

        for entry in fs::read_dir(bundles)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name.starts_with("tomcat-") {
                    return Ok(path);
                }
            }
        }

        anyhow::bail!("No 'tomcat-*' directory found inside 'bundles'")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_detection() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir(root.join("liferay")).unwrap();
        fs::create_dir(root.join("webserver")).unwrap();

        let project = LiferayProject {
            current_dir: root.to_path_buf(),
        };
        assert_eq!(project.detect_type(root), ProjectType::LiferayCloud);
    }

    #[test]
    fn test_find_tomcat() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let bundles = root.join("bundles");
        let tomcat = bundles.join("tomcat-9.0.82");
        fs::create_dir_all(&tomcat).unwrap();

        let project = LiferayProject {
            current_dir: root.to_path_buf(),
        };
        let found = project.find_tomcat(root).unwrap();
        assert_eq!(found, tomcat);
    }
}

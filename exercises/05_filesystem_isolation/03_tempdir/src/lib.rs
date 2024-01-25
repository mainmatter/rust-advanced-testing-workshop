use cargo_manifest::Manifest;
use std::io::BufRead;
use std::path::Path;

fn get_workspace_member_manifests(workspace_root_dir: &Path) -> Vec<String> {
    let workspace_manifest_path = workspace_root_dir.join("Cargo.toml");
    let workspace_manifest = std::fs::read_to_string(&workspace_manifest_path)
        .expect("Failed to read workspace manifest");
    let member_manifest_paths: Vec<_> = toml::from_str::<Manifest>(&workspace_manifest)
        .expect("Failed to parse workspace manifest")
        .workspace
        .unwrap()
        .members
        .iter()
        // Member can contain glob patterns, but we ignore them here for the sake of simplicity.
        // It's an example in the end!
        .map(|path| workspace_root_dir.join(path).join("Cargo.toml"))
        .collect();
    member_manifest_paths
        .iter()
        .map(|path| std::fs::read_to_string(path).expect("Failed to read member manifest"))
        .collect()
}

#[cfg(test)]
mod tests {
    use cargo_manifest::{Manifest, Package, Workspace};
    use googletest::expect_that;
    use googletest::matchers::{eq, len};
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    #[googletest::test]
    fn happy_path() {
        // Arrange
        let workspace_root = todo!();
        let workspace_root_path = todo!();

        let workspace_manifest = Manifest {
            workspace: Some(Workspace {
                members: vec!["api".to_string(), "helpers".to_string()],
                ..Default::default()
            }),
            ..Default::default()
        };
        save_workspace_manifest(workspace_manifest, workspace_root_path);
        let api_manifest = Manifest {
            package: Some(Package::new("api".into(), "0.1.0".into())),
            ..Default::default()
        };
        let helpers_manifest = Manifest {
            package: Some(Package::new("helpers".into(), "0.1.0".into())),
            ..Default::default()
        };
        save_member_manifest(api_manifest, workspace_root_path);
        save_member_manifest(helpers_manifest, workspace_root_path);

        // Act
        let manifests = super::get_workspace_member_manifests(workspace_root_path);

        // Assert
        expect_that!(manifests, len(eq(2)));
    }

    fn save_member_manifest(m: Manifest, workspace_root: &Path) {
        todo!()
    }

    fn save_workspace_manifest(m: Manifest, workspace_root: &Path) {
        todo!()
    }
}

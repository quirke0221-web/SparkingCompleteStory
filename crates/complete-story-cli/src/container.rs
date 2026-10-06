use crate::config::Config;
use crate::process::run_checked;
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct ContainerArtifacts {
    pub pak_path: PathBuf,
    pub utoc_path: PathBuf,
    pub ucas_path: PathBuf,
}

pub fn pack_zen_container(
    config: &Config,
    container_staging_dir: &Path,
    output_dir: &Path,
    container_name: &str,
) -> Result<ContainerArtifacts> {
    if !config.retoc_exe.exists() {
        bail!("retoc executable not found at {:?}", config.retoc_exe);
    }
    if !container_staging_dir.exists() {
        bail!("Container staging directory not found at {:?}", container_staging_dir);
    }

    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create container output dir at {:?}", output_dir))?;

    let utoc_path = output_dir.join(format!("{}.utoc", container_name));
    let pak_path = output_dir.join(format!("{}.pak", container_name));
    let ucas_path = output_dir.join(format!("{}.ucas", container_name));

    // Execute retoc to-zen --version UE5_1 <staging> <utoc>
    let mut cmd = Command::new(&config.retoc_exe);
    cmd.arg("to-zen")
        .arg("--version")
        .arg("UE5_1")
        .arg(container_staging_dir)
        .arg(&utoc_path);

    run_checked(&mut cmd, &format!("Packing Zen container '{}'", container_name))?;

    // Post-flight assertions for all 3 container files
    for (path, ext) in [(&pak_path, ".pak"), (&utoc_path, ".utoc"), (&ucas_path, ".ucas")] {
        if !path.exists() {
            bail!("Container packing failed: {} file was not created at {:?}", ext, path);
        }
        let size = std::fs::metadata(path)?.len();
        if size == 0 {
            bail!("Container packing failed: {} file at {:?} is 0 bytes", ext, path);
        }
    }

    // Verify container chunk hash integrity via retoc verify
    verify_container(config, &utoc_path)?;

    Ok(ContainerArtifacts {
        pak_path,
        utoc_path,
        ucas_path,
    })
}

pub fn verify_container(config: &Config, utoc_path: &Path) -> Result<()> {
    if !utoc_path.exists() {
        bail!("Cannot verify non-existent utoc container at {:?}", utoc_path);
    }

    let mut cmd = Command::new(&config.retoc_exe);
    cmd.arg("verify").arg(utoc_path);

    run_checked(&mut cmd, &format!("Verifying container integrity for {:?}", utoc_path))?;
    Ok(())
}

use crate::config::Config;
use crate::container::ContainerArtifacts;
use anyhow::{bail, Context, Result};
use std::path::Path;

pub fn deploy_to_game(config: &Config, containers: &ContainerArtifacts) -> Result<()> {
    if !config.steam_game_root.exists() {
        bail!("Steam game directory not found at {:?}", config.steam_game_root);
    }

    // 1. Deploy IoStore Containers to Content/Paks/~mods/
    std::fs::create_dir_all(&config.target_paks_mod_dir)
        .with_context(|| format!("Failed to create ~mods directory at {:?}", config.target_paks_mod_dir))?;

    for src_path in [&containers.pak_path, &containers.utoc_path, &containers.ucas_path] {
        let file_name = src_path
            .file_name()
            .context("Invalid container file path")?;
        let dest_path = config.target_paks_mod_dir.join(file_name);

        std::fs::copy(src_path, &dest_path)
            .with_context(|| format!("Failed to copy {:?} to {:?}", src_path, dest_path))?;

        let src_len = std::fs::metadata(src_path)?.len();
        let dest_len = std::fs::metadata(&dest_path)?.len();
        if src_len != dest_len || dest_len == 0 {
            bail!(
                "Deployment verification failed for {:?}: src size ({}) != dest size ({})",
                file_name,
                src_len,
                dest_len
            );
        }
    }

    // 2. Deploy UE4SS Runtime Mod to Binaries/Win64/Mods/CompleteStory/
    if config.runtime_mod_src.exists() {
        std::fs::create_dir_all(&config.target_ue4ss_mod_dir)
            .with_context(|| format!("Failed to create UE4SS mods directory at {:?}", config.target_ue4ss_mod_dir))?;

        copy_directory_recursive(&config.runtime_mod_src, &config.target_ue4ss_mod_dir)?;
    }

    Ok(())
}

fn copy_directory_recursive(src: &Path, dest: &Path) -> Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let target = dest.join(file_name);

        if path.is_dir() {
            std::fs::create_dir_all(&target)?;
            copy_directory_recursive(&path, &target)?;
        } else {
            std::fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

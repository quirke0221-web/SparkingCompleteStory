use crate::config::Config;
use crate::container::ContainerArtifacts;
use anyhow::{bail, Context, Result};

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

    // 2. Build and Deploy Native Runtime Plugin (CompleteStory.asi) to plugins/
    println!("Building native runtime hook (complete-story-runtime)...");
    let build_status = std::process::Command::new("cargo")
        .args(["build", "--release", "-p", "complete-story-runtime"])
        .current_dir(&config.project_root)
        .status()
        .context("Failed to execute cargo build for complete-story-runtime")?;

    if !build_status.success() {
        bail!("Failed to compile complete-story-runtime native plugin");
    }

    if !config.runtime_dll_release.exists() {
        bail!(
            "Compiled runtime DLL not found at {:?}",
            config.runtime_dll_release
        );
    }

    std::fs::create_dir_all(&config.target_plugins_dir).with_context(|| {
        format!(
            "Failed to create plugins directory at {:?}",
            config.target_plugins_dir
        )
    })?;

    let asi_dest = config.target_plugins_dir.join("CompleteStory.asi");
    if let Err(_) = std::fs::copy(&config.runtime_dll_release, &asi_dest) {
        let old_backup = config.target_plugins_dir.join("CompleteStory.asi.old");
        let _ = std::fs::remove_file(&old_backup);
        let _ = std::fs::rename(&asi_dest, &old_backup);
        std::fs::copy(&config.runtime_dll_release, &asi_dest)
            .with_context(|| format!("Failed to copy CompleteStory.asi to {:?}", asi_dest))?;
    }

    let asi_len = std::fs::metadata(&asi_dest)?.len();
    if asi_len == 0 {
        bail!("Deployed CompleteStory.asi is empty");
    }
    println!(
        "Deployed CompleteStory.asi to {:?} ({} bytes)",
        asi_dest, asi_len
    );

    // 3. Clean up legacy UE4SS Lua mod if present to avoid conflicts
    if config.target_ue4ss_mod_dir.exists() {
        let _ = std::fs::remove_dir_all(&config.target_ue4ss_mod_dir);
    }

    Ok(())
}

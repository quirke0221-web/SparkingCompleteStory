use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    pub project_root: PathBuf,
    pub steam_game_root: PathBuf,
    pub retoc_exe: PathBuf,
    pub uassetgui_exe: PathBuf,
    pub usmap_path: PathBuf,
    pub staging_dir: PathBuf,
    pub staging_legacy_dir: PathBuf,
    pub staging_json_dir: PathBuf,
    pub staging_zen_dir: PathBuf,
    pub dist_dir: PathBuf,
    pub runtime_mod_src: PathBuf,
    pub target_paks_mod_dir: PathBuf,
    pub target_ue4ss_mod_dir: PathBuf,
    pub target_ue4ss_log: PathBuf,
}

impl Config {
    pub fn discover() -> Result<Self> {
        let cwd = std::env::current_dir().context("Failed to get current working directory")?;
        let project_root = Self::find_project_root(&cwd)?;

        let steam_default = PathBuf::from(
            r"C:\Program Files (x86)\Steam\steamapps\common\DRAGON BALL Sparking! ZERO",
        );
        let steam_game_root = if steam_default.exists() {
            steam_default
        } else if let Ok(custom) = std::env::var("SPARKING_ZERO_GAME_ROOT") {
            PathBuf::from(custom)
        } else {
            steam_default
        };

        let tools_dir = if project_root.join(".tools").exists() {
            project_root.join(".tools")
        } else {
            project_root.join("tools")
        };
        let retoc_exe = tools_dir.join("retoc").join("retoc.exe");
        let uassetgui_exe = tools_dir.join("UAssetGUI.exe");
        let usmap_path = tools_dir
            .join("Data")
            .join("Mappings")
            .join("SparkingZERO.usmap");

        let build_dir = project_root.join("build");
        let staging_dir = build_dir.join("staging");
        let staging_legacy_dir = staging_dir.join("legacy");
        let staging_json_dir = staging_dir.join("json");
        let staging_zen_dir = staging_dir.join("zen");
        let dist_dir = build_dir.join("dist");
        let runtime_mod_src = project_root.join("CompleteStory");

        let target_paks_mod_dir = steam_game_root
            .join("SparkingZERO")
            .join("Content")
            .join("Paks")
            .join("~mods");

        let target_ue4ss_mod_dir = steam_game_root
            .join("SparkingZERO")
            .join("Binaries")
            .join("Win64")
            .join("Mods")
            .join("CompleteStory");

        let target_ue4ss_log = steam_game_root
            .join("SparkingZERO")
            .join("Binaries")
            .join("Win64")
            .join("ue4ss.log");

        Ok(Self {
            project_root,
            steam_game_root,
            retoc_exe,
            uassetgui_exe,
            usmap_path,
            staging_dir,
            staging_legacy_dir,
            staging_json_dir,
            staging_zen_dir,
            dist_dir,
            runtime_mod_src,
            target_paks_mod_dir,
            target_ue4ss_mod_dir,
            target_ue4ss_log,
        })
    }

    fn find_project_root(start: &Path) -> Result<PathBuf> {
        let mut curr = start;
        loop {
            if curr.join("Cargo.toml").exists() && curr.join("CompleteStory").exists() {
                return Ok(curr.to_path_buf());
            }
            match curr.parent() {
                Some(parent) => curr = parent,
                None => break,
            }
        }
        if start.join("CompleteStory").exists() {
            return Ok(start.to_path_buf());
        }
        bail!(
            "Could not find project root starting from {:?}. Expected Cargo.toml and CompleteStory/",
            start
        );
    }

    pub fn get_aes_key(&self) -> Result<String> {
        match std::env::var("SPARKING_ZERO_AES_KEY") {
            Ok(key) if !key.trim().is_empty() => Ok(key.trim().to_string()),
            _ => bail!(
                "Environment variable SPARKING_ZERO_AES_KEY is missing or empty. Required for retoc."
            ),
        }
    }

    pub fn game_utoc_path(&self) -> PathBuf {
        self.steam_game_root
            .join("SparkingZERO")
            .join("Content")
            .join("Paks")
            .join("SparkingZERO-Windows.utoc")
    }
}

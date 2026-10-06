use crate::config::Config;
use crate::process::run_checked;
use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn extract_stock_assets(config: &Config) -> Result<()> {
    let aes = config.get_aes_key()?;
    let paks_dir = config
        .steam_game_root
        .join("SparkingZERO")
        .join("Content")
        .join("Paks");

    if !config.retoc_exe.exists() {
        bail!("retoc executable not found at {:?}", config.retoc_exe);
    }
    if !paks_dir.exists() {
        bail!("Game Paks directory not found at {:?}", paks_dir);
    }

    std::fs::create_dir_all(&config.staging_legacy_dir)
        .with_context(|| format!("Failed to create staging legacy dir at {:?}", config.staging_legacy_dir))?;

    let filters = [
        "SparkingZERO/Content/SS/Blueprints/DragonAdventureIFData",
        "SparkingZERO/Content/SS/Blueprints/DragonAdventureIFChartData",
        "SparkingZERO/Content/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00",
    ];

    for filter in &filters {
        let mut cmd = Command::new(&config.retoc_exe);
        cmd.arg("--aes-key")
            .arg(&aes)
            .arg("to-legacy")
            .arg("--version")
            .arg("UE5_1")
            .arg("--filter")
            .arg(filter)
            .arg(&paks_dir)
            .arg(&config.staging_legacy_dir);

        run_checked(&mut cmd, &format!("Extracting filter: {}", filter))?;
    }

    Ok(())
}

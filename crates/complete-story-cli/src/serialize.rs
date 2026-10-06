use crate::config::Config;
use crate::process::run_checked;
use anyhow::{bail, Result};
use std::path::Path;
use std::process::Command;

pub fn uasset_to_json(
    config: &Config,
    source_uasset: &Path,
    dest_json: &Path,
) -> Result<()> {
    if !source_uasset.exists() {
        bail!("Source uasset file not found at {:?}", source_uasset);
    }
    if !config.uassetgui_exe.exists() {
        bail!("UAssetGUI executable not found at {:?}", config.uassetgui_exe);
    }

    if let Some(parent) = dest_json.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut cmd = Command::new(&config.uassetgui_exe);
    cmd.arg("--portable")
        .arg("tojson")
        .arg(source_uasset)
        .arg(dest_json)
        .arg("VER_UE5_1")
        .arg("SparkingZERO");

    run_checked(&mut cmd, &format!("Serializing {:?} to JSON", source_uasset))?;

    // Post-flight assertion (WinForms exits 0 on failure)
    if !dest_json.exists() {
        bail!(
            "UAssetGUI tojson failed silently: destination JSON {:?} was not created",
            dest_json
        );
    }
    let metadata = std::fs::metadata(dest_json)?;
    if metadata.len() == 0 {
        bail!(
            "UAssetGUI tojson failed: destination JSON {:?} is 0 bytes",
            dest_json
        );
    }

    Ok(())
}

pub fn json_to_uasset(
    config: &Config,
    source_json: &Path,
    dest_uasset: &Path,
) -> Result<()> {
    if !source_json.exists() {
        bail!("Source JSON file not found at {:?}", source_json);
    }
    if !config.uassetgui_exe.exists() {
        bail!("UAssetGUI executable not found at {:?}", config.uassetgui_exe);
    }

    if let Some(parent) = dest_uasset.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut cmd = Command::new(&config.uassetgui_exe);
    cmd.arg("--portable")
        .arg("fromjson")
        .arg(source_json)
        .arg(dest_uasset)
        .arg("SparkingZERO");

    run_checked(&mut cmd, &format!("Compiling {:?} to UAsset", source_json))?;

    // Post-flight assertions for both .uasset and companion .uexp
    let companion_uexp = dest_uasset.with_extension("uexp");

    if !dest_uasset.exists() {
        bail!(
            "UAssetGUI fromjson failed silently: destination .uasset {:?} was not created",
            dest_uasset
        );
    }
    if std::fs::metadata(dest_uasset)?.len() == 0 {
        bail!(
            "UAssetGUI fromjson failed: destination .uasset {:?} is 0 bytes",
            dest_uasset
        );
    }

    if !companion_uexp.exists() {
        bail!(
            "UAssetGUI fromjson failed: companion .uexp {:?} was not created",
            companion_uexp
        );
    }
    if std::fs::metadata(&companion_uexp)?.len() == 0 {
        bail!(
            "UAssetGUI fromjson failed: companion .uexp {:?} is 0 bytes",
            companion_uexp
        );
    }

    Ok(())
}

pub mod cli;
pub mod config;
pub mod container;
pub mod deploy;
pub mod extract;
pub mod process;
pub mod release;
pub mod serialize;
pub mod transform;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{BuildArgs, Cli, Commands};
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::discover()?;

    match cli.command {
        Commands::Build(args) => run_build(&config, &args)?,
        Commands::Pack => run_pack(&config)?,
        Commands::Deploy => run_deploy(&config)?,
        Commands::Logs => run_logs(&config)?,
    }

    Ok(())
}

fn run_build(config: &Config, args: &BuildArgs) -> Result<()> {
    println!("=== Complete Story Mod Pipeline ===");

    // Stage 1: Targeted Extraction
    if !args.skip_extract {
        println!(">>> Stage 1: Extracting stock assets via retoc to-legacy...");
        extract::extract_stock_assets(config)?;
    } else {
        println!(">>> Stage 1: Skipping extraction (--skip-extract set)");
    }

    // Stage 2: JSON Deserialization
    println!(">>> Stage 2: Deserializing stock assets to JSON via UAssetGUI...");
    let stock_assets = [
        (
            config.staging_legacy_dir.join("SparkingZERO/Content/SS/Blueprints/DragonAdventureIFData.uasset"),
            config.staging_json_dir.join("DragonAdventureIFData.json"),
        ),
        (
            config.staging_legacy_dir.join("SparkingZERO/Content/SS/Blueprints/DragonAdventureIFChartData.uasset"),
            config.staging_json_dir.join("DragonAdventureIFChartData.json"),
        ),
        (
            config.staging_legacy_dir.join("SparkingZERO/Content/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00.uasset"),
            config.staging_json_dir.join("DAIF_CharaData_0000_00.json"),
        ),
    ];

    for (uasset, json) in &stock_assets {
        serialize::uasset_to_json(config, uasset, json)?;
    }

    // Stage 3: Pure Domain JSON AST Transformation
    println!(">>> Stage 3: Transforming JSON asset ASTs via serde_json...");
    let modified_json_dir = config.staging_json_dir.join("modified");
    let transforms = transform::transform_all_assets(&config.staging_json_dir, &modified_json_dir)?;

    // Stage 4: Compile Modified JSON Back to UAsset
    println!(">>> Stage 4: Compiling modified JSON to binary assets via UAssetGUI...");
    let container_staging = config.staging_dir.join("container");
    let outputs = [
        (
            transforms.registry_json,
            container_staging.join("SparkingZERO/Content/SS/Blueprints/DragonAdventureIFData.uasset"),
        ),
        (
            transforms.chart_json,
            container_staging.join("SparkingZERO/Content/SS/Blueprints/DragonAdventureIFChartData.uasset"),
        ),
        (
            transforms.character_json,
            container_staging.join("SparkingZERO/Content/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory.uasset"),
        ),
    ];

    for (json, uasset) in &outputs {
        serialize::json_to_uasset(config, json, uasset)?;
    }

    // Copy scriptobjects.bin from legacy staging
    let scriptobjects = config.staging_legacy_dir.join("scriptobjects.bin");
    if scriptobjects.exists() {
        std::fs::copy(&scriptobjects, container_staging.join("scriptobjects.bin"))
            .with_context(|| "Failed to copy scriptobjects.bin to container staging")?;
    }

    // Stage 5: Pack IoStore Container & Verify
    println!(">>> Stage 5: Packing Zen IoStore container via retoc to-zen...");
    let containers = container::pack_zen_container(
        config,
        &container_staging,
        &config.staging_zen_dir,
        "CompleteStory_P",
    )?;

    // Stage 6: Package Release Archive
    println!(">>> Stage 6: Packaging release archive via zip...");
    let zip_path = release::create_release_zip(config, &containers, "CompleteStory-Release")?;
    println!(">>> Release archive generated: {:?}", zip_path);

    // Optional Deployment
    if args.deploy {
        println!(">>> Stage 7: Deploying container and runtime mod to Steam directory...");
        deploy::deploy_to_game(config, &containers)?;
        println!(">>> Deployed successfully to {:?}", config.target_paks_mod_dir);
    }

    println!("=== Build Complete ===");
    Ok(())
}

fn run_pack(config: &Config) -> Result<()> {
    let container_staging = config.staging_dir.join("container");
    container::pack_zen_container(config, &container_staging, &config.staging_zen_dir, "CompleteStory_P")?;
    Ok(())
}

fn run_deploy(config: &Config) -> Result<()> {
    let containers = container::ContainerArtifacts {
        pak_path: config.staging_zen_dir.join("CompleteStory_P.pak"),
        utoc_path: config.staging_zen_dir.join("CompleteStory_P.utoc"),
        ucas_path: config.staging_zen_dir.join("CompleteStory_P.ucas"),
    };
    deploy::deploy_to_game(config, &containers)?;
    println!(">>> Deployed successfully to {:?}", config.target_paks_mod_dir);
    Ok(())
}

fn run_logs(config: &Config) -> Result<()> {
    if config.target_runtime_log.exists() {
        println!("=== CompleteStoryRuntime.log ({:?}) ===", config.target_runtime_log);
        let content = std::fs::read_to_string(&config.target_runtime_log)?;
        for line in content.lines().rev().take(50).collect::<Vec<_>>().into_iter().rev() {
            println!("{}", line);
        }
        println!();
    } else {
        println!("No CompleteStoryRuntime.log found at {:?}", config.target_runtime_log);
    }

    if config.target_ue4ss_log.exists() {
        println!("=== ue4ss.log ({:?}) ===", config.target_ue4ss_log);
        let content = std::fs::read_to_string(&config.target_ue4ss_log)?;
        for line in content.lines().rev().take(30).collect::<Vec<_>>().into_iter().rev() {
            println!("{}", line);
        }
    } else {
        println!("No ue4ss.log found at {:?}", config.target_ue4ss_log);
    }
    Ok(())
}

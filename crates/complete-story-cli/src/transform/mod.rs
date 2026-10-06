pub mod character;
pub mod chart;
pub mod registry;

use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub struct TransformOutputs {
    pub registry_json: std::path::PathBuf,
    pub chart_json: std::path::PathBuf,
    pub character_json: std::path::PathBuf,
}

pub fn transform_all_assets(
    json_dir: &Path,
    output_dir: &Path,
) -> Result<TransformOutputs> {
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory at {:?}", output_dir))?;

    let in_char_path = json_dir.join("DAIF_CharaData_0000_00.json");
    let in_reg_path = json_dir.join("DragonAdventureIFData.json");
    let in_chart_path = json_dir.join("DragonAdventureIFChartData.json");

    let out_char_path = output_dir.join("DAIF_CharaData_CompleteStory.json");
    let out_reg_path = output_dir.join("DragonAdventureIFData.modified.json");
    let out_chart_path = output_dir.join("DragonAdventureIFChartData.modified.json");

    // 1. Transform Character Data Asset
    let mut char_ast = read_json(&in_char_path)?;
    character::transform_character_ast(&mut char_ast)
        .with_context(|| "Failed transforming character data asset AST")?;
    write_json(&out_char_path, &char_ast)?;

    // 2. Transform Master Registry Asset
    let mut reg_ast = read_json(&in_reg_path)?;
    registry::transform_registry_ast(&mut reg_ast)
        .with_context(|| "Failed transforming master registry asset AST")?;
    write_json(&out_reg_path, &reg_ast)?;

    // 3. Transform Chart Registry Asset
    let mut chart_ast = read_json(&in_chart_path)?;
    chart::transform_chart_ast(&mut chart_ast)
        .with_context(|| "Failed transforming chart registry asset AST")?;
    write_json(&out_chart_path, &chart_ast)?;

    Ok(TransformOutputs {
        registry_json: out_reg_path,
        chart_json: out_chart_path,
        character_json: out_char_path,
    })
}

fn read_json(path: &Path) -> Result<Value> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open JSON file at {:?}", path))?;
    let reader = BufReader::new(file);
    let val: Value = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to parse JSON file at {:?}", path))?;
    Ok(val)
}

fn write_json(path: &Path, val: &Value) -> Result<()> {
    let file = File::create(path)
        .with_context(|| format!("Failed to create JSON file at {:?}", path))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, val)
        .with_context(|| format!("Failed to write pretty JSON at {:?}", path))?;
    Ok(())
}

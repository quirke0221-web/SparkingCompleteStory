use anyhow::{bail, Context, Result};
use serde_json::Value;

pub const OLD_OBJECT_NAME: &str = "DAIF_CharaData_0000_00";
pub const NEW_OBJECT_NAME: &str = "DAIF_CharaData_CompleteStory";
pub const OLD_PACKAGE_NAME: &str =
    "/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00";
pub const NEW_PACKAGE_NAME: &str =
    "/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory";

pub fn transform_character_ast(root: &mut Value) -> Result<()> {
    // 1. Update export ObjectName
    {
        let exports = root
            .get_mut("Exports")
            .and_then(Value::as_array_mut)
            .context("Missing 'Exports' array in character JSON AST")?;

        if exports.len() != 1 {
            bail!(
                "Expected exactly 1 export in character data, found {}",
                exports.len()
            );
        }

        let export = &mut exports[0];
        let obj_name = export
            .get("ObjectName")
            .and_then(Value::as_str)
            .context("Missing 'ObjectName' in character Export")?;

        if obj_name != OLD_OBJECT_NAME {
            bail!(
                "Expected export ObjectName '{}', found '{}'",
                OLD_OBJECT_NAME,
                obj_name
            );
        }

        export["ObjectName"] = Value::String(NEW_OBJECT_NAME.to_string());
    }

    // 2. Update NameMap entries
    let name_count = {
        let name_map = root
            .get_mut("NameMap")
            .and_then(Value::as_array_mut)
            .context("Missing 'NameMap' array in character JSON AST")?;

        for entry in name_map.iter_mut() {
            if let Some(s) = entry.as_str() {
                if s == OLD_OBJECT_NAME {
                    *entry = Value::String(NEW_OBJECT_NAME.to_string());
                } else if s == OLD_PACKAGE_NAME {
                    *entry = Value::String(NEW_PACKAGE_NAME.to_string());
                }
            }
        }
        name_map.len()
    };

    // 3. Update FolderName if present
    if root.get("FolderName").is_some() {
        root["FolderName"] = Value::String(NEW_PACKAGE_NAME.to_string());
    }

    // 4. Update Generations NameCount if present
    if let Some(gens) = root.get_mut("Generations").and_then(Value::as_array_mut) {
        if let Some(gen0) = gens.first_mut() {
            gen0["NameCount"] = Value::from(name_count);
        }
    }

    Ok(())
}

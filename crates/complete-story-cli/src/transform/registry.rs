use crate::transform::character::{NEW_OBJECT_NAME, NEW_PACKAGE_NAME};
use anyhow::{bail, Context, Result};
use serde_json::Value;

pub const ROUTE_KEY: &str = "9999_00";

pub fn transform_registry_ast(root: &mut Value) -> Result<()> {
    // 1. Validate layout and clone template record
    let template_record = {
        let exports = root
            .get("Exports")
            .and_then(Value::as_array)
            .context("Missing 'Exports' in registry JSON")?;

        if exports.len() != 1 {
            bail!("Expected 1 export in registry, found {}", exports.len());
        }

        let export_data = exports[0]
            .get("Data")
            .and_then(Value::as_array)
            .context("Missing 'Data' array in registry Export")?;

        let records_prop = export_data
            .iter()
            .find(|p| p.get("Name").and_then(Value::as_str) == Some("PtrRecords"))
            .context("Missing 'PtrRecords' property in registry export")?;

        let records_val = records_prop
            .get("Value")
            .and_then(Value::as_array)
            .context("Missing 'Value' array in PtrRecords")?;

        if records_val.len() != 12 {
            bail!(
                "Expected 12 stock PtrRecords entries in registry, found {}",
                records_val.len()
            );
        }

        for rec in records_val.iter() {
            if let Some(key_val) = rec.pointer("/0/Value/0/Value").and_then(Value::as_str) {
                if key_val == ROUTE_KEY {
                    bail!("Route key '{}' already exists in registry PtrRecords", ROUTE_KEY);
                }
            }
        }

        records_val[0].clone()
    };

    // 2. Update NameMap
    {
        let ref_count = root
            .get("NamesReferencedFromExportDataCount")
            .and_then(Value::as_i64)
            .context("Missing 'NamesReferencedFromExportDataCount'")? as usize;

        let name_map = root
            .get_mut("NameMap")
            .and_then(Value::as_array_mut)
            .context("Missing 'NameMap' in registry JSON")?;

        if ref_count > name_map.len() {
            bail!("Invalid NamesReferencedFromExportDataCount: {} > {}", ref_count, name_map.len());
        }

        name_map.insert(ref_count, Value::String(ROUTE_KEY.to_string()));
        name_map.push(Value::String(NEW_PACKAGE_NAME.to_string()));
        name_map.push(Value::String(NEW_OBJECT_NAME.to_string()));
        let total_names = name_map.len();

        root["NamesReferencedFromExportDataCount"] = Value::from(ref_count + 1);
        if let Some(gens) = root.get_mut("Generations").and_then(Value::as_array_mut) {
            if let Some(gen0) = gens.first_mut() {
                gen0["NameCount"] = Value::from(total_names);
            }
        }
    }

    // 3. Add Imports
    let (_pkg_import_idx, obj_import_idx) = {
        let imports = root
            .get_mut("Imports")
            .and_then(Value::as_array_mut)
            .context("Missing 'Imports' array in registry JSON")?;

        if imports.len() != 27 {
            bail!("Expected 27 stock imports in registry, found {}", imports.len());
        }

        let pkg_idx = imports.len(); // 27
        let mut pkg_import = imports[1].clone();
        pkg_import["ObjectName"] = Value::String(NEW_PACKAGE_NAME.to_string());
        pkg_import["OuterIndex"] = Value::from(0);
        pkg_import["ClassPackage"] = Value::String("/Script/CoreUObject".to_string());
        pkg_import["ClassName"] = Value::String("Package".to_string());
        pkg_import["PackageName"] = Value::Null;
        pkg_import["bImportOptional"] = Value::Bool(false);
        imports.push(pkg_import);

        let obj_idx = imports.len(); // 28
        let mut obj_import = imports[14].clone();
        obj_import["ObjectName"] = Value::String(NEW_OBJECT_NAME.to_string());
        obj_import["OuterIndex"] = Value::from(-((pkg_idx as i64) + 1));
        obj_import["ClassPackage"] = Value::String("/Script/SS".to_string());
        obj_import["ClassName"] = Value::String("SSDragonAdventureIFCharacterDataAsset".to_string());
        obj_import["PackageName"] = Value::Null;
        obj_import["bImportOptional"] = Value::Bool(false);
        imports.push(obj_import);

        (pkg_idx, obj_idx)
    };

    // 4. Create and append 13th PtrRecord and dependency
    {
        let exports = root
            .get_mut("Exports")
            .and_then(Value::as_array_mut)
            .context("Missing 'Exports' in registry JSON")?;

        let mut new_record = template_record;
        new_record[0]["Value"][0]["Value"] = Value::String(ROUTE_KEY.to_string());
        new_record[1]["Value"] = Value::from(-((obj_import_idx as i64) + 1));

        let export_data = exports[0]
            .get_mut("Data")
            .and_then(Value::as_array_mut)
            .context("Missing 'Data' array in registry Export")?;

        let records_prop = export_data
            .iter_mut()
            .find(|p| p.get("Name").and_then(Value::as_str) == Some("PtrRecords"))
            .context("Missing 'PtrRecords' property in registry export")?;

        let records_val = records_prop
            .get_mut("Value")
            .and_then(Value::as_array_mut)
            .context("Missing 'Value' array in PtrRecords")?;

        // Replace slot 8 (cut Cell placeholder 0153_00) with Complete Story
        records_val[8] = new_record;

        if records_val.len() != 12 {
            bail!("Postcondition failed: expected 12 records, found {}", records_val.len());
        }

        let deps = exports[0]
            .get_mut("CreateBeforeCreateDependencies")
            .and_then(Value::as_array_mut)
            .context("Missing 'CreateBeforeCreateDependencies'")?;
        let new_dep = Value::from(-((obj_import_idx as i64) + 1));
        if let Some(pos) = deps.iter().position(|d| d.as_i64() == Some(-23)) {
            deps[pos] = new_dep;
        } else {
            deps.push(new_dep);
        }
    }

    let final_imports = root.get("Imports").and_then(Value::as_array).unwrap().len();
    if final_imports != 29 {
        bail!("Postcondition failed: expected 29 imports, found {}", final_imports);
    }

    Ok(())
}

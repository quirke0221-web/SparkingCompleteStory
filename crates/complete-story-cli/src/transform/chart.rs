use crate::transform::registry::ROUTE_KEY;
use anyhow::{bail, Context, Result};
use serde_json::Value;

pub const GOKU_CHART_KEY: &str = "0000_40";

pub fn transform_chart_ast(root: &mut Value) -> Result<()> {
    // 1. Validate layout and extract/clone Goku's record
    let mut new_record = {
        let exports = root
            .get("Exports")
            .and_then(Value::as_array)
            .context("Missing 'Exports' array in chart JSON")?;

        if exports.len() != 1 {
            bail!("Expected 1 export in chart data, found {}", exports.len());
        }

        let export_data = exports[0]
            .get("Data")
            .and_then(Value::as_array)
            .context("Missing 'Data' in chart Export")?;

        let records_prop = export_data
            .iter()
            .find(|p| p.get("Name").and_then(Value::as_str) == Some("PtrRecords"))
            .context("Missing 'PtrRecords' in chart export")?;

        let records_val = records_prop
            .get("Value")
            .and_then(Value::as_array)
            .context("Missing 'Value' array in chart PtrRecords")?;

        if records_val.len() != 12 {
            bail!(
                "Expected 12 stock PtrRecords in chart data, found {}",
                records_val.len()
            );
        }

        let mut goku_rec_opt = None;
        for rec in records_val.iter() {
            if let Some(key) = rec.pointer("/0/Value/0/Value").and_then(Value::as_str) {
                if key == ROUTE_KEY {
                    bail!("Route key '{}' already exists in chart PtrRecords", ROUTE_KEY);
                }
                if key == GOKU_CHART_KEY {
                    goku_rec_opt = Some(rec.clone());
                }
            }
        }

        goku_rec_opt.context("Goku chart record ('0000_40') not found")?
    };

    // 2. Mutate cloned record to have ROUTE_KEY
    {
        let key_node = new_record
            .pointer_mut("/0/Value/0/Value")
            .context("Invalid key structure in Goku chart record")?;
        *key_node = Value::String(ROUTE_KEY.to_string());
    }

    // 3. Update NameMap and Generations
    {
        let ref_count = root
            .get("NamesReferencedFromExportDataCount")
            .and_then(Value::as_i64)
            .context("Missing 'NamesReferencedFromExportDataCount' in chart JSON")? as usize;

        let name_map = root
            .get_mut("NameMap")
            .and_then(Value::as_array_mut)
            .context("Missing 'NameMap' in chart JSON")?;

        if ref_count > name_map.len() {
            bail!("Invalid NamesReferencedFromExportDataCount: {} > {}", ref_count, name_map.len());
        }

        name_map.insert(ref_count, Value::String(ROUTE_KEY.to_string()));
        let total_names = name_map.len();

        root["NamesReferencedFromExportDataCount"] = Value::from(ref_count + 1);

        if let Some(gens) = root.get_mut("Generations").and_then(Value::as_array_mut) {
            if let Some(gen0) = gens.first_mut() {
                gen0["NameCount"] = Value::from(total_names);
            }
        }
    }

    // 4. Append new record to PtrRecords
    {
        let exports = root
            .get_mut("Exports")
            .and_then(Value::as_array_mut)
            .context("Missing 'Exports' array in chart JSON")?;

        let export_data = exports[0]
            .get_mut("Data")
            .and_then(Value::as_array_mut)
            .context("Missing 'Data' in chart Export")?;

        let records_prop = export_data
            .iter_mut()
            .find(|p| p.get("Name").and_then(Value::as_str) == Some("PtrRecords"))
            .context("Missing 'PtrRecords' in chart export")?;

        let records_val = records_prop
            .get_mut("Value")
            .and_then(Value::as_array_mut)
            .context("Missing 'Value' array in chart PtrRecords")?;

        // Prune placeholder slots (indices 8..), retaining 8 official playable campaigns
        records_val.truncate(8);
        records_val.push(new_record);

        if records_val.len() != 9 {
            bail!(
                "Chart postcondition failed: expected 9 records (8 official + Complete Story), found {}",
                records_val.len()
            );
        }
    }

    Ok(())
}

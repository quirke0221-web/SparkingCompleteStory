use anyhow::{bail, Context, Result};
use serde_json::Value;

pub const OLD_OBJECT_NAME: &str = "DAIF_CharaData_0000_00";
pub const NEW_OBJECT_NAME: &str = "DAIF_CharaData_CompleteStory";
pub const OLD_PACKAGE_NAME: &str =
    "/Game/SS/MasterDataAsset/DragonAdventureIF/0000_00/DAIF_CharaData_0000_00";
pub const NEW_PACKAGE_NAME: &str =
    "/Game/SS/MasterDataAsset/DragonAdventureIF/CompleteStory/DAIF_CharaData_CompleteStory";
pub const CUSTOM_DISPLAY_NAME: &str = "Complete Story";

pub fn transform_character_ast(root: &mut Value) -> Result<()> {
    // 1. Update export ObjectName and mutate binary FText payload
    {
        let exports = root
            .get_mut("Exports")
            .and_then(Value::as_array_mut)
            .context("Missing 'Exports' array in character JSON AST")?;

        if exports.len() != 1 {
            bail!("Expected exactly 1 export in character data, found {}", exports.len());
        }

        let export = &mut exports[0];
        let obj_name = export
            .get("ObjectName")
            .and_then(Value::as_str)
            .context("Missing 'ObjectName' in character Export")?;

        if obj_name != OLD_OBJECT_NAME {
            bail!("Expected export ObjectName '{}', found '{}'", OLD_OBJECT_NAME, obj_name);
        }

        export["ObjectName"] = Value::String(NEW_OBJECT_NAME.to_string());

        // Mutate binary CharacterName FText payload in RawExport Data
        let raw_b64 = export
            .get("Data")
            .and_then(Value::as_str)
            .context("Missing 'Data' string in character export")?;

        let bytes = base64_decode(raw_b64)?;
        if bytes.len() < 50 {
            bail!("Character data payload too small: {} bytes < 50", bytes.len());
        }

        // FText with ETextHistoryType::Base (Flags=0, HistoryType=0, Namespace="", Key="", SourceString="Complete Story")
        let name_bytes = format!("{}\0", CUSTOM_DISPLAY_NAME).into_bytes();
        let mut new_ftext = Vec::with_capacity(17 + name_bytes.len());
        new_ftext.extend_from_slice(&0u32.to_le_bytes()); // Flags = 0
        new_ftext.push(0x00); // HistoryType = 0 (Base)
        new_ftext.extend_from_slice(&0i32.to_le_bytes()); // Namespace length = 0
        new_ftext.extend_from_slice(&0i32.to_le_bytes()); // Key length = 0
        new_ftext.extend_from_slice(&(name_bytes.len() as i32).to_le_bytes()); // SourceString length = 15
        new_ftext.extend_from_slice(&name_bytes); // "Complete Story\0"

        // Splice: Header (0..8) + new_ftext (32 bytes) + Tail (50..)
        let mut new_payload = Vec::with_capacity(8 + new_ftext.len() + bytes.len() - 50);
        new_payload.extend_from_slice(&bytes[..8]);
        new_payload.extend_from_slice(&new_ftext);
        new_payload.extend_from_slice(&bytes[50..]);

        let new_len = new_payload.len();
        export["Data"] = Value::String(base64_encode(&new_payload));
        export["SerialSize"] = Value::from(new_len as i64);
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

pub fn base64_decode(input: &str) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut buf: u32 = 0;
    let mut bits = 0;
    for &b in input.as_bytes() {
        if b == b'=' || b.is_ascii_whitespace() {
            continue;
        }
        let val = match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => bail!("Invalid base64 character: {}", b as char),
        };
        buf = (buf << 6) | (val as u32);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    Ok(out)
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < input.len() {
        let b0 = input[i];
        let b1 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let b2 = if i + 2 < input.len() { input[i + 2] } else { 0 };

        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < input.len() {
            out.push(TABLE[(((b1 & 0xF) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < input.len() {
            out.push(TABLE[(b2 & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_roundtrip() {
        let test_data = b"Hello Sparking Zero Complete Story \x00\xFF\x42";
        let encoded = base64_encode(test_data);
        let decoded = base64_decode(&encoded).expect("Decode should succeed");
        assert_eq!(decoded, test_data);
    }

    #[test]
    fn test_character_ast_transform_payload() {
        // Construct dummy 364-byte payload
        let mut dummy_bytes = vec![0u8; 364];
        dummy_bytes[0..8].copy_from_slice(&[0, 2, 1, 6, 1, 10, 2, 15]);
        let b64 = base64_encode(&dummy_bytes);

        let mut ast = serde_json::json!({
            "Exports": [{
                "ObjectName": "DAIF_CharaData_0000_00",
                "Data": b64,
                "SerialSize": 364
            }],
            "NameMap": ["DAIF_CharaData_0000_00"]
        });

        transform_character_ast(&mut ast).expect("Transform should succeed");

        assert_eq!(ast["Exports"][0]["ObjectName"], "DAIF_CharaData_CompleteStory");
        assert_eq!(ast["Exports"][0]["SerialSize"], 354);

        let new_b64 = ast["Exports"][0]["Data"].as_str().unwrap();
        let new_bytes = base64_decode(new_b64).unwrap();
        assert_eq!(new_bytes.len(), 354);
        assert_eq!(&new_bytes[0..8], &[0, 2, 1, 6, 1, 10, 2, 15]);
        assert_eq!(&new_bytes[8..12], &[0, 0, 0, 0]); // Flags = 0
        assert_eq!(new_bytes[12], 0x00); // HistoryType = Base (0)
        assert_eq!(&new_bytes[13..17], &[0, 0, 0, 0]); // Namespace length = 0
        assert_eq!(&new_bytes[17..21], &[0, 0, 0, 0]); // Key length = 0
        assert_eq!(&new_bytes[21..25], &[15, 0, 0, 0]); // SourceString length = 15
        assert_eq!(&new_bytes[25..40], b"Complete Story\0");
    }
}

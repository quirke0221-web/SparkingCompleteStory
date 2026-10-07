use complete_story_cli::transform::character::{transform_character_ast, NEW_OBJECT_NAME, NEW_PACKAGE_NAME};
use complete_story_cli::transform::chart::transform_chart_ast;
use complete_story_cli::transform::registry::{transform_registry_ast, ROUTE_KEY};
use serde_json::{json, Value};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn get_staging_json(name: &str) -> Value {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let path = if root.join("build").join("staging").join("json").join(name).exists() {
        root.join("build").join("staging").join("json").join(name)
    } else {
        root.join("staging").join("json").join(name)
    };

    let file = File::open(&path).unwrap_or_else(|_| panic!("Failed to open staging JSON at {:?}", path));
    serde_json::from_reader(BufReader::new(file)).expect("Valid JSON expected")
}

#[test]
fn test_chart_transformation_zero_wrapping() {
    let mut chart_ast = get_staging_json("DragonAdventureIFChartData.json");
    let res = transform_chart_ast(&mut chart_ast);
    assert!(res.is_ok(), "Chart transformation failed: {:?}", res.err());

    let records = chart_ast
        .pointer("/Exports/0/Data/1/Value")
        .and_then(Value::as_array)
        .expect("Records array missing");

    // Must be exactly 9 records (8 official + Complete Story)
    assert_eq!(records.len(), 9, "Expected 9 records in chart data");

    // 9th entry must be a native 2-element JSON array [StructData, ObjectData]
    let entry_9 = &records[8];
    assert!(
        entry_9.is_array(),
        "9th chart entry must be a native JSON array, NOT an object wrapper!"
    );
    let entry_arr = entry_9.as_array().unwrap();
    assert_eq!(entry_arr.len(), 2, "9th chart entry must have 2 elements");

    // Verify it is NOT the PowerShell { value: [...], Count: 2 } wrapper bug
    assert!(
        entry_9.get("Count").is_none(),
        "Detected PowerShell 'Count' property bug!"
    );
    assert!(
        entry_9.get("value").is_none(),
        "Detected PowerShell 'value' property wrapper bug!"
    );

    // Verify key is ROUTE_KEY (9999_00)
    let key = entry_9
        .pointer("/0/Value/0/Value")
        .and_then(Value::as_str)
        .expect("Missing key in 9th entry");
    assert_eq!(key, ROUTE_KEY);
}

#[test]
fn test_registry_transformation_integrity() {
    let mut reg_ast = get_staging_json("DragonAdventureIFData.json");
    let res = transform_registry_ast(&mut reg_ast);
    assert!(res.is_ok(), "Registry transformation failed: {:?}", res.err());

    let records = reg_ast
        .pointer("/Exports/0/Data/0/Value")
        .and_then(Value::as_array)
        .expect("Records array missing");

    assert_eq!(records.len(), 9, "Expected 9 records in master registry");

    let imports = reg_ast
        .get("Imports")
        .and_then(Value::as_array)
        .expect("Imports array missing");

    assert_eq!(imports.len(), 29, "Expected 29 imports in master registry");

    // Verify package and object imports
    assert_eq!(imports[27]["ObjectName"], NEW_PACKAGE_NAME);
    assert_eq!(imports[28]["ObjectName"], NEW_OBJECT_NAME);

    // Verify DefaultOpenCharacter remains stock Goku (0000_40)
    let export_data = reg_ast
        .pointer("/Exports/0/Data")
        .and_then(Value::as_array)
        .expect("Export Data missing");
    let default_char_prop = export_data
        .iter()
        .find(|p| p.get("Name").and_then(Value::as_str) == Some("DefaultOpenCharacter"))
        .expect("DefaultOpenCharacter property missing");
    let default_char = default_char_prop
        .pointer("/Value/0/Value")
        .and_then(Value::as_str)
        .expect("DefaultOpenCharacter key missing");
    assert_eq!(default_char, "0000_40");
}

#[test]
fn test_character_transformation_name_payload_and_tail_preservation() {
    let mut char_ast = get_staging_json("DAIF_CharaData_0000_00.json");

    let original_b64 = char_ast
        .pointer("/Exports/0/Data")
        .and_then(Value::as_str)
        .expect("RawExport Data missing")
        .to_string();

    let res = transform_character_ast(&mut char_ast);
    assert!(res.is_ok(), "Character transformation failed: {:?}", res.err());

    let exports = char_ast.get("Exports").and_then(Value::as_array).unwrap();
    assert_eq!(exports[0]["ObjectName"], NEW_OBJECT_NAME);
    assert_eq!(exports[0]["SerialSize"], 354);

    // Verify mutated payload: CharacterName is "Complete Story", and tail is preserved
    let transformed_b64 = exports[0]["Data"].as_str().unwrap();
    assert_ne!(original_b64, transformed_b64, "Data payload must be updated with Complete Story name");

    let decoded = complete_story_cli::transform::character::base64_decode(transformed_b64)
        .expect("Base64 decode must succeed");
    assert_eq!(decoded.len(), 354);
    assert_eq!(&decoded[25..40], b"Complete Story\0");
}

#[test]
fn test_fail_hard_on_malformed_json_zero_mocks() {
    // Under no circumstance should a malformed AST silently fallback to synthetic/mock data
    let mut empty_ast = json!({});

    let res1 = transform_character_ast(&mut empty_ast);
    assert!(res1.is_err(), "Must fail hard on empty character AST");
    let res2 = transform_registry_ast(&mut empty_ast);
    assert!(res2.is_err(), "Must fail hard on empty registry AST");

    let res3 = transform_chart_ast(&mut empty_ast);
    assert!(res3.is_err(), "Must fail hard on empty chart AST");
}

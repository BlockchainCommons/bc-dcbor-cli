use anyhow::Result;

mod common;
use common::*;

#[test]
fn test_map_basic() -> Result<()> {
    run_cli_expect(
        &["map", "--out", "diag", "1", "2", "3", "4"],
        "{1: 2, 3: 4}"
    )?;
    Ok(())
}

#[test]
fn test_map_text_keys() -> Result<()> {
    run_cli_expect(
        &["map", "--out", "diag", r#""key1""#, r#""value1""#, r#""key2""#, r#""value2""#],
        r#"{"key1": "value1", "key2": "value2"}"#
    )?;
    Ok(())
}

#[test]
fn test_map_hex_output() -> Result<()> {
    run_cli_expect(
        &["map", "--out", "hex", "1", "2", "3", "4"],
        "a201020304"
    )?;
    Ok(())
}

#[test]
fn test_map_annotated_hex() -> Result<()> {
    run_cli_expect(
        &["map", "--out", "hex", "--annotate", "1", "2"],
        "a1      # map(1)\n    01  # unsigned(1)\n    02  # unsigned(2)"
    )?;
    Ok(())
}

#[test]
fn test_map_empty() -> Result<()> {
    run_cli_expect(
        &["map", "--out", "diag"],
        "{}"
    )?;
    run_cli_expect(
        &["map", "--out", "hex"],
        "a0"
    )?;
    Ok(())
}

#[test]
fn test_map_mixed_types() -> Result<()> {
    run_cli_expect(
        &["map", "--out", "diag", "1", r#""text""#, r#""key""#, "42"],
        r#"{1: "text", "key": 42}"#
    )?;
    Ok(())
}

#[test]
fn test_map_nested_values() -> Result<()> {
    run_cli_expect(
        &["map", "--out", "diag", "1", "[1, 2]", "2", r#"{3: 4}"#],
        r#"{1: [1, 2], 2: {3: 4}}"#
    )?;
    Ok(())
}

#[test]
fn test_map_odd_arguments() -> Result<()> {
    // Test that map with odd number of arguments fails
    let result = run_cli(&["map", "--out", "diag", "1", "2", "3"]);
    assert!(result.is_err());
    Ok(())
}

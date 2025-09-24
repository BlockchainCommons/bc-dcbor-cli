use anyhow::Result;

mod common;
use common::*;

#[test]
fn test_array_basic() -> Result<()> {
    run_cli_expect(&["array", "--out", "diag", "1", "2", "3"], "[1, 2, 3]")?;
    Ok(())
}

#[test]
fn test_array_mixed_types() -> Result<()> {
    run_cli_expect(
        &["array", "--out", "diag", "42", r#""hello""#, "true"],
        r#"[42, "hello", true]"#,
    )?;
    Ok(())
}

#[test]
fn test_array_hex_output() -> Result<()> {
    run_cli_expect(&["array", "--out", "hex", "1", "2", "3"], "83010203")?;
    Ok(())
}

#[test]
fn test_array_annotated_hex() -> Result<()> {
    run_cli_expect(
        &["array", "--out", "hex", "--annotate", "1", "2"],
        "82      # array(2)\n    01  # unsigned(1)\n    02  # unsigned(2)",
    )?;
    Ok(())
}

#[test]
fn test_array_empty() -> Result<()> {
    run_cli_expect(&["array", "--out", "diag"], "[]")?;
    run_cli_expect(&["array", "--out", "hex"], "80")?;
    Ok(())
}

#[test]
fn test_array_nested() -> Result<()> {
    run_cli_expect(
        &["array", "--out", "diag", "[1, 2]", "[3, 4]"],
        "[[1, 2], [3, 4]]",
    )?;
    Ok(())
}

#[test]
fn test_array_complex_elements() -> Result<()> {
    run_cli_expect(
        &["array", "--out", "diag", r#"{1: "a"}"#, r#"{2: "b"}"#],
        r#"[{1: "a"}, {2: "b"}]"#,
    )?;
    Ok(())
}

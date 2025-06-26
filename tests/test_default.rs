use anyhow::Result;

mod common;
use common::*;

#[test]
fn test_default_diag_to_hex() -> Result<()> {
    run_cli_expect(&["42"], "182a")?;
    run_cli_expect(&[r#""Hello""#], "6548656c6c6f")?;
    run_cli_expect(&["true"], "f5")?;
    run_cli_expect(&["false"], "f4")?;
    run_cli_expect(&["null"], "f6")?;
    Ok(())
}

#[test]
fn test_default_hex_to_diag() -> Result<()> {
    run_cli_expect(&["--in", "hex", "--out", "diag", "182a"], "42")?;
    run_cli_expect(
        &["--in", "hex", "--out", "diag", "6548656c6c6f"],
        r#""Hello""#,
    )?;
    run_cli_expect(&["--in", "hex", "--out", "diag", "f5"], "true")?;
    run_cli_expect(&["--in", "hex", "--out", "diag", "f4"], "false")?;
    run_cli_expect(&["--in", "hex", "--out", "diag", "f6"], "null")?;
    Ok(())
}

#[test]
fn test_annotations() -> Result<()> {
    run_cli_expect(
        &["--out", "hex", "--annotate", "42"],
        "182a    # unsigned(42)",
    )?;
    run_cli_expect(
        &["--in", "hex", "--out", "diag", "--annotate", "182a"],
        "42",
    )?;
    Ok(())
}

#[test]
fn test_complex_structures() -> Result<()> {
    // Test arrays
    run_cli_expect(&["[1, 2, 3]"], "83010203")?;
    run_cli_expect(&["--in", "hex", "--out", "diag", "83010203"], "[1, 2, 3]")?;

    // Test maps
    run_cli_expect(&[r#"{1: 2, 3: 4}"#], "a201020304")?;
    run_cli_expect(
        &["--in", "hex", "--out", "diag", "a201020304"],
        "{1: 2, 3: 4}",
    )?;

    Ok(())
}

#[test]
fn test_round_trip_conversions() -> Result<()> {
    let test_cases = [
        ("42", "182a"),
        (r#""Hello""#, "6548656c6c6f"),
        ("[1, 2, 3]", "83010203"),
        ("{1: 2}", "a10102"),
        ("true", "f5"),
        ("false", "f4"),
        ("null", "f6"),
    ];

    for (diag, hex) in test_cases {
        // diag -> hex
        run_cli_expect(&[diag], hex)?;
        // hex -> diag
        run_cli_expect(&["--in", "hex", "--out", "diag", hex], diag)?;
    }
    Ok(())
}

#[test]
fn test_tagged_values() -> Result<()> {
    // Date tags
    run_cli_expect(&["1(1747267200)"], "c11a68252e80")?;
    run_cli_expect(
        &["--in", "hex", "--out", "diag", "c11a68252e80"],
        "1(1747267200)",
    )?;

    // Custom tags
    run_cli_expect(&["40000(0)"], "d99c4000")?;
    run_cli_expect(&["--in", "hex", "--out", "diag", "d99c4000"], "40000(0)")?;

    Ok(())
}

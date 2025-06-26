use anyhow::Result;
use indoc::indoc;

mod common;
use common::{run_cli, run_cli_expect};

#[test]
fn test_match_simple_patterns() -> Result<()> {
    // Test basic value patterns
    run_cli_expect(&["match", "NUMBER", r#"42"#], "42")?;

    run_cli_expect(&["match", "TEXT", r#""hello""#], r#""hello""#)?;

    run_cli_expect(&["match", "BOOL", "true"], "true")?;

    Ok(())
}

#[test]
fn test_match_structure_patterns() -> Result<()> {
    run_cli_expect(
        &["match", "ARRAY(NUMBER > TEXT)", r#"[42, "hello"]"#],
        r#"[42, "hello"]"#,
    )?;

    run_cli_expect(
        &["match", "MAP(NUMBER(1): NUMBER)", r#"{1: 42, 2: "text"}"#],
        r#"{1: 42, 2: "text"}"#,
    )?;

    Ok(())
}

#[test]
fn test_match_search_patterns() -> Result<()> {
    let input = r#"{1: 42, 2: "text", 3: [1, 2, 3]}"#;

    #[rustfmt::skip]
    run_cli_expect(
        &["match", "SEARCH(NUMBER)", input],
        indoc! {r#"
            {1: 42, 2: "text", 3: [1, 2, 3]}
                1
            {1: 42, 2: "text", 3: [1, 2, 3]}
                42
            {1: 42, 2: "text", 3: [1, 2, 3]}
                2
            {1: 42, 2: "text", 3: [1, 2, 3]}
                3
            {1: 42, 2: "text", 3: [1, 2, 3]}
                [1, 2, 3]
                    1
            {1: 42, 2: "text", 3: [1, 2, 3]}
                [1, 2, 3]
                    2
            {1: 42, 2: "text", 3: [1, 2, 3]}
                [1, 2, 3]
                    3
        "#}.trim()
    )?;

    Ok(())
}

#[test]
fn test_match_captures() -> Result<()> {
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "@num(NUMBER)", "--captures", "42"],
        indoc! {r#"
            @num
                42
            42
        "#}.trim()
    )?;

    // Test multiple captures
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "@first(NUMBER) | @second(TEXT)", "--captures", "42"],
        indoc! {r#"
            @first
                42
            42
        "#}.trim()
    )?;

    #[rustfmt::skip]
    run_cli_expect(
        &["match", "@first(NUMBER) | @second(TEXT)", "--captures", r#""hello""#],
        indoc! {r#"
            @second
                "hello"
            "hello"
        "#}.trim()
    )?;

    Ok(())
}

#[test]
fn test_match_error_handling() -> Result<()> {
    // Test invalid pattern syntax
    let result = run_cli(&["match", "INVALID(", "42"]);
    assert!(result.is_err());

    // Test pattern that doesn't match
    let result = run_cli(&["match", "TEXT", "42"]);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_match_input_formats() -> Result<()> {
    // Test hex input
    run_cli_expect(&["match", "--in", "hex", "NUMBER", "182a"], "42")?;

    // Test diagnostic input (default)
    run_cli_expect(&["match", "NUMBER", "42"], "42")?;

    Ok(())
}

#[test]
fn test_match_output_formats() -> Result<()> {
    // Test hex output
    run_cli_expect(&["match", "--out", "hex", "NUMBER", "42"], "182a")?;

    // Test last-only option
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "--last-only", "SEARCH(NUMBER)", r#"[1, 2, 3]"#],
        indoc! {r#"
            1
            2
            3
        "#}.trim()
    )?;

    Ok(())
}

#[test]
fn test_match_array_patterns() -> Result<()> {
    // Test array with specific values
    run_cli_expect(
        &["match", "ARRAY(NUMBER(42) > TEXT)", r#"[42, "hello"]"#],
        r#"[42, "hello"]"#,
    )?;

    // Test array with captures
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "ARRAY(@first(NUMBER) > @second(TEXT))", "--captures", r#"[42, "hello"]"#],
        indoc! {r#"
            @first
                [42, "hello"]
                    42
            @second
                [42, "hello"]
                    "hello"
            [42, "hello"]
        "#}.trim()
    )?;

    Ok(())
}

#[test]
fn test_match_map_patterns() -> Result<()> {
    // Test map with specific keys
    run_cli_expect(
        &[
            "match",
            "MAP(TEXT(\"name\"): TEXT)",
            r#"{"name": "Alice", "age": 30}"#,
        ],
        r#"{"age": 30, "name": "Alice"}"#,
    )?;

    // Test map with captures
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "MAP(@key(TEXT(\"name\")): @value(TEXT))", "--captures", r#"{"name": "Alice"}"#],
        indoc! {r#"
            @key
                {"name": "Alice"}
                    "name"
            @value
                {"name": "Alice"}
                    "Alice"
            {"name": "Alice"}
        "#}.trim()
    )?;

    Ok(())
}

#[test]
fn test_match_complex_patterns() -> Result<()> {
    // Test nested structures
    let input =
        r#"{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}"#;

    #[rustfmt::skip]
    run_cli_expect(
        &["match", "SEARCH(MAP(TEXT(\"id\"): NUMBER))", input],
        indoc! {r#"
            {"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
                [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
                    {"id": 1, "name": "Alice"}
            {"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
                [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
                    {"id": 2, "name": "Bob"}
        "#}.trim()
    )?;

    Ok(())
}

#[test]
fn test_match_formatting_options() -> Result<()> {
    // Test no indentation
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "--no-indent", "SEARCH(NUMBER)", r#"[1, 2]"#],
        indoc! {r#"
            [1, 2]
            1
            [1, 2]
            2
        "#}.trim()
    )?;

    // Test last-only with captures
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "--last-only", "--captures", "SEARCH(@num(NUMBER))", r#"[1, 2]"#],
        indoc! {r#"
            @num
                1
                2
            1
            2
        "#}.trim()
    )?;

    Ok(())
}

#[test]
fn test_match_tagged_values() -> Result<()> {
    // Test tagged value patterns (note: tag 1 is timestamp, so we see a
    // formatted date)
    run_cli_expect(
        &["match", "TAG(1, NUMBER)", "1(42)"],
        "1970-01-01T00:00:42Z",
    )?;

    // Test tagged values with captures
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "TAG(1, @content(NUMBER))", "--captures", "1(42)"],
        indoc! {r#"
            @content
                1970-01-01T00:00:42Z
                    42
            1970-01-01T00:00:42Z
        "#}.trim()
    )?;

    Ok(())
}

#[test]
fn test_match_binary_input_output() -> Result<()> {
    // Test that binary input/output works correctly by round-tripping

    // First convert diag to hex
    let hex_result = run_cli(&["match", "--out", "hex", "NUMBER", "42"])?;
    assert_eq!(hex_result.trim(), "182a");

    // Then use that hex as input
    run_cli_expect(
        &["match", "--in", "hex", "--out", "diag", "NUMBER", "182a"],
        "42",
    )?;

    Ok(())
}

#[test]
fn test_match_edge_cases() -> Result<()> {
    // Test empty array
    run_cli_expect(&["match", "ARRAY", "[]"], "[]")?;

    // Test empty map
    run_cli_expect(&["match", "MAP", "{}"], "{}")?;

    // Test null value
    run_cli_expect(&["match", "NULL", "null"], "null")?;

    Ok(())
}

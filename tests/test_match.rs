use anyhow::Result;
use indoc::indoc;

mod common;
use common::{run_cli, run_cli_expect};

#[test]
fn test_match_simple_patterns() -> Result<()> {
    // Test basic value patterns
    run_cli_expect(&["match", "number", r#"42"#], "42")?;

    run_cli_expect(&["match", "text", r#""hello""#], r#""hello""#)?;

    run_cli_expect(&["match", "bool", "true"], "true")?;

    Ok(())
}

#[test]
fn test_match_structure_patterns() -> Result<()> {
    run_cli_expect(
        &["match", "[number, text]", r#"[42, "hello"]"#],
        r#"[42, "hello"]"#,
    )?;

    run_cli_expect(
        &["match", "{1: number}", r#"{1: 42, 2: "text"}"#],
        r#"{1: 42, 2: "text"}"#,
    )?;

    Ok(())
}

#[test]
fn test_match_search_patterns() -> Result<()> {
    let input = r#"{1: 42, 2: "text", 3: [1, 2, 3]}"#;

    #[rustfmt::skip]
    run_cli_expect(
        &["match", "search(number)", input],
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
        &["match", "@num(number)", "--captures", "42"],
        indoc! {r#"
            @num
                42
            42
        "#}.trim()
    )?;

    // Test multiple captures
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "@first(number) | @second(text)", "--captures", "42"],
        indoc! {r#"
            @first
                42
            42
        "#}.trim()
    )?;

    #[rustfmt::skip]
    run_cli_expect(
        &["match", "@first(number) | @second(text)", "--captures", r#""hello""#],
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
    let result = run_cli(&["match", "invalid(", "42"]);
    assert!(result.is_err());

    // Test pattern that doesn't match
    let result = run_cli(&["match", "text", "42"]);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_match_input_formats() -> Result<()> {
    // Test hex input
    run_cli_expect(&["match", "--in", "hex", "number", "182a"], "42")?;

    // Test diagnostic input (default)
    run_cli_expect(&["match", "number", "42"], "42")?;

    Ok(())
}

#[test]
fn test_match_output_formats() -> Result<()> {
    // Test hex output
    run_cli_expect(&["match", "--out", "hex", "number", "42"], "182a")?;

    // Test last-only option
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "--last-only", "search(number)", r#"[1, 2, 3]"#],
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
        &["match", "[42, text]", r#"[42, "hello"]"#],
        r#"[42, "hello"]"#,
    )?;

    // Test array with captures
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "[@first(number), @second(text)]", "--captures", r#"[42, "hello"]"#],
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
            r#"{"name": text}"#,
            r#"{"name": "Alice", "age": 30}"#,
        ],
        r#"{"age": 30, "name": "Alice"}"#,
    )?;

    // Test map with captures
    #[rustfmt::skip]
    run_cli_expect(
        &["match", r#"{@key("name"): @value(text)}"#, "--captures", r#"{"name": "Alice"}"#],
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
        &["match", r#"search({"id": number})"#, input],
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
        &["match", "--no-indent", "search(number)", r#"[1, 2]"#],
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
        &["match", "--last-only", "--captures", "search(@num(number))", r#"[1, 2]"#],
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
        &["match", "tagged(1, number)", "1(42)"],
        "1970-01-01T00:00:42Z",
    )?;

    // Test tagged values with captures
    #[rustfmt::skip]
    run_cli_expect(
        &["match", "tagged(1, @content(number))", "--captures", "1(42)"],
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
    let hex_result = run_cli(&["match", "--out", "hex", "number", "42"])?;
    assert_eq!(hex_result.trim(), "182a");

    // Then use that hex as input
    run_cli_expect(
        &["match", "--in", "hex", "--out", "diag", "number", "182a"],
        "42",
    )?;

    Ok(())
}

#[test]
fn test_match_edge_cases() -> Result<()> {
    // Test empty array
    run_cli_expect(&["match", "[*]", "[]"], "[]")?;

    // Test empty map
    run_cli_expect(&["match", "{*}", "{}"], "{}")?;

    // Test null value
    run_cli_expect(&["match", "null", "null"], "null")?;

    Ok(())
}

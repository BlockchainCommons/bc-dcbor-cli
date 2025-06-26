# dcbor-cli Crate Documentation

## Overview

This crate provides a command line interface for parsing, validating, and formatting Deterministic CBOR (dCBOR) data. It serves as a validation and diagnostic tool for dCBOR, supporting multiple input and output formats including CBOR diagnostic notation, hexadecimal, and binary.

The CLI tool is ready for community review, with complete functionality and comprehensive support for dCBOR validation and format conversion.

### Usage Example
```bash
# Convert CBOR diagnostic notation to hexadecimal
dcbor '"Hello, World!"'

# Validate and format with annotations
dcbor --in hex --out diag --annotate d99d6ca4...

# Convert between formats
dcbor --in hex --out bin input.hex > output.bin
```

## Development Guidelines

- Ensure `cargo test` and `cargo clippy` pass before committing
- Avoid `as_case` and `CBORCase` where possible - use the full `dcbor` API
- Use 4 spaces for indentation in formatted output (consistent with dCBOR diagnostic notation)
- Follow Rust CLI best practices for error handling and user experience
- Maintain compatibility with shell scripting and pipeline usage

## Architecture

### Input/Output Formats
- **Diagnostic Notation**: Human-readable CBOR diagnostic format
- **Hexadecimal**: Compact hex representation with optional annotations
- **Binary**: Raw CBOR binary data for file I/O operations

### Key Components
- CLI argument parsing with `clap`
- Format detection and conversion
- CBOR validation and error reporting
- Annotation generation for debugging

### Dependencies
- `dcbor`: Core deterministic CBOR implementation
- `clap`: Command line argument parsing
- `anyhow`: Error handling and reporting
- Standard library I/O for file operations

## Integration and Development Plan: `match` Subcommand

This section outlines the plan to integrate `dcbor-pattern` functionality into `dcbor-cli` via a new `match` subcommand, following the pattern established by `bc-envelope-cli`'s integration of `bc-envelope-pattern`.

### Current State Analysis

**bc-envelope-cli Integration Pattern:**
- Uses `bc-envelope-pattern` crate as a dependency
- Implements a `Match` command in `src/cmd/pattern.rs`
- Provides pattern matching against Gordian Envelope structures
- Returns paths through the envelope tree that match the pattern
- Supports multiple output formats: tree paths, digest URs, envelope URs, summaries
- Includes formatting options: indentation, last-element-only, annotations

**dcbor-cli Current Architecture:**
- Simple CLI with array/map subcommands and default processing
- Uses `clap` for argument parsing with a straightforward structure
- Single `main.rs` file handling all functionality
- No modular command structure like `bc-envelope-cli`

### Integration Goals

1. **Add Pattern Matching**: Enable users to match patterns against dCBOR data structures
2. **Follow Established Patterns**: Mirror the API and user experience of `bc-envelope-cli match`
3. **Maintain Compatibility**: Preserve existing dcbor-cli functionality and CLI interface
4. **Support dCBOR Patterns**: Leverage the full power of `dcbor-pattern` syntax

### Development Plan

#### Phase 1: Architecture Refactoring

**Objective**: Refactor dcbor-cli to support a modular command structure

**Tasks**:
1. **Create command module structure**:
    - Create `src/cmd/mod.rs` (similar to bc-envelope-cli)
    - Move existing array/map functionality to `src/cmd/array.rs` and `src/cmd/map.rs`
    - Create `src/cmd/default.rs` for the default behavior (parsing/validation)

2. **Refactor main.rs**:
    - Update `Commands` enum to include the new modular commands
    - Implement `Exec` trait pattern similar to bc-envelope-cli
    - Maintain backward compatibility with existing CLI usage

3. **Add dcbor-pattern dependency**:
    ```toml
    dcbor-pattern = "^0.1.0"
    ```

#### Phase 2: Testing Architecture Deployment

**Objective**: Deploy the comprehensive testing architecture on existing dcbor-cli functionality

**Tasks**:
1. **Create the testing infrastructure**:
   - Create `tests/` directory structure
   - Implement `tests/common/mod.rs` with testing utilities
   - Add testing dependencies to `Cargo.toml`

2. **Implement common testing utilities** (`tests/common/mod.rs`):
   ```rust
   use anyhow::{Result, bail};
   use assert_cmd::Command;

   pub fn run_cli_raw_stdin(args: &[&str], stdin: &str) -> Result<String> {
       let output = Command::cargo_bin("dcbor")
           .unwrap()
           .args(args)
           .write_stdin(stdin)
           .assert();

       if output.get_output().status.success() {
           Ok(String::from_utf8(output.get_output().stdout.to_vec()).unwrap())
       } else {
           bail!(
               "Command failed: {:?}",
               String::from_utf8(output.get_output().stderr.to_vec()).unwrap()
           );
       }
   }

   pub fn run_cli(args: &[&str]) -> Result<String> {
       run_cli_raw_stdin(args, "").map(|s| s.trim().to_string())
   }

   pub fn run_cli_expect(args: &[&str], expected: &str) -> Result<()> {
       let output = run_cli(args)?;
       if output != expected.trim() {
           bail!(
               "\n\n=== Expected ===\n{}\n\n=== Got ===\n{}",
               expected, output
           );
       }
       assert_eq!(expected.trim(), output);
       Ok(())
   }
   ```

3. **Create comprehensive tests for existing functionality**:

   **`tests/test_default.rs`** - Test default parsing/validation behavior:
   ```rust
   #[test]
   fn test_default_diag_to_hex() -> Result<()> {
       run_cli_expect(&["42"], "182a")?;
       run_cli_expect(&[r#""Hello""#], "6548656c6c6f")?;
       run_cli_expect(&["true"], "f5")?;
       Ok(())
   }

   #[test]
   fn test_default_hex_to_diag() -> Result<()> {
       run_cli_expect(&["--in", "hex", "--out", "diag", "182a"], "42")?;
       run_cli_expect(&["--in", "hex", "--out", "diag", "6548656c6c6f"], r#""Hello""#)?;
       Ok(())
   }

   #[test]
   fn test_annotations() -> Result<()> {
       run_cli_expect(
           &["--out", "hex", "--annotate", "42"],
           "18 2a                           # unsigned(42)"
       )?;
       Ok(())
   }
   ```

   **`tests/test_array.rs`** - Test array composition:
   ```rust
   #[test]
   fn test_array_basic() -> Result<()> {
       run_cli_expect(
           &["array", "--out", "diag", "1", "2", "3"],
           "[1, 2, 3]"
       )?;
       Ok(())
   }

   #[test]
   fn test_array_mixed_types() -> Result<()> {
       run_cli_expect(
           &["array", "--out", "diag", "42", r#""hello""#, "true"],
           r#"[42, "hello", true]"#
       )?;
       Ok(())
   }

   #[test]
   fn test_array_hex_output() -> Result<()> {
       run_cli_expect(
           &["array", "--out", "hex", "1", "2", "3"],
           "83010203"
       )?;
       Ok(())
   }
   ```

   **`tests/test_map.rs`** - Test map composition:
   ```rust
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
   ```

4. **Test complex scenarios and edge cases**:
   - Large dCBOR structures
   - Nested arrays and maps
   - Various input/output format combinations
   - Error conditions and invalid input
   - Stdin vs command line argument handling

5. **Validate round-trip testing**:
   ```rust
   #[test]
   fn test_round_trip_conversions() -> Result<()> {
       let test_cases = [
           ("42", "182a"),
           (r#""Hello""#, "6548656c6c6f"),
           ("[1, 2, 3]", "83010203"),
           ("{1: 2}", "a10102"),
       ];

       for (diag, hex) in test_cases {
           // diag -> hex
           run_cli_expect(&[diag], hex)?;
           // hex -> diag
           run_cli_expect(&["--in", "hex", "--out", "diag", hex], diag)?;
       }
       Ok(())
   }
   ```

6. **Performance and regression testing**:
   - Baseline performance measurements
   - Memory usage validation
   - Large file processing tests

**Benefits of this phase**:
- Establishes robust testing foundation before new features
- Validates existing functionality works correctly
- Provides testing patterns for the match command implementation
- Ensures no regressions during refactoring
- Creates comprehensive test coverage baseline

#### Phase 3: Match Command Implementation

**Objective**: Implement the core `match` subcommand functionality

**Tasks**:
1. **Create `src/cmd/match.rs`**:
    - Implement `CommandArgs` struct with pattern string and formatting options
    - Support input from stdin or command line argument
    - Handle pattern parsing errors with user-friendly messages
    - Implement pattern matching against dCBOR data

2. **Command line interface**:
    ```bash
    dcbor match [OPTIONS] <PATTERN> [INPUT]

    Arguments:
      <PATTERN>     The pattern to match against
      [INPUT]       dCBOR input (hex, diag, or binary)

    Options:
      --in <FORMAT>         Input format [default: diag] [possible: diag, hex, bin]
      --out <FORMAT>        Output format [default: paths] [possible: paths, hex, diag]
      --no-indent          Disable indentation of path elements
      --last-only          Show only the last element of each path
      --annotate           Add annotations to output
      --captures           Include capture information in output
    ```

3. **Pattern matching logic**:
    - Parse dCBOR input using existing dcbor-cli parsing logic
    - Parse pattern using `dcbor_pattern::Pattern::parse()`
    - Execute matching using `pattern.paths_with_captures()`
    - Format output using `dcbor_pattern::format_paths_with_captures()`

#### Phase 4: Output Formatting

**Objective**: Implement comprehensive output formatting options

**Tasks**:
1. **Path formatting**:
    - Default: Show full paths with 4-space indentation
    - `--last-only`: Show only the final elements that matched
    - `--no-indent`: Flat output without indentation
    - `--captures`: Include named captures in output

2. **Data format options**:
    - `--out paths`: Default path format with dCBOR diagnostic notation
    - `--out hex`: Output matching elements as hexadecimal
    - `--out diag`: Output matching elements as diagnostic notation
    - `--annotate`: Add comments/annotations to output

3. **Error handling**:
    - Clear error messages for invalid patterns
    - Helpful context for parsing failures
    - Position indicators for syntax errors

#### Phase 5: Match Command Testing

**Objective**: Implement comprehensive testing for the new match command functionality

**Tasks**:

1. **Create `tests/test_match.rs`** - Comprehensive match command tests:

2. **Test categories for match command**:

    **Basic Pattern Matching Tests**:
    ```rust
    #[test]
    fn test_match_simple_patterns() -> Result<()> {
        // Test basic value patterns
        run_cli_expect(
            &["match", "NUMBER", r#"42"#],
            "42"
        )?;

        run_cli_expect(
            &["match", "TEXT", r#""hello""#],
            r#""hello""#
        )?;

        run_cli_expect(
            &["match", "BOOL", "true"],
            "true"
        )?;
        Ok(())
    }
    ```

    **Structure Pattern Tests**:
    ```rust
    #[test]
    fn test_match_structure_patterns() -> Result<()> {
        run_cli_expect(
            &["match", "ARRAY(NUMBER, TEXT)", r#"[42, "hello"]"#],
            r#"[42, "hello"]"#
        )?;

        run_cli_expect(
            &["match", "MAP(1 > NUMBER)", r#"{1: 42, 2: "text"}"#],
            r#"{1: 42, 2: "text"}"#
        )?;
        Ok(())
    }
    ```

    **Search and Complex Pattern Tests**:
    ```rust
    #[test]
    fn test_match_search_patterns() -> Result<()> {
        let input = r#"{1: 42, 2: "text", 3: [1, 2, 3]}"#;

        #[rustfmt::skip]
        run_cli_expect(
            &["match", "SEARCH(NUMBER)", input],
            indoc! {r#"
                {1: 42, 2: "text", 3: [1, 2, 3]}
                        42
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
    ```

    **Capture and Formatting Tests**:
    ```rust
    #[test]
    fn test_match_captures() -> Result<()> {
        #[rustfmt::skip]
        run_cli_expect(
            &["match", "@num(NUMBER)", "--captures", "42"],
            indoc! {r#"
                @num
                        42
            "#}.trim()
        )?;
        Ok(())
    }
    ```

    **Error Handling Tests**:
    ```rust
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
    ```

    **Input Format Tests**:
    ```rust
    #[test]
    fn test_match_input_formats() -> Result<()> {
        // Test hex input
        run_cli_expect(
            &["match", "--in", "hex", "NUMBER", "182a"],
            "42"
        )?;

        // Test diagnostic input (default)
        run_cli_expect(
            &["match", "NUMBER", "42"],
            "42"
        )?;
        Ok(())
    }
    ```

    **Output Format Tests**:
    ```rust
    #[test]
    fn test_match_output_formats() -> Result<()> {
        // Test hex output
        run_cli_expect(
            &["match", "--out", "hex", "NUMBER", "42"],
            "182a"
        )?;

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
    ```

5. **Integration and pipeline tests**:
    ```rust
    #[test]
    fn test_match_pipeline_integration() -> Result<()> {
        // Test piping match results to other commands
        #[rustfmt::skip]
        run_cli_piped_expect(&[
            &["match", "--out", "hex", "--last-only", "SEARCH(NUMBER)", r#"[1, 2, 3]"#],
            &["--in", "hex", "--out", "diag"]
        ], indoc! {r#"
            1
            2
            3
        "#}.trim())?;
        Ok(())
    }
    ```

7. **Performance and edge case tests**:
    - Large dCBOR structures
    - Complex nested patterns
    - Memory usage validation
    - Pattern compilation performance

8. **Integration with existing test suite**:
    - Ensure match command tests integrate with existing test infrastructure
    - Validate backward compatibility with existing functionality
    - Test command chaining and pipeline integration with match command

### Implementation Details

#### Pattern Syntax Support

The match command will support the full `dcbor-pattern` syntax including:

- **Value patterns**: `NUMBER`, `TEXT`, `BOOL`, `NULL`, `BYTES`
- **Structure patterns**: `ARRAY`, `MAP`, `TAGGED`
- **Meta patterns**: `AND`, `OR`, `NOT`, `SEARCH`, sequence matching
- **Captures**: `@name(pattern)` for named captures
- **Quantifiers**: `?`, `*`, `+`, `{n,m}` for repetition
- **Specific values**: Exact matches using dCBOR diagnostic notation

#### Usage Examples

```bash
# Find all numbers in a dCBOR structure
dcbor match 'SEARCH(NUMBER)' '{1: 42, 2: "text", 3: [1, 2, 3]}'

# Match specific array structure with captures
dcbor match '@values(ARRAY(NUMBER, TEXT))' '[42, "hello"]' --captures

# Find tagged values
dcbor match 'SEARCH(TAGGED)' --in hex d99d6ca401...

# Complex pattern with sequence matching
dcbor match 'MAP(1 > @num(NUMBER), 2 > @text(TEXT))' '{1: 42, 2: "hello"}'
```

#### Error Handling Strategy

1. **Pattern parsing errors**: Clear position-based error messages
2. **Input format errors**: Leverage existing dcbor-cli error handling
3. **No matches**: Return appropriate exit code and message
4. **Invalid combinations**: Validate flag combinations

#### Backward Compatibility

- Existing `dcbor` command usage remains unchanged
- New `match` subcommand is additive
- All existing tests continue to pass
- No breaking changes to existing API

### Success Criteria

1. ✅ **Functional**: `dcbor match` command works for all dcbor-pattern syntax
2. ✅ **Compatible**: Existing dcbor-cli functionality unchanged
3. ✅ **Consistent**: API follows bc-envelope-cli patterns
4. ✅ **Tested**: Comprehensive test coverage for new functionality
5. ✅ **Documented**: Clear documentation and examples
6. ✅ **Robust**: Good error handling and user experience

This integration will provide dcbor-cli users with powerful pattern matching capabilities for analyzing and extracting data from dCBOR structures, following the proven design patterns established by bc-envelope-cli.

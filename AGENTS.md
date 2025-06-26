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

#### Phase 1: Architecture Refactoring ✅ COMPLETED

**Objective**: Refactor dcbor-cli to support a modular command structure

**Completed Tasks**:
1. ✅ **Created command module structure**:
    - Created `src/cmd/mod.rs` with `Exec` trait pattern
    - Moved existing array/map functionality to `src/cmd/array.rs` and `src/cmd/map.rs`
    - Created `src/cmd/default.rs` for the default behavior (parsing/validation)

2. ✅ **Refactored main.rs**:
    - Updated `Commands` enum to use modular commands
    - Implemented `Exec` trait pattern similar to bc-envelope-cli
    - Maintained backward compatibility with existing CLI usage
    - Added `format_output` utility function
    - Added `hex` dependency for binary output handling

3. ✅ **Dependencies ready**:
    - `dcbor-pattern = "^0.1.0"` already present
    - Added `hex = "^0.4.0"` for binary format support

#### Phase 2: Testing Architecture Deployment ✅ COMPLETED

**Objective**: Deploy the comprehensive testing architecture on existing dcbor-cli functionality

**Completed Tasks**:
1. ✅ **Created the testing infrastructure**:
   - Created `tests/` directory structure
   - Implemented `tests/common/mod.rs` with testing utilities
   - Added testing dependencies to `Cargo.toml`: `assert_cmd = "^2.0.0"`

2. ✅ **Implemented common testing utilities** (`tests/common/mod.rs`):
   - `run_cli_raw_stdin()` - Execute CLI with stdin input
   - `run_cli()` - Execute CLI with trimmed output
   - `run_cli_expect()` - Execute CLI and assert expected output

3. ✅ **Created comprehensive tests for existing functionality**:

   **`tests/test_default.rs`** - Default parsing/validation behavior (6 tests):
   - Basic diag-to-hex and hex-to-diag conversions
   - Annotation support
   - Complex structures (arrays, maps, nested)
   - Round-trip conversions
   - Tagged values support

   **`tests/test_array.rs`** - Array composition (7 tests):
   - Basic array creation
   - Mixed types, nested arrays, complex elements
   - Hex output, annotated hex output
   - Empty arrays

   **`tests/test_map.rs`** - Map composition (8 tests):
   - Basic map creation with numeric and text keys
   - Mixed types, nested values
   - Hex output, annotated hex output
   - Empty maps, error handling for odd arguments

4. ✅ **Testing Results**:
   - **Total: 24 tests** (3 legacy + 21 new comprehensive tests)
   - **All tests passing** ✅
   - Comprehensive coverage of existing functionality
   - Robust testing foundation established

**Benefits achieved**:
- ✅ Robust testing foundation before new features
- ✅ Validated existing functionality works correctly
- ✅ Established testing patterns for match command implementation
- ✅ Ensured no regressions during refactoring
- ✅ Created comprehensive test coverage baseline

#### Phase 3: Match Command Implementation 🚧 NEXT

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

### Current Status Summary

✅ **Phase 1 & 2 Complete**: Successfully refactored dcbor-cli architecture and deployed comprehensive testing infrastructure.

**Architectural Achievements**:
- ✅ Modular command structure with `Exec` trait pattern
- ✅ Maintained full backward compatibility
- ✅ 24 comprehensive tests covering all existing functionality
- ✅ Ready for match command integration

**Next Implementation Phase**: Phase 3 - Match Command Implementation

**Ready to implement**:
1. Create `src/cmd/match.rs` with pattern matching functionality
2. Add Match variant to Commands enum
3. Implement comprehensive match command tests
4. Support full dcbor-pattern syntax

This integration will follow the proven design patterns established by bc-envelope-cli and provide dcbor-cli users with powerful pattern matching capabilities for analyzing and extracting data from dCBOR structures.

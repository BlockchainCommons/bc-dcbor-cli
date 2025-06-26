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

#### Phase 3: Match Command Implementation ✅ COMPLETED

**Objective**: Implement the core `match` subcommand functionality

**Completed Tasks**:
1. ✅ **Created `src/cmd/match.rs`**:
    - Implemented `CommandArgs` struct with pattern string and formatting options
    - Support input from stdin or command line argument
    - Handle pattern parsing errors with user-friendly messages
    - Implement pattern matching against dCBOR data

2. ✅ **Command line interface**:
    ```bash
    dcbor match [OPTIONS] <PATTERN> [INPUT]

    Arguments:
      <PATTERN>     The pattern to match against
      [INPUT]       dCBOR input (hex, diag, or binary)

    Options:
      --in <FORMAT>         Input format [default: diag] [possible: diag, hex, bin]
      --out <FORMAT>        Output format [default: paths] [possible: paths, hex, diag, bin]
      --no-indent          Disable indentation of path elements
      --last-only          Show only the last element of each path
      --annotate           Add annotations to output
      --captures           Include capture information in output
    ```

3. ✅ **Pattern matching logic**:
    - Parse dCBOR input using existing dcbor-cli parsing logic
    - Parse pattern using `dcbor_pattern::Pattern::parse()`
    - Execute matching using `pattern.paths_with_captures()`
    - Format output using `dcbor_pattern::format_paths_with_captures()`

4. ✅ **Comprehensive test coverage**:
    - Created `tests/test_match.rs` with 14 comprehensive tests
    - All tests passing: pattern matching, captures, input/output formats, error handling
    - Total test count: 38 tests (3 legacy + 21 existing + 14 new match tests)

#### Phase 4: Output Formatting ✅ COMPLETED

**Objective**: Implement comprehensive output formatting options

**Completed Tasks**:
1. ✅ **Path formatting**:
    - Default: Show full paths with 4-space indentation
    - `--last-only`: Show only the final elements that matched
    - `--no-indent`: Flat output without indentation
    - `--captures`: Include named captures in output

2. ✅ **Data format options**:
    - `--out paths`: Default path format with dCBOR diagnostic notation
    - `--out hex`: Output matching elements as hexadecimal
    - `--out diag`: Output matching elements as diagnostic notation
    - `--out bin`: Output matching elements as raw binary
    - `--annotate`: Add comments/annotations to output

3. ✅ **Error handling**:
    - Clear error messages for invalid patterns
    - Helpful context for parsing failures
    - Position indicators for syntax errors

#### Phase 5: Match Command Testing ✅ COMPLETED

**Objective**: Implement comprehensive testing for the new match command functionality

**Completed Tasks**:

1. ✅ **Created `tests/test_match.rs`** - Comprehensive match command tests with 14 test functions

2. ✅ **Test categories for match command**:

    **Basic Pattern Matching Tests**: ✅
    - Simple value patterns (NUMBER, TEXT, BOOL)
    - Structure patterns (ARRAY, MAP with proper syntax)
    - All tests passing with correct dcbor-pattern syntax

    **Search and Complex Pattern Tests**: ✅
    - SEARCH patterns with multiple matches
    - Complex nested structures
    - Proper path ordering and formatting

    **Capture and Formatting Tests**: ✅
    - Named captures with @name syntax
    - Multiple captures and nested captures
    - Proper alphabetical sorting of capture output

    **Error Handling Tests**: ✅
    - Invalid pattern syntax detection
    - Pattern that doesn't match scenarios
    - Clear error message validation

    **Input Format Tests**: ✅
    - Hex input processing
    - Diagnostic input (default)
    - Binary input handling

    **Output Format Tests**: ✅
    - Hex output formatting
    - Last-only option functionality
    - All format combinations working correctly

3. ✅ **Integration and pipeline tests**:
    - Round-trip hex to diag conversions
    - Format switching validation
    - Binary input/output processing

4. ✅ **Performance and edge case tests**:
    - Empty arrays and maps
    - Null values
    - Tagged value handling (including timestamp formatting)

5. ✅ **Integration with existing test suite**:
    - All 38 tests passing (3 legacy + 21 existing + 14 new match tests)
    - Backward compatibility maintained
    - No regressions in existing functionality

### Implementation Details

#### Pattern Syntax Support

The match command supports the full `dcbor-pattern` syntax including:

- **Value patterns**: `NUMBER`, `TEXT`, `BOOL`, `NULL`, `BYTES`
- **Structure patterns**: `ARRAY`, `MAP`, `TAG`
- **Meta patterns**: `AND`, `OR`, `NOT`, `SEARCH`, sequence matching
- **Captures**: `@name(pattern)` for named captures
- **Quantifiers**: `?`, `*`, `+`, `{n,m}` for repetition
- **Specific values**: Exact matches using dCBOR diagnostic notation

#### Usage Examples

```bash
# Find all numbers in a dCBOR structure
dcbor match 'SEARCH(NUMBER)' '{1: 42, 2: "text", 3: [1, 2, 3]}'

# Match specific array structure with captures
dcbor match 'ARRAY(@first(NUMBER) > @second(TEXT))' '[42, "hello"]' --captures

# Find tagged values
dcbor match 'TAG(1, NUMBER)' '1(42)'
dcbor match 'TAG(1, NUMBER)' '1970-01-01T00:00:42Z'
dcbor match 'DATE' '1970-01-01T00:00:42Z'

# Complex pattern with map matching
dcbor match 'MAP(TEXT("name"): @value(TEXT))' '{"name": "Alice"}' --captures
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

✅ **Phases 1-5 Complete**: Successfully implemented dcbor-pattern integration into dcbor-cli.

**Implementation Achievements**:
- ✅ **Phase 1**: Modular command structure with `Exec` trait pattern
- ✅ **Phase 2**: Comprehensive testing infrastructure (24 tests for existing functionality)
- ✅ **Phase 3**: Match command implementation with full dcbor-pattern support
- ✅ **Phase 4**: Complete output formatting options (paths, hex, diag, bin)
- ✅ **Phase 5**: Comprehensive match command testing (14 new tests)

**Final Results**:
- ✅ **Total Tests**: 38 tests (3 legacy + 21 existing + 14 new match tests)
- ✅ **Full Pattern Support**: All dcbor-pattern syntax working
- ✅ **Complete API**: Input/output formats, captures, formatting options
- ✅ **Robust Error Handling**: Clear, position-based error messages
- ✅ **Backward Compatibility**: No regressions in existing functionality
1. Create `src/cmd/match.rs` with pattern matching functionality
2. Add Match variant to Commands enum
3. Implement comprehensive match command tests
4. Support full dcbor-pattern syntax

This integration will follow the proven design patterns established by bc-envelope-cli and provide dcbor-cli users with powerful pattern matching capabilities for analyzing and extracting data from dCBOR structures.

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

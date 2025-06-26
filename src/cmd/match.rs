use std::collections::HashMap;

use anyhow::{Result, bail};
use clap::{Args, ValueEnum};
use dcbor::prelude::*;
use dcbor_pattern::{
    FormatPathsOpts, Matcher, Pattern, format_paths_with_captures,
};

use crate::{InputFormat, OutputFormat, cmd::Exec, format_output, read_data};

/// Match dCBOR data against a pattern.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The pattern to match against.
    pattern: String,

    /// dCBOR input (hex, diag, or binary). If not provided, reads from stdin.
    input: Option<String>,

    /// Input format.
    #[arg(long, value_enum, default_value = "diag")]
    r#in: InputFormat,

    /// Output format.
    #[arg(long, value_enum, default_value = "paths")]
    out: MatchOutputFormat,

    /// Disable indentation of path elements.
    #[arg(long)]
    no_indent: bool,

    /// Show only the last element of each path.
    #[arg(long)]
    last_only: bool,

    /// Add annotations to output.
    #[arg(long)]
    annotate: bool,

    /// Include capture information in output.
    #[arg(long)]
    captures: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[doc(hidden)]
enum MatchOutputFormat {
    /// Show matching paths (default)
    Paths,
    /// CBOR diagnostic notation
    Diag,
    /// Hexadecimal
    Hex,
    /// Raw binary
    Bin,
}

impl Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        // Read input data
        let input_data = if let Some(input) = &self.input {
            input.as_bytes().to_vec()
        } else {
            read_data(&mut std::io::stdin())?
        };

        // Parse input based on format
        let cbor = match self.r#in {
            InputFormat::Diag => {
                let input_str = String::from_utf8(input_data)?;
                dcbor_parse::parse_dcbor_item(input_str.trim())?
            }
            InputFormat::Hex => {
                let input_str = String::from_utf8(input_data)?;
                let hex_str = input_str.trim();
                let bytes = hex::decode(hex_str)?;
                CBOR::try_from_data(&bytes)?
            }
            InputFormat::Bin => CBOR::try_from_data(&input_data)?,
        };

        // Parse pattern
        let pattern = Pattern::parse(&self.pattern)
            .map_err(|e| {
                match e {
                    dcbor_pattern::Error::UnrecognizedToken(span) => {
                        let input = &self.pattern;
                        let start = span.start.min(input.len());
                        let end = span.end.min(input.len());
                        let error_text = if start < input.len() {
                            &input[start..end]
                        } else {
                            "<end of input>"
                        };
                        anyhow::anyhow!(
                            "Failed to parse pattern at position {}..{}: unrecognized token '{}'\nPattern: {}\n         {}^",
                            start, end, error_text, input,
                            " ".repeat(start)
                        )
                    }
                    dcbor_pattern::Error::ExtraData(span) => {
                        let input = &self.pattern;
                        let start = span.start.min(input.len());
                        anyhow::anyhow!(
                            "Failed to parse pattern: extra data at position {}\nPattern: {}\n         {}^",
                            start, input, " ".repeat(start)
                        )
                    }
                    dcbor_pattern::Error::UnexpectedToken(token, span) => {
                        let input = &self.pattern;
                        let start = span.start.min(input.len());
                        anyhow::anyhow!(
                            "Failed to parse pattern at position {}: unexpected token {:?}\nPattern: {}\n         {}^",
                            start, token, input, " ".repeat(start)
                        )
                    }
                    _ => anyhow::anyhow!("Failed to parse pattern: {}", e),
                }
            })?;

        // Execute pattern matching
        let (paths, captures) = pattern.paths_with_captures(&cbor);

        // Check for matches
        if paths.is_empty() {
            bail!("No match");
        }

        // Format output based on requested format
        match self.out {
            MatchOutputFormat::Paths => {
                // Build format options from command line arguments
                let format_options = FormatPathsOpts::new()
                    .indent(!self.no_indent)
                    .last_element_only(self.last_only);

                // Show captures only if explicitly requested
                if self.captures {
                    Ok(format_paths_with_captures(
                        &paths,
                        &captures,
                        format_options,
                    ))
                } else {
                    // Show paths without captures
                    Ok(format_paths_with_captures(
                        &paths,
                        &HashMap::new(),
                        format_options,
                    ))
                }
            }
            MatchOutputFormat::Diag
            | MatchOutputFormat::Hex
            | MatchOutputFormat::Bin => {
                // For data format outputs, extract the matched elements
                let output_format = match self.out {
                    MatchOutputFormat::Diag => OutputFormat::Diag,
                    MatchOutputFormat::Hex => OutputFormat::Hex,
                    MatchOutputFormat::Bin => OutputFormat::Bin,
                    _ => unreachable!(),
                };

                let elements_to_output = if self.last_only {
                    // Get just the last element of each path
                    paths
                        .iter()
                        .filter_map(|path| path.last())
                        .collect::<Vec<_>>()
                } else {
                    // Get all matching elements (typically the root elements)
                    paths
                        .iter()
                        .filter_map(|path| path.first())
                        .collect::<Vec<_>>()
                };

                let mut results = Vec::new();
                for element in elements_to_output {
                    results.push(format_output(
                        element,
                        output_format,
                        self.annotate,
                    )?);
                }

                Ok(results.join("\n"))
            }
        }
    }
}

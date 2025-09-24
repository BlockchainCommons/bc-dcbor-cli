//! A command line tool for composing, parsing and validating Gordian dCBOR. See the main repo [README](https://github.com/BlockchainCommons/bc-dcbor-cli/blob/master/README.md).

use std::{
    ffi::OsString,
    io::{self, Read, Write},
};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use dcbor::prelude::*;

mod cmd;
use cmd::Exec;

#[derive(Subcommand)]
enum Commands {
    /// Compose a dCBOR array from the provided elements
    Array(cmd::array::CommandArgs),
    /// Compose a dCBOR map from the provided keys and values
    Map(cmd::map::CommandArgs),
    /// Match dCBOR data against a pattern
    Match(cmd::r#match::CommandArgs),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[doc(hidden)]
struct Cli {
    /// Subcommands for custom operations
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    default_args: cmd::default::CommandArgs,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[doc(hidden)]
enum InputFormat {
    /// CBOR diagnostic notation
    Diag,
    /// Hexadecimal
    Hex,
    /// Raw binary
    Bin,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[doc(hidden)]
enum OutputFormat {
    /// CBOR diagnostic notation
    Diag,
    /// Hexadecimal
    Hex,
    /// Raw binary
    Bin,
    /// No output: merely succeeds on validation of input
    None,
}

#[doc(hidden)]
fn format_output(
    cbor: &CBOR,
    out_format: OutputFormat,
    annotate: bool,
) -> Result<String> {
    match out_format {
        OutputFormat::Diag => {
            if annotate {
                Ok(cbor.diagnostic_annotated())
            } else {
                Ok(cbor.diagnostic_flat())
            }
        }
        OutputFormat::Hex => {
            if annotate {
                Ok(cbor.hex_annotated())
            } else {
                Ok(cbor.hex())
            }
        }
        OutputFormat::Bin => {
            // For binary output, we'll return the hex representation
            // and let the caller handle writing binary data
            Ok(hex::encode(cbor.to_cbor_data()))
        }
        OutputFormat::None => Ok(String::new()),
    }
}

#[doc(hidden)]
fn read_data<R>(reader: &mut R) -> Result<Vec<u8>>
where
    R: Read,
{
    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

#[doc(hidden)]
fn read_string<R>(reader: &mut R) -> Result<String>
where
    R: Read,
{
    let mut result = String::new();
    reader.read_to_string(&mut result)?;
    Ok(result)
}

#[doc(hidden)]
fn run<I, T, R, W>(args: I, reader: &mut R, writer: &mut W) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    R: Read,
    W: Write,
{
    bc_components::register_tags();

    let cli = Cli::parse_from(args);

    let output = if let Some(command) = cli.command {
        match command {
            Commands::Array(args) => args.exec()?,
            Commands::Map(args) => args.exec()?,
            Commands::Match(args) => args.exec()?,
        }
    } else {
        cli.default_args.exec_with_reader(reader)?
    };

    if cli.default_args.out == OutputFormat::Bin {
        // For binary output, decode hex back to bytes
        let data = hex::decode(&output)?;
        writer.write_all(&data)?;
    } else if !output.is_empty() {
        writer.write_all(format!("{}\n", output).as_bytes())?;
    }

    Ok(())
}

#[doc(hidden)]
fn main() -> Result<()> {
    run(std::env::args_os(), &mut io::stdin(), &mut io::stdout())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use indoc::indoc;

    use crate::run;

    fn run_expect(options: &[&str], input: &str, expect: &str) {
        let mut all_args = vec!["dcbor"];
        all_args.extend(options.iter());
        if !input.is_empty() {
            all_args.push(input);
        }
        let mut output: Vec<u8> = Vec::new();
        let input: Vec<u8> = Vec::new();
        let mut input_cursor = Cursor::new(input);
        run(all_args, &mut input_cursor, &mut output).unwrap();
        let raw_output_string = String::from_utf8(output).unwrap();
        let output_string = raw_output_string.trim();
        if expect != output_string {
            println!(
                "=== Expected ===\n{}\n\n=== Got ===\n{}",
                expect, output_string
            );
        }
        assert_eq!(expect, output_string);
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse() {
        let diag_to_hex: &[&str] = &["--"]; // Signal end of options so `-Infinity` below is not treated as an option
        let hex_to_diag: &[&str] = &["--in", "hex", "--out", "diag"];
        let hex_to_diag_annotate: &[&str] = &["--in", "hex", "--out", "diag", "--annotate"];
        let diag_to_hex_annotate: &[&str] = &["--out", "hex", "--annotate"];

        let round_trip_diag_hex = |diag: &str, hex: &str, ret_diag: Option<&str>| {
            run_expect(diag_to_hex, diag, hex);
            run_expect(hex_to_diag, hex, ret_diag.unwrap_or(diag));
        };
        round_trip_diag_hex(
            r#"0"#,
            "00",
            None
        );
        round_trip_diag_hex(
            r#"42"#,
            "182a",
            None
        );
        round_trip_diag_hex(
            r#"3.14"#,
            "fb40091eb851eb851f",
            None
        );
        round_trip_diag_hex(
            r#"40000(0)"#,
            "d99c4000",
            None
        );
        round_trip_diag_hex(
            r#"true"#,
            "f5",
            None
        );
        round_trip_diag_hex(
            r#"false"#,
            "f4",
            None
        );
        round_trip_diag_hex(
            r#"null"#,
            "f6",
            None
        );
        round_trip_diag_hex(
            r#"Infinity"#,
            "f97c00",
            None
        );
        round_trip_diag_hex(
            r#"-Infinity"#,
            "f9fc00",
            None
        );
        round_trip_diag_hex(
            r#"NaN"#,
            "f97e00",
            None
        );
        round_trip_diag_hex(
            r#""Hello, world!""#,
            "6d48656c6c6f2c20776f726c6421",
            None
        );
        round_trip_diag_hex(
            r#"h'0102030405060708090a'"#,
            "4a0102030405060708090a",
            None
        );
        round_trip_diag_hex(
            r#"b64'AQIDBAUGBwgJCg=='"#,
            "4a0102030405060708090a",
            Some("h'0102030405060708090a'")
        );
        round_trip_diag_hex(
            r#"[1, 2, 3]"#,
            "83010203",
            None
        );
        round_trip_diag_hex(
            r#"[true, false, null]"#,
            "83f5f4f6",
            None
        );
        round_trip_diag_hex(
            r#"{1: "value1", 2: "value2", 3: "value3"}"#,
            "a3016676616c756531026676616c756532036676616c756533",
            None
        );
        round_trip_diag_hex(
            r#"{"key1": h'0102', "key2": "value2", "key3": {1: "value1", 2: "value2", 3: "value3"}}"#,
            "a3646b657931420102646b6579326676616c756532646b657933a3016676616c756531026676616c756532036676616c756533",
            None
        );
        round_trip_diag_hex(
            r#"ur:date/cyisdadmlasgtapttl"#,
            "c11a68252e80",
            Some("1(1747267200)")
        );
        round_trip_diag_hex(
            r#"date(1747267200)"#,
            "c11a68252e80",
            Some("1(1747267200)")
        );
        round_trip_diag_hex(
            r#"40000(0)"#,
            "d99c4000",
            None
        );
        round_trip_diag_hex(
            r#"'0'"#,
            "d99c4000",
            Some("40000(0)")
        );
        round_trip_diag_hex(
            r#"''"#,
            "d99c4000",
            Some("40000(0)")
        );
        round_trip_diag_hex(
            r#"'isA'"#,
            "d99c4001",
            Some("40000(1)")
        );

        let seed_hex = "d99d6ca4015059f2293a5bce7d4de59e71b4207ac5d202c11a6035970003754461726b20507572706c652041717561204c6f766504787b4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e";
        let seed_diag = r#"40300({1: h'59f2293a5bce7d4de59e71b4207ac5d2', 2: 1(1614124800), 3: "Dark Purple Aqua Love", 4: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."})"#;
        round_trip_diag_hex(seed_diag, seed_hex, None);

        let seed_diag_annotate = indoc! {r#"
            40300(   / seed /
                {
                    1:
                    h'59f2293a5bce7d4de59e71b4207ac5d2',
                    2:
                    1(1614124800),   / date /
                    3:
                    "Dark Purple Aqua Love",
                    4:
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                }
            )
        "#}.trim();
        run_expect(
            hex_to_diag_annotate,
            seed_hex,
            seed_diag_annotate
        );

        let seed_hex_annotate = indoc! {r#"
            d9 9d6c                                 # tag(40300) seed
                a4                                  # map(4)
                    01                              # unsigned(1)
                    50                              # bytes(16)
                        59f2293a5bce7d4de59e71b4207ac5d2
                    02                              # unsigned(2)
                    c1                              # tag(1) date
                        1a60359700                  # unsigned(1614124800)
                    03                              # unsigned(3)
                    75                              # text(21)
                        4461726b20507572706c652041717561204c6f7665 # "Dark Purple Aqua Love"
                    04                              # unsigned(4)
                    78 7b                           # text(123)
                        4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e # "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
        "#}.trim();
        run_expect(
            diag_to_hex_annotate,
            seed_diag,
            seed_hex_annotate
        );
    }

    #[test]
    fn test_compose_array() {
        let expected = r#"[1, 2, 3]"#;
        run_expect(&["array", "--out", "diag", "1", "2", "3"], "", expected);
    }

    #[test]
    fn test_compose_map() {
        let expected = r#"{1: 2, 3: 4}"#;
        run_expect(&["map", "--out", "diag", "1", "2", "3", "4"], "", expected);
    }
}

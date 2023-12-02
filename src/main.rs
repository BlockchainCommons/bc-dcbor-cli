use std::{io::{self, Read, Write, BufRead, BufReader}, ffi::OsString, error::Error};

use clap::{Parser, ValueEnum};
use dcbor::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input dCBOR as hexadecimal. If not provided here or input format is binary, input is read from STDIN
    hex: Option<String>,

    /// The input format
    #[arg(short, long, value_enum, default_value_t = InputFormat::Hex)]
    r#in: InputFormat,

    /// The output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Diag)]
    out: OutputFormat,

    /// Output diagnostic notation or hexadecimal in compact form. Ignored for other output formats
    #[arg(short, long, default_value_t = false)]
    compact: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum InputFormat {
    /// Hexadecimal
    Hex,
    /// Raw binary
    Bin,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
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

fn read_data<R>(reader: &mut R) -> Result<Vec<u8>, io::Error> where R: Read {
    let mut buf = vec!();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

fn read_string<R>(reader: &mut R) -> Result<String, io::Error> where R: Read {
    let mut reader = BufReader::new(reader);
    let mut result = String::new();
    reader.read_line(&mut result)?;
    Ok(result)
}

fn run<I, T, R, W>(args: I, reader: &mut R, writer: &mut W) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    R: Read,
    W: Write
{
    let mut known_tags = TagsStore::new([]);
    known_tags.insert(Tag::new_with_name(1, "date"));

    let cli = Cli::parse_from(args);

    let cbor: CBOR = match (cli.r#in, cli.hex) {
        (InputFormat::Hex, Some(hex)) => {
            CBOR::from_hex(&hex)?
        },
        (InputFormat::Hex, None) => {
            let string = read_string(reader)?;
            let hex = string.trim();
            CBOR::from_hex(hex)?
        },
        (InputFormat::Bin, _) => {
            let data = read_data(reader)?;
            CBOR::from_data(&data)?
        },
    };

    match cli.out {
        OutputFormat::Diag => {
            if cli.compact {
                writer.write_all(format!("{}\n", cbor).as_bytes())?;
            } else {
                writer.write_all(format!("{}\n", cbor.diagnostic_opt(true, Some(&known_tags))).as_bytes())?;
            }
        },
        OutputFormat::Hex => {
            writer.write_all(format!("{}\n", cbor.hex_opt(!cli.compact, Some(&known_tags))).as_bytes())?;
        },
        OutputFormat::Bin => {
            writer.write_all(&cbor.cbor_data())?;
        },
        OutputFormat::None => {},
    };

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run(std::env::args_os(), &mut io::stdin(), &mut io::stdout())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use crate::run;
    use indoc::indoc;

    fn test_diag(args: &[&str], diag: &str) {
        let mut all_args = vec!["dcbor"];
        all_args.extend(args.iter());
        let mut output: Vec<u8> = Vec::new();
        let input: Vec<u8> = Vec::new();
        let mut input_cursor = Cursor::new(input);
        run(all_args, &mut input_cursor, &mut output).unwrap();
        let output_string = String::from_utf8(output).unwrap();
        assert_eq!(diag, output_string.trim())
    }

    fn test_hex_diag(hex: &str, diag: &str) {
        test_diag(&[hex], diag)
    }

    #[test]
    fn test1() {
        test_hex_diag("00", "0");
        let hex = "d9012ca4015059f2293a5bce7d4de59e71b4207ac5d202c11a6035970003754461726b20507572706c652041717561204c6f766504787b4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e";
        let expected = indoc! {r#"
        300(
           {
              1:
              h'59f2293a5bce7d4de59e71b4207ac5d2',
              2:
              1(2021-02-24T00:00:00Z),   / date /
              3:
              "Dark Purple Aqua Love",
              4:
              "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
           }
        )
        "#}.trim();
        test_hex_diag(hex, expected);
    }
}

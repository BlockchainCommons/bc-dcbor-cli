use anyhow::Result;
use clap::Args;
use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item;
use std::io::Read;

use crate::{InputFormat, OutputFormat, format_output, read_data, read_string};

/// Default parsing and validation behavior
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Input dCBOR in the format specified by `--in`. If not provided here or
    /// input format is binary, input is read from STDIN
    pub input: Option<String>,

    /// The input format
    #[arg(short, long, value_enum, default_value_t = InputFormat::Diag)]
    pub r#in: InputFormat,

    /// The output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Hex)]
    pub out: OutputFormat,

    /// Output diagnostic notation or hexadecimal with annotations. Ignored for
    /// other output formats
    #[arg(short, long)]
    pub annotate: bool,
}

impl CommandArgs {
    pub fn exec_with_reader<R>(&self, reader: &mut R) -> Result<String>
    where
        R: Read,
    {
        let cbor: CBOR = match (self.r#in, &self.input) {
            (InputFormat::Diag, Some(diag)) => parse_dcbor_item(diag)
                .map_err(|e| anyhow::anyhow!("{}", e.full_message(diag)))?,
            (InputFormat::Diag, None) => {
                let diag = read_string(reader)?;
                parse_dcbor_item(&diag)
                    .map_err(|e| anyhow::anyhow!("{}", e.full_message(&diag)))?
            }
            (InputFormat::Hex, Some(hex)) => CBOR::try_from_hex(hex)?,
            (InputFormat::Hex, None) => {
                let string = read_string(reader)?;
                let hex = string.trim();
                CBOR::try_from_hex(hex)?
            }
            (InputFormat::Bin, _) => {
                let data = read_data(reader)?;
                CBOR::try_from_data(data)?
            }
        };

        format_output(&cbor, self.out, self.annotate)
    }
}

impl super::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        use std::io;
        self.exec_with_reader(&mut io::stdin())
    }
}

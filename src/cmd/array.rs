use anyhow::Result;
use clap::Args;
use dcbor_parse::compose_dcbor_array;

use crate::{OutputFormat, format_output};

/// Compose a dCBOR array from the provided elements
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// Each of the elements is parsed as a dCBOR item in diagnostic
    /// notation and added to the output dCBOR array.
    pub elements: Vec<String>,

    /// The output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Hex)]
    pub out: OutputFormat,

    /// Output diagnostic notation or hexadecimal with annotations. Ignored for
    /// other output formats
    #[arg(short, long)]
    pub annotate: bool,
}

impl super::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let element_refs: Vec<&str> =
            self.elements.iter().map(|s| s.as_str()).collect();
        let cbor = compose_dcbor_array(&element_refs)?;
        format_output(&cbor, self.out, self.annotate)
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventParseError {
    #[error("Unable to find header column `{0}` in Pronto export")]
    MissingHeaderColumn(&'static str),

    #[error(
        "Encountered an unexpected end of row for a record. One common reason for this is a \
         driver that did not attend the event but remains in Pronto.\n'{row}'"
    )]
    UnexpectedEndOfRow { row: String },

    #[error("Failed to deserialize row {row} due to {source:?}")]
    DeserializeRow { row: String, source: csv::Error },

    #[error("Failed to parse MSR driver; raw row was:\n\t{row}")]
    MsrDeserialize { row: String, source: csv::Error },

    #[error("Failed to match Pronto driver {pronto_id} in MSR data")]
    MsrDriverMissing { pronto_id: String },

    #[error("Invalid PAX multiplier `{raw}`")]
    InvalidPaxMultiplier {
        raw: String,
        source: bigdecimal::ParseBigDecimalError,
    },
}

#[derive(Debug, Error)]
pub enum ChampionshipParseError {
    #[error("Unrecognized file format ({file_name}). Expected .xls or .xlsx")]
    UnsupportedExtension { file_name: String },

    #[error("Unable to find sheet with name dissimilar to 'calculations'")]
    NoUsableSheet,

    #[error("File {file_name} contains only empty sheets")]
    AllSheetsEmpty { file_name: String },

    #[error("Unable to find header row in championship XLS")]
    MissingHeaderRow,

    #[error("Missing `{0}` column in championship XLS header")]
    MissingColumn(&'static str),

    #[error("{cell_description} for {context} championship input XLS")]
    MissingSheetCell {
        cell_description: &'static str,
        context: &'static str,
    },

    #[error("{cell_description} for {context} championship input XLS")]
    InvalidCellContent {
        cell_description: &'static str,
        context: &'static str,
    },
}

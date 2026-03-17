use clap::{ArgAction, Parser};

// `derive(Parser)` tells clap to generate command-line parsing code
// from this struct definition.
#[derive(Parser, Debug)]
#[command(
    name = "minigenome",
    version,
    about = "Reduce a reference genome to genic regions and remap annotation coordinates"
)]
pub struct Args {
    // `PathBuf` is Rust's owned path type.
    // Use it when you want to store file paths safely instead of plain strings.
    #[arg(short = 'f', long = "fasta")]
    pub fasta: PathBuf,

    #[arg(short = 'a', long = "annotation")]
    pub annotation: PathBuf,

    // `default_value_t = 0` means the user can skip this flag and padding
    // will default to zero bases.
    #[arg(short = 'p', long = "padding", default_value_t = 0)]
    pub padding: u64,

    #[arg(short = 'o', long = "out-prefix")]
    pub out_prefix: String,

    // Users can repeat `--region chr1:100-200` to force extra intervals
    // into the reduced genome even if they are not annotated genes.
    #[arg(short = 'r', long = "region", value_name = "CHR:START-END", action = ArgAction::Append)]
    pub regions: Vec<String>,

    // A plain text file with one `chr:start-end` region per line.
    #[arg(long = "regions-file", value_name = "PATH")]
    pub regions_file: Option<PathBuf>,
}

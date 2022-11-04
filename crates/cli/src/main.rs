use std::{ffi::OsStr, fs::File, io::Read, path::PathBuf};

use ccc_core::Ccc;
use clap::{arg, Parser};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut ccc = Ccc::new();

    let mut f = File::open(&cli.file)?;
    let ext = cli.file.extension().and_then(OsStr::to_str).unwrap();

    let mut buf = vec![];
    f.read_to_end(&mut buf)?;

    println!(
        "{:?}",
        ccc.parse(&buf, ext.to_string().try_into()?)
            .map(|t| t.root_node().to_sexp())
    );

    Ok(())
}

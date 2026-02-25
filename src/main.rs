use clap::Parser;
use std::{
	fs::File,
	io::{self, Read, Write},
	path::PathBuf,
};
use walkdir::WalkDir;

mod util;
#[allow(clippy::wildcard_imports, reason = "")]
use util::*;

/// <https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/limits.h.html#:~:text=32-,%7B%5FPOSIX%5FNAME%5FMAX>
const _POSIX_NAME_MAX: u8 = 14;
/// <https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/limits.h.html#:~:text=20-,%7B%5FPOSIX%5FPATH%5FMAX>
const _POSIX_PATH_MAX: u16 = 256;
const POSIX_PATH_MAX_NO_NULL: u8 = (_POSIX_PATH_MAX - 1) as _;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	/// Maximum length of path component
	// ASK: `14` is annoying, should it be `32`?
	#[arg(short, long, value_name = "LENGTH", default_value = _POSIX_NAME_MAX)]
	len: Option<u8>,
	/// Allow infix white-spaces (leading/trailing still forbidden)
	#[arg(short, long)]
	space: Option<bool>,
	/// Exit on first invalid path. Implies `inr`
	#[arg(short, long, value_name = "FAIL-FAST")]
	ff: Option<bool>,
	/// Don't descend into invalid directories
	#[arg(short, long, value_name = "INVAL-NO-RECURSE")]
	inr: Option<bool>,
	/// [default: .]
	paths: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli = Cli::parse();

	// TO-DO: use `out` for warns only
	let mut out = io::stdout().lock();
	let mut err = io::stderr().lock();

	let mut p = cli.paths;
	if p.is_empty() {
		p.push(".".into());
	}
	let paths = p;

	for path in paths {
		for rde in WalkDir::new(&path)
			.max_open(8) // power of 2 that's closest to `10` (last known default)
			.follow_links(true)
		// not using `filter_map`, to avoid `stderr` lifetime issues
		{
			let de = match rde {
				Ok(de) => de,
				Err(e) => {
					write!(&mut err, "{e}")?;
					err.flush()?;
					continue;
				}
			};
			let mut f = if de.file_type().is_file() {
			} else {
				continue;
			};
		}
		write!(out, "{}\n{}", path.display())?;
		out.flush()?;
	}

	Ok(())
}

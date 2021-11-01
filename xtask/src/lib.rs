//! Codegen tools mostly used to generate ast and syntax definitions. Adapted from rust analyzer's codegen

pub mod codegen;
pub mod coverage;
pub mod docgen;
pub mod glue;

use std::{
	env,
	path::{Path, PathBuf},
};

use crate::{
	codegen::Mode,
	glue::{pushd, pushenv},
};

pub use anyhow::{bail, Context as _, Result};

pub fn project_root() -> PathBuf {
	Path::new(
		&env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
	)
	.ancestors()
	.nth(1)
	.unwrap()
	.to_path_buf()
}

pub fn run_rustfmt(mode: Mode) -> Result<()> {
	let _dir = pushd(project_root());
	let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
	ensure_rustfmt()?;
	match mode {
		Mode::Overwrite => run!("cargo fmt"),
		Mode::Verify => run!("cargo fmt -- --check"),
	}?;
	Ok(())
}

fn reformat(text: impl std::fmt::Display) -> Result<String> {
	let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
	ensure_rustfmt()?;
	let stdout = run!(
		"rustfmt --config fn_single_line=true";
		<text.to_string().as_bytes()
	)?;
	let preamble = "Generated file, do not edit by hand, see `xtask/src/codegen`";
	Ok(format!("//! {}\n\n{}\n", preamble, stdout))
}

fn ensure_rustfmt() -> Result<()> {
	let out = run!("rustfmt --version")?;
	if !out.contains("stable") {
		bail!(
			"Failed to run rustfmt from toolchain 'stable'. \
             Please run `rustup component add rustfmt --toolchain stable` to install it.",
		)
	}
	Ok(())
}

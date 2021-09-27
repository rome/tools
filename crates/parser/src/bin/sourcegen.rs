use std::path::PathBuf;

use anyhow::{anyhow, Result};
use rome_parser::sourcegen::Grammar;

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();
	let lang = args.get(1).ok_or_else(|| anyhow!("Missing language"))?;

	let node_types = match lang.as_str() {
		"tsx" => tree_sitter_typescript::TSX_NODE_TYPES,
		"ts" => tree_sitter_typescript::TYPESCRIPT_NODE_TYPES,
		lang => return Err(anyhow!("Language not found: {}", lang)),
	};

	let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR")?.into();

	let path = manifest_dir
		.as_path()
		.join("src")
		.join("languages")
		.join(lang)
		.with_extension("rs");

	let grammar = Grammar::from_node_types(node_types, lang)?;
	let syntax_kinds = grammar.create_syntax_kinds()?;

	std::fs::write(path, syntax_kinds)?;
	Ok(())
}

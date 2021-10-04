use std::path::PathBuf;

use anyhow::Result;
use syntax_codegen::{
	generate_nodes_from_grammars, generate_tokens_from_grammars, syntax_kind::create_syntax_kinds,
	Grammar,
};
use tree_sitter_typescript::{
	language_tsx, language_typescript, TSX_NODE_TYPES, TYPESCRIPT_NODE_TYPES,
};

fn main() -> Result<()> {
	let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR")?.into();

	let workspace = manifest_dir.as_path().parent().unwrap();
	let syntax_generated = workspace
		.join("syntax")
		.join("src")
		.join("ast")
		.join("generated");

	let nodes_path = syntax_generated.join("nodes.rs");

	// let tokens_path = syntax_generated.join("tokens.rs");

	let syntax_kind_path = workspace
		.join("parser")
		.join("src")
		.join("syntax_kind")
		.join("generated.rs");

	let tsx = Grammar::new(TSX_NODE_TYPES, "tsx", language_tsx())?;
	let ts = Grammar::new(TYPESCRIPT_NODE_TYPES, "ts", language_typescript())?;

	let grammars = vec![tsx, ts];

	let syntax_kinds = create_syntax_kinds(&grammars)?;
	let nodes = generate_nodes_from_grammars(&grammars);
	let _tokens = generate_tokens_from_grammars(&grammars);

	std::fs::write(nodes_path, nodes)?;
	// std::fs::write(tokens_path, tokens)?;
	std::fs::write(syntax_kind_path, syntax_kinds)?;
	Ok(())
}

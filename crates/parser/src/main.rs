use anyhow::{anyhow, Result};
use rome_parser::{languages, Parser, ParserLanguage};

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();

	let lang = args.get(1).ok_or_else(|| anyhow!("Missing language"))?;
	let path = args.get(2).ok_or_else(|| anyhow!("Missing filename"))?;

	let src = std::fs::read_to_string(path)?;

	match lang.as_str() {
		"tsx" => print_tree(&src, languages::TSX),
		"ts" => print_tree(&src, languages::TS),
		lang => return Err(anyhow!("Invalid language: {}", lang)),
	}
}

fn print_tree(src: &str, syntax: impl ParserLanguage) -> Result<()> {
	let mut parser = Parser::new(syntax)?;
	let tree = parser.parse_text(src)?;
	println!("{:#?}", tree);
	Ok(())
}

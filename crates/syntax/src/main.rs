use anyhow::{anyhow, Result};

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();

	let lang = args.get(1).ok_or_else(|| anyhow!("Missing language"))?;
	let path = args.get(2).ok_or_else(|| anyhow!("Missing filename"))?;

	let language = match lang.as_str() {
		"ts" => syntax::Language::Ts,
		"tsx" => syntax::Language::Tsx,
		lang => return Err(anyhow!("Invalid language: {}", lang)),
	};

	let src = std::fs::read_to_string(path)?;
	let tree = syntax::parse(&src, language)?;

	dbg!(tree);
	Ok(())
}

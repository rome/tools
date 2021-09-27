use anyhow::{anyhow, Result};

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();

	let path = args.get(1).ok_or_else(|| anyhow!("Missing filename"))?;

	let src = std::fs::read_to_string(path)?;
	let tree = syntax::parse(&src)?;

	dbg!(tree);
	Ok(())
}

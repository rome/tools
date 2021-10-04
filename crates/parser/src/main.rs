use anyhow::{anyhow, Result};
use parser::{Language, TreeSink};

struct TestPrinter {
	depth: usize,
}

impl TestPrinter {
	fn new() -> Self {
		Self { depth: 0 }
	}
}

impl TreeSink for TestPrinter {
	fn token(&mut self, kind: parser::SyntaxKind, text: &str) {
		println!("{}{:?} {:?}", "  ".repeat(self.depth), kind, text)
	}

	fn start_node(&mut self, kind: parser::SyntaxKind) {
		println!("{}{:?}", "  ".repeat(self.depth), kind);
		self.depth += 1;
	}

	fn finish_node(&mut self) {
		debug_assert!(self.depth > 0);
		self.depth -= 1;
	}
}

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();

	let path = args.get(1).ok_or_else(|| anyhow!("Missing filename"))?;
	let src = std::fs::read_to_string(path)?;

	let mut printer = TestPrinter::new();
	parser::parse(&src, &mut printer, Language::Tsx)?;
	Ok(())
}

use pico_args::Arguments;
use xtask::{project_root, pushd, Mode, Result};

use xtask_codegen::{generate_ast, generate_formatter, generate_parser_tests, generate_tables};

fn main() -> Result<()> {
    let _d = pushd(project_root());

    let mut args = Arguments::from_env();
    let command = args.subcommand()?.unwrap_or_default();

    match command.as_str() {
        "grammar" => {
            generate_ast(Mode::Overwrite)?;
            Ok(())
        }
        "formatter" => {
            generate_formatter();
            Ok(())
        }
        "test" => {
            generate_parser_tests(Mode::Overwrite)?;
            Ok(())
        }
        "unicode" => {
            generate_tables()?;
            Ok(())
        }
        _ => {
            eprintln!(
                "\
cargo codegen
Run codegen command.
USAGE:
	cargo codegen <SUBCOMMAND> [option]
SUBCOMMANDS:
	grammar      Transforms js.ungram into AST
	test         Extracts parser inline comments into test files
	unicode      Generates unicode table inside lexer
			"
            );
            Ok(())
        }
    }
}

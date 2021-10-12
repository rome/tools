use clap::{App, Arg};
use xtask::{
	codegen::{self, Mode},
	coverage,
	glue::pushd,
	project_root, run_rustfmt, Result,
};

fn main() -> Result<()> {
	let _d = pushd(project_root());

	let commands = App::new("xtask")
		.about("Run custom build command.")
		.subcommand(App::new("codegen"))
		.subcommand(App::new("syntax"))
		.subcommand(App::new("format"))
		.subcommand(App::new("docgen"))
		.subcommand(
			App::new("coverage").arg(
				Arg::new("coverage_query")
					.about("Query parameter for coverage")
					.index(1)
					.required(true)
					.takes_value(true),
			),
		)
		.get_matches();

	match commands.subcommand() {
		Some(("codegen", _)) => {
			codegen::generate_parser_tests(Mode::Overwrite).ok();
			Ok(())
		}
		Some(("syntax", _)) => {
			codegen::generate_syntax(Mode::Overwrite).ok();
			Ok(())
		}
		Some(("format", _)) => {
			run_rustfmt(Mode::Overwrite).ok();
			Ok(())
		}
		Some(("docgen", _)) => {
			// docgen::run();
			Ok(())
		}
		Some(("coverage", coverage_matches)) => {
			let query = coverage_matches.value_of("coverage_query");
			let pool = yastl::ThreadConfig::new().stack_size(8 << 30);
			coverage::run(query, yastl::Pool::with_config(num_cpus::get(), pool));
			Ok(())
		}
		_ => Ok(()),
	}
}

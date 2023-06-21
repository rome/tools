_default:
  just --list -u

# Installs the tools needed to develop with Rome
install-tools:
	cargo install binstall
	cargo binstall cargo-insta cargo-nextest wasm-pack

# Upgrades the tools needed to develop with Rome
upgrade-tools:
	cargo install binstall --force
	cargo binstall cargo-insta cargo-nextest wasm-pack --force

# Generate all files across crates and tools. You rarely want to use it locally.
codegen:
  cargo codegen all
  cargo codegen-configuration
  just codegen-bindings

# Generates TypeScript types and JSON schema of the configuration
codegen-bindings:
  cargo codegen-schema
  cargo codegen-bindings

# Generates code generated files for the linter
codegen-linter:
  cargo codegen analyzer
  cargo codegen-configuration
  just codegen-bindings
  cargo lintdoc

# Generates the linter documentation and Rust documentation
documentation:
  cargo lintdoc
  cargo documentation

# Creates a new lint rule in the given path, with the given name. Name has to be camel case.
new-lintrule path rulename:
  cargo run -p xtask_codegen -- newlintrule --path={{path}} --name={{rulename}}
  just codegen-linter
  just documentation

# Promotes a rule from the nursery group to a new group
promote-rule rulename group:
	cargo run -p xtask_codegen -- promoterule --rule={{rulename}} --group={{group}}
	just codegen-linter
	just documentation
	-cargo test -p rome_js_analyze -- {{snakecase(rulename)}}
	cargo insta accept

[unix]
_touch file:
  touch {{file}}

[windows]
_touch file:
  (gci {{file}}).LastWriteTime = Get-Date

# Tests a lint rule. The name of the rule needs to be camel case
test-lintrule name:
  just _touch crates/rome_js_analyze/tests/spec_tests.rs
  just _touch crates/rome_json_analyze/tests/spec_tests.rs
  cargo test -p rome_js_analyze -- {{snakecase(name)}}
  cargo test -p rome_json_analyze -- {{snakecase(name)}}

# When you finished coding, run this command to run the same commands in the CI.
check-ready:
  git diff --exit-code --quiet
  just codegen
  just documentation
  cargo lint
  cargo fmt
  cargo test
  git diff --exit-code --quiet

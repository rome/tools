_default:
  just --list -u

codegen:
  cargo codegen all
  cargo codegen-configuration
  just codegen-bindings

codegen-bindings:
  cargo codegen-schema
  cargo codegen-bindings

# Generates code generated files for the linter
codegen-linter:
  cargo codegen analyzer
  cargo codegen-configuration
  just codegen-bindings
  cargo lintdoc

# Generates the documentation
documentation:
  cargo lintdoc
  cargo documentation

# Creates a new lint rule in the given path, with the given name. Name has to be camel case.
new-lintrule path name:
  cargo run -p xtask_codegen -- newlintrule --path={{path}} --name={{name}}
  just codegen-linter
  just documentation

# Promotes a rule from the nursery group to a new group
promote-rule rule group:
	cargo run -p xtask_codegen -- promoterule --rule={{rule}} --group={{group}}
	just codegen-linter
	just documentation
	-cargo test -p rome_js_analyze -- {{snakecase(rule)}}
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
  cargo test -p rome_js_analyze -- {{snakecase(name)}}

check-ready:
  git diff --exit-code --quiet
  just codegen
  just documentation
  cargo lint
  cargo fmt
  cargo test
  git diff --exit-code --quiet

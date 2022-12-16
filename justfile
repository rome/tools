_default:
  just --list -u

codegen:
  cargo codegen all
  cargo codegen-configuration
  cargo codegen-schema
  cargo codegen-bindings

documentation:
  cargo lintdoc
  cargo documentation

new-lintrule path name:
  cargo run -p xtask_codegen -- newlintrule --path={{path}} --name={{name}}
  just codegen
  just documentation

[unix]
_touch file:
  touch {{file}}

[windows]
_touch file:
  (gci {{file}}).LastWriteTime = Get-Date

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

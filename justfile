default:
  just --list -u
  
codegen:
  cargo codegen all
  cargo codegen-configuration
  cargo codegen-schema
  cargo codegen-bindings
  cargo codegen-aria
  
documentation:
  cargo lintdoc
  cargo documentation

new-lintrule path name:
  cargo run -p xtask_codegen -- newlintrule --path={{path}} --name={{name}}
  just codegen
  just documentation

test-lintrule name:
  cargo test -p rome_js_analyze -- {{snakecase(name)}}

check-ready:
  git diff --exit-code
  just codegen
  just documentation
  cargo lint
  cargo fmt
  cargo test
  git diff --exit-code

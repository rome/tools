default:
  just --list
  
codegen:
    cargo codegen all
    cargo codegen-configuration
    cargo codegen-schema
    cargo codegen-bindings
    cargo codegen-aria

new-lintrule path name:
    cargo run -p xtask_codegen -- newlintrule --path={{path}} --name={{name}}
    cargo codegen analyzer
    cargo codegen-configuration
    cargo lintdoc
    cargo documentation

check-lintrule name:
  cargo test -p rome_js_analyze -- {{snakecase(name)}}
  cargo codegen analyzer
  cargo codegen-configuration
  cargo lintdoc
  cargo documentation

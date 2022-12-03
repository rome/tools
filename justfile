default:
  just --list
  
codegen:
    cargo codegen all
    cargo codegen-configuration
    cargo codegen-schema
    cargo codegen-bindings
    cargo codegen-aria

newlintrule path name:
    cargo run -p xtask_codegen -- newlintrule --path={{path}} --name={{name}}
    #crates/rome_diagnostics_categories/src/categories.rs
    cargo codegen analyzer
    cargo lintdoc

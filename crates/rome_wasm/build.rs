use std::{env, fs, io, path::PathBuf};

use quote::{format_ident, quote};

use rome_js_factory::syntax::SourceType;
use rome_js_factory::{
    make,
    syntax::{JsAnyDeclaration, JsAnyModuleItem, JsAnyStatement},
};
use rome_js_formatter::{context::JsFormatOptions, format_node};
use rome_rowan::AstNode;
use rome_service::workspace_types::{generate_type, methods, ModuleQueue};

fn main() -> io::Result<()> {
    let methods = methods();

    let mut items = Vec::new();
    let mut queue = ModuleQueue::default();

    for method in &methods {
        generate_type(&mut items, &mut queue, &method.params);
        generate_type(&mut items, &mut queue, &method.result);
    }

    let module = make::js_module(
        make::js_directive_list(None),
        make::js_module_item_list(items.into_iter().map(|(decl, _)| {
            JsAnyModuleItem::JsAnyStatement(match decl {
                JsAnyDeclaration::JsClassDeclaration(decl) => {
                    JsAnyStatement::JsClassDeclaration(decl)
                }
                JsAnyDeclaration::JsFunctionDeclaration(decl) => {
                    JsAnyStatement::JsFunctionDeclaration(decl)
                }
                JsAnyDeclaration::JsVariableDeclaration(decl) => {
                    JsAnyStatement::JsVariableStatement(make::js_variable_statement(decl).build())
                }
                JsAnyDeclaration::TsDeclareFunctionDeclaration(decl) => {
                    JsAnyStatement::TsDeclareFunctionDeclaration(decl)
                }
                JsAnyDeclaration::TsEnumDeclaration(decl) => {
                    JsAnyStatement::TsEnumDeclaration(decl)
                }
                JsAnyDeclaration::TsExternalModuleDeclaration(decl) => {
                    JsAnyStatement::TsExternalModuleDeclaration(decl)
                }
                JsAnyDeclaration::TsGlobalDeclaration(decl) => {
                    JsAnyStatement::TsGlobalDeclaration(decl)
                }
                JsAnyDeclaration::TsImportEqualsDeclaration(decl) => {
                    JsAnyStatement::TsImportEqualsDeclaration(decl)
                }
                JsAnyDeclaration::TsInterfaceDeclaration(decl) => {
                    JsAnyStatement::TsInterfaceDeclaration(decl)
                }
                JsAnyDeclaration::TsModuleDeclaration(decl) => {
                    JsAnyStatement::TsModuleDeclaration(decl)
                }
                JsAnyDeclaration::TsTypeAliasDeclaration(decl) => {
                    JsAnyStatement::TsTypeAliasDeclaration(decl)
                }
            })
        })),
        make::eof(),
    )
    .build();

    // Wasm-bindgen will paste the generated TS code as-is into the final .d.ts file,
    // ensure it looks good by running it through the formatter
    let formatted = format_node(JsFormatOptions::new(SourceType::ts()), module.syntax()).unwrap();
    let printed = formatted.print().unwrap();
    let definitions = printed.into_code();

    // Generate wasm-bindgen extern type imports for all the types defined in the TS code
    let types = queue.visited().iter().map(|name| {
        let ident = format_ident!("I{name}");
        quote! {
            #[wasm_bindgen(typescript_type = #name)]
            #[allow(non_camel_case_types)]
            pub type #ident;
        }
    });

    let tokens = quote! {
        #[wasm_bindgen(typescript_custom_section)]
        const TS_TYPEDEFS: &'static str = #definitions;

        #[wasm_bindgen]
        extern "C" {
            #( #types )*
        }
    };

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(
        PathBuf::from(out_dir).join("ts_types.rs"),
        tokens.to_string(),
    )?;

    Ok(())
}

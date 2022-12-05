//! Metadata of:
//! - ARIA properties
//! - ARIA property types
//! - ARIA roles

use case::CaseExt;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use std::path::PathBuf;
use std::{env, fs, io};

pub const ARIA_PROPERTIES: [&str; 48] = [
    "aria-activedescendant",
    "aria-atomic",
    "aria-autocomplete",
    "aria-busy",
    "aria-checked",
    "aria-colcount",
    "aria-colindex",
    "aria-colspan",
    "aria-controls",
    "aria-current",
    "aria-describedby",
    "aria-details",
    "aria-disabled",
    "aria-dropeffect",
    "aria-errormessage",
    "aria-expanded",
    "aria-flowto",
    "aria-grabbed",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-level",
    "aria-live",
    "aria-modal",
    "aria-multiline",
    "aria-multiselectable",
    "aria-orientation",
    "aria-owns",
    "aria-placeholder",
    "aria-posinset",
    "aria-pressed",
    "aria-readonly",
    "aria-relevant",
    "aria-required",
    "aria-roledescription",
    "aria-rowcount",
    "aria-rowindex",
    "aria-rowspan",
    "aria-selected",
    "aria-setsize",
    "aria-sort",
    "aria-valuemax",
    "aria-valuemin",
    "aria-valuenow",
    "aria-valuetext",
];

pub const ARIA_PROPERTY_TYPE: [&str; 9] = [
    "boolean",
    "id",
    "idlist",
    "integer",
    "number",
    "string",
    "token",
    "tokenlist",
    "tristate",
];

pub const ARIA_WIDGET_ROLES: [&str; 27] = [
    "alert",
    "alertdialog",
    "button",
    "checkbox",
    "dialog",
    "gridcell",
    "link",
    "log",
    "marquee",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "option",
    "progressbar",
    "radio",
    "scrollbar",
    "searchbox",
    "slider",
    "spinbutton",
    "status",
    "switch",
    "tab",
    "tabpanel",
    "textbox",
    "timer",
    "tooltip",
    "treeitem",
];

pub const ARIA_ABSTRACT_ROLES: [&str; 12] = [
    "command",
    "composite",
    "input",
    "landmark",
    "range",
    "roletype",
    "section",
    "sectionhead",
    "select",
    "structure",
    "widget",
    "window",
];

pub const ARIA_DOCUMENT_STRUCTURE_ROLES: [&str; 25] = [
    "article",
    "cell",
    "columnheader",
    "definition",
    "directory",
    "document",
    "feed",
    "figure",
    "group",
    "heading",
    "img",
    "list",
    "listitem",
    "math",
    "none",
    "note",
    "presentation",
    "region",
    "row",
    "rowgroup",
    "rowheader",
    "separator",
    "table",
    "term",
    "toolbar",
];

fn main() -> io::Result<()> {
    let aria_properties = generate_properties();
    let aria_roles = generate_roles();
    let tokens = quote! {
        use std::str::FromStr;


        #aria_properties
        #aria_roles
    };
    let ast = tokens.to_string();

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(PathBuf::from(out_dir).join("enums.rs"), ast)?;

    Ok(())
}

fn generate_properties() -> TokenStream {
    let properties = generate_enums(
        ARIA_PROPERTIES.len(),
        ARIA_PROPERTIES.iter(),
        "AriaPropertiesEnum",
    );

    let property_types = generate_enums(
        ARIA_PROPERTY_TYPE.len(),
        ARIA_PROPERTY_TYPE.iter(),
        "AriaPropertyTypeEnum",
    );

    quote! {
        #properties
        #property_types
    }
}

fn generate_roles() -> TokenStream {
    let widget_roles = generate_enums(
        ARIA_WIDGET_ROLES.len(),
        ARIA_WIDGET_ROLES.iter(),
        "AriaWidgetRolesEnum",
    );
    let abstract_roles = generate_enums(
        ARIA_ABSTRACT_ROLES.len(),
        ARIA_ABSTRACT_ROLES.iter(),
        "AriaAbstractRolesEnum",
    );

    let document_structure_roles = generate_enums(
        ARIA_DOCUMENT_STRUCTURE_ROLES.len(),
        ARIA_DOCUMENT_STRUCTURE_ROLES.iter(),
        "AriaDocumentStructureRolesEnum",
    );

    quote! {
        #widget_roles
        #abstract_roles
        #document_structure_roles
    }
}

fn generate_enums(len: usize, array: std::slice::Iter<&str>, enum_name: &str) -> TokenStream {
    let enum_name = Ident::new(enum_name, Span::call_site());
    let mut enum_metadata = Vec::with_capacity(len);
    let mut from_enum_metadata = Vec::with_capacity(len);
    let mut from_string_metadata = Vec::with_capacity(len);
    for property in array {
        let name = Ident::new(&property.replace('-', "_").to_camel(), Span::call_site());
        let property = Literal::string(property);
        from_enum_metadata.push(quote! {
            #enum_name::#name => #property
        });
        from_string_metadata.push(quote! {
            #property => Ok(#enum_name::#name)
        });
        enum_metadata.push(name);
    }

    from_string_metadata.push(quote! {
        _ => Err("aria property not implemented".to_string())
    });

    quote! {

        #[derive(Debug, Eq, PartialEq)]
        pub enum #enum_name {
            #( #enum_metadata ),*
        }

        impl From<#enum_name> for &str {
            fn from(property: #enum_name) -> Self {
                match property {
                    #( #from_enum_metadata ),*
                }
            }
        }

        impl FromStr for #enum_name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #from_string_metadata ),*
                }
            }
        }

        impl #enum_name {
            pub fn as_str(&self) -> &str {
                match self {
                    #( #from_enum_metadata ),*
                }
            }
        }
    }
}

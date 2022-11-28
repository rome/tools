use case::CaseExt;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use rome_aria::constants::{
    ARIA_ABSTRACT_ROLES, ARIA_DOCUMENT_STRUCTURE_ROLES, ARIA_PROPERTIES, ARIA_PROPERTY_TYPE,
    ARIA_WIDGET_ROLES,
};
use xtask::*;
use xtask::{project_root, Mode};
use xtask_codegen::update;

pub(crate) fn generate_aria(mode: Mode) -> Result<()> {
    let config_root = project_root().join("crates/rome_aria/src");
    let aria_properties = generate_properties();
    let aria_roles = generate_roles();
    let tokens = quote! {
        use std::str::FromStr;

        #aria_properties
        #aria_roles
    };
    let ast = tokens.to_string();
    let pretty = xtask::reformat(ast)?;

    update(&config_root.join("generated.rs"), &pretty, &mode)?;

    Ok(())
}

fn generate_properties() -> TokenStream {
    let properties = generate_enums(
        ARIA_PROPERTIES.len(),
        ARIA_PROPERTIES.iter(),
        "AriaPropertiesEnum",
        "ARIA_PROPERTIES",
    );

    let property_types = generate_enums(
        ARIA_PROPERTY_TYPE.len(),
        ARIA_PROPERTY_TYPE.iter(),
        "AriaPropertyTypeEnum",
        "ARIA_PROPERTY_TYPE",
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
        "ARIA_WIDGET_ROLES",
    );
    let abstract_roles = generate_enums(
        ARIA_ABSTRACT_ROLES.len(),
        ARIA_ABSTRACT_ROLES.iter(),
        "AriaAbstractRolesEnum",
        "ARIA_ABSTRACT_ROLES",
    );

    let document_structure_roles = generate_enums(
        ARIA_DOCUMENT_STRUCTURE_ROLES.len(),
        ARIA_DOCUMENT_STRUCTURE_ROLES.iter(),
        "AriaDocumentStructureRolesEnum",
        "ARIA_DOCUMENT_STRUCTURE_ROLES",
    );

    quote! {
        #widget_roles
        #abstract_roles
        #document_structure_roles
    }
}

fn generate_enums<'a>(
    len: usize,
    array: std::slice::Iter<&str>,
    enum_name: &str,
    const_name: &str,
) -> TokenStream {
    let enum_name = Ident::new(enum_name, Span::call_site());
    let const_name = Ident::new(const_name, Span::call_site());
    let mut enum_metadata = Vec::with_capacity(len);
    let mut from_enum_metadata = Vec::with_capacity(len);
    let mut from_string_metadata = Vec::with_capacity(len);
    let iter = array.enumerate();
    for (index, property) in iter {
        let name = Ident::new(
            &property.replace("-", "_").to_camel().to_string(),
            Span::call_site(),
        );
        let property = Literal::string(property);
        enum_metadata.push(quote! {
            #name
        });
        from_enum_metadata.push(quote! {
            #enum_name::#name => #property
        });
        from_string_metadata.push(quote! {
            #property => Ok(#enum_name::#name)
        })
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

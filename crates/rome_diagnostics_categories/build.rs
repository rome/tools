use quote::{format_ident, quote};
use std::{env, fs, io, path::PathBuf};

macro_rules! define_dategories {
    ( $( $name_link:literal : $link:literal, )* ; $( $name:literal , )* ) => {
        const CATEGORIES: &[(&str, Option<&str>)] = &[
            $( ($name_link, Some($link)), )*
            $( ($name, None), )*
        ];
    };
}

include!("src/categories.rs");

pub fn main() -> io::Result<()> {
    let mut metadata = Vec::with_capacity(CATEGORIES.len());
    let mut macro_arms = Vec::with_capacity(CATEGORIES.len());
    let mut parse_arms = Vec::with_capacity(CATEGORIES.len());

    for (name, link) in CATEGORIES {
        let meta_name = name.replace('/', "_").to_uppercase();
        let meta_ident = format_ident!("{meta_name}");

        let link = if let Some(link) = link {
            quote! { Some(#link) }
        } else {
            quote! { None }
        };

        metadata.push(quote! {
            pub static #meta_ident: crate::Category = crate::Category {
                name: #name,
                link: #link,
            };
        });

        macro_arms.push(quote! {
            (#name) => { &$crate::registry::#meta_ident };
        });

        parse_arms.push(quote! {
            #name => Ok(&crate::registry::#meta_ident),
        });
    }

    let tokens = quote! {
        impl FromStr for &'static Category {
            type Err = ();

            fn from_str(name: &str) -> Result<Self, ()> {
                match name {
                    #( #parse_arms )*
                    _ => Err(()),
                }
            }
        }

        /// The `category!` macro can be used to statically lookup a category
        /// by name from the registry
        ///
        /// # Example
        ///
        /// ```
        /// # use rome_diagnostics_categories::{Category, category};
        /// let category: &'static Category = category!("internalError/io");
        /// assert_eq!(category.name(), "internalError/io");
        /// assert_eq!(category.link(), None);
        /// ```
        #[macro_export]
        macro_rules! category {
            #( #macro_arms )*
        }

        pub mod registry {
            #( #metadata )*
        }
    };

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(
        PathBuf::from(out_dir).join("categories.rs"),
        tokens.to_string(),
    )?;

    Ok(())
}

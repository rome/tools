use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::*;
use quote::quote;

use crate::parse::{DeriveInput, StaticOrDynamic, StringOrMarkup};

pub(crate) fn generate_diagnostic(input: DeriveInput) -> TokenStream {
    let category = generate_category(&input);
    let severity = generate_severity(&input);
    let description = generate_description(&input);
    let message = generate_message(&input);
    let advices = generate_advices(&input);
    let verbose_advices = generate_verbose_advices(&input);
    let location = generate_location(&input);
    let tags = generate_tags(&input);
    let source = generate_source(&input);

    let ident = input.ident;

    quote! {
        impl rome_diagnostics::v2::Diagnostic for #ident {
            #category
            #severity
            #description
            #message
            #advices
            #verbose_advices
            #location
            #tags
            #source
        }
    }
}

fn generate_category(input: &DeriveInput) -> TokenStream {
    let category = match &input.category {
        Some(StaticOrDynamic::Static(value)) => quote! {
            rome_diagnostics::v2::category!(#value)
        },
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            self.#value
        },
        None => return quote!(),
    };

    quote! {
        fn category(&self) -> Option<&rome_diagnostics::v2::Category> {
            Some(#category)
        }
    }
}

fn generate_severity(input: &DeriveInput) -> TokenStream {
    let severity = match &input.severity {
        Some(StaticOrDynamic::Static(value)) => quote! {
            rome_diagnostics::v2::Severity::#value
        },
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            self.#value
        },
        None => return quote!(),
    };

    quote! {
        fn severity(&self) -> rome_diagnostics::v2::Severity {
            #severity
        }
    }
}

fn generate_description(input: &DeriveInput) -> TokenStream {
    let description = match &input.description {
        Some(StaticOrDynamic::Static(StringOrMarkup::String(value))) => {
            let mut format_string = String::new();
            let mut format_params = Vec::new();

            let input = value.value();
            let mut input = input.as_str();

            while let Some(idx) = input.find('{') {
                let (before, after) = input.split_at(idx);
                format_string.push_str(before);

                let after = &after[1..];
                format_string.push('{');

                if let Some(after) = after.strip_prefix('{') {
                    input = after;
                    continue;
                }

                let end = match after.find([':', '}']) {
                    Some(end) => end,
                    None => abort!(value.span(), "failed to parse format string"),
                };

                let (ident, after) = after.split_at(end);
                let ident = Ident::new(ident, Span::call_site());
                format_params.push(quote! { self.#ident });

                input = after;
            }

            if !input.is_empty() {
                format_string.push_str(input);
            }

            if format_params.is_empty() {
                quote! {
                    fmt.write_str(#format_string)
                }
            } else {
                quote! {
                    fmt.write_fmt(::std::format_args!(#format_string, #( #format_params ),*))
                }
            }
        }
        Some(StaticOrDynamic::Static(StringOrMarkup::Markup(markup))) => quote! {
            let mut buffer = Vec::new();

            let write = rome_diagnostics::termcolor::NoColor::new(&mut buffer);
            let mut write = rome_diagnostics::v2::console::fmt::Termcolor(write);
            let mut write = rome_diagnostics::v2::console::fmt::Formatter::new(&mut write);

            use rome_diagnostics::v2::console as rome_console;
            write.write_markup(&rome_diagnostics::v2::console::markup!{ #markup })
                .map_err(|_| ::std::fmt::Error)?;

            fmt.write_str(::std::str::from_utf8(&buffer).map_err(|_| ::std::fmt::Error)?)
        },
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            fmt.write_fmt(::std::format_args!("{}", self.#value))
        },
        None => return quote!(),
    };

    quote! {
        fn description(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            #description
        }
    }
}

fn generate_message(input: &DeriveInput) -> TokenStream {
    let message = match &input.message {
        Some(StaticOrDynamic::Static(StringOrMarkup::String(value))) => quote! {
            fmt.write_str(#value)
        },
        Some(StaticOrDynamic::Static(StringOrMarkup::Markup(markup))) => quote! {
            use rome_diagnostics::v2::console as rome_console;
            fmt.write_markup(rome_diagnostics::v2::console::markup!{ #markup })
        },
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            rome_diagnostics::v2::console::fmt::Display::fmt(&self.#value, fmt)
        },
        None => return quote!(),
    };

    quote! {
        fn message(&self, fmt: &mut rome_diagnostics::v2::console::fmt::Formatter<'_>) -> ::std::io::Result<()> {
            #message
        }
    }
}

fn generate_advices(input: &DeriveInput) -> TokenStream {
    if input.advices.is_empty() {
        return quote!();
    }

    let advices = input.advices.iter();

    quote! {
        fn advices(&self, visitor: &mut dyn rome_diagnostics::v2::Visitor) -> ::std::io::Result<()> {
            #( rome_diagnostics::v2::IntoAdvices::visit(&self.#advices, visitor)?; )*
            Ok(())
        }
    }
}

fn generate_verbose_advices(input: &DeriveInput) -> TokenStream {
    if input.verbose_advices.is_empty() {
        return quote!();
    }

    let verbose_advices = input.verbose_advices.iter();

    quote! {
        fn verbose_advices(&self, visitor: &mut dyn rome_diagnostics::v2::Visitor) -> ::std::io::Result<()> {
            #( rome_diagnostics::v2::IntoAdvices::visit(&self.#verbose_advices, visitor)?; )*
            Ok(())
        }
    }
}

fn generate_location(input: &DeriveInput) -> TokenStream {
    if input.location.is_empty() {
        return quote!();
    }

    let field = input.location.iter().map(|(field, _)| field);
    let method = input.location.iter().map(|(_, method)| method);

    quote! {
        fn location(&self) -> Option<rome_diagnostics::v2::Location<'_>> {
            rome_diagnostics::v2::Location::builder()
                #( .#method(&self.#field) )*
                .build()
        }
    }
}

fn generate_tags(input: &DeriveInput) -> TokenStream {
    let tags = match &input.tags {
        Some(StaticOrDynamic::Static(value)) => {
            let values = value.iter();
            quote! {
                #( rome_diagnostics::v2::DiagnosticTags::#values )|*
            }
        }
        Some(StaticOrDynamic::Dynamic(value)) => quote! {
            self.#value
        },
        None => return quote!(),
    };

    quote! {
        fn tags(&self) -> rome_diagnostics::v2::DiagnosticTags {
            #tags
        }
    }
}

fn generate_source(input: &DeriveInput) -> TokenStream {
    match &input.source {
        Some(value) => quote! {
            fn source(&self) -> Option<&dyn rome_diagnostics::v2::Diagnostic> {
                self.#value.as_deref()
            }
        },
        None => quote!(),
    }
}

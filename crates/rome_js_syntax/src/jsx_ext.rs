use std::mem::MaybeUninit;

use crate::{
    JsxAttribute, JsxAttributeList, JsxName, JsxOpeningElement, JsxSelfClosingElement, JsxString,
    TextSize,
};
use rome_rowan::{AstNode, AstNodeList, SyntaxResult, SyntaxTokenText};

impl JsxString {
    /// Get the inner text of a string not including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make::{jsx_ident, jsx_string};
    /// use rome_rowan::TriviaPieceKind;
    ///
    ///let string = jsx_string(jsx_ident("button").with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// assert_eq!(string.inner_string_text().unwrap().text(), "button");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<SyntaxTokenText> {
        let value = self.value_token()?;
        let mut text = value.token_text_trimmed();

        static QUOTES: [char; 2] = ['"', '\''];

        if text.starts_with(QUOTES) {
            let range = text.range().add_start(TextSize::from(1));
            text = text.slice(range);
        }

        if text.ends_with(QUOTES) {
            let range = text.range().sub_end(TextSize::from(1));
            text = text.slice(range);
        }

        Ok(text)
    }
}

impl JsxOpeningElement {
    /// Find and return the `JsxAttribute` that matches the given name
    ///
    /// ## Examples
    ///
    /// ```
    ///
    /// use rome_js_factory::make;
    /// use rome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list};
    /// use rome_js_syntax::{JsxAnyAttribute, JsxAnyAttributeName, JsxAnyElementName, T};
    ///
    /// let div = JsxAnyAttribute::JsxAttribute(jsx_attribute(
    ///     JsxAnyAttributeName::JsxName(
    ///         jsx_name(ident("div"))
    ///     )
    /// ).build());
    ///
    /// let img = JsxAnyAttribute::JsxAttribute(jsx_attribute(
    ///     JsxAnyAttributeName::JsxName(
    ///         jsx_name(ident("img"))
    ///     )
    /// ).build());
    ///
    /// let attributes = jsx_attribute_list(vec![
    ///     div,
    ///     img
    /// ]);
    ///
    /// let opening_element = jsx_opening_element(
    ///     token(T![<]),
    ///     JsxAnyElementName::JsxName(
    ///         jsx_name(ident("Test"))
    ///     ),
    ///     attributes,
    ///     token(T![>]),
    /// ).build();
    ///
    /// assert_eq!(opening_element.find_attribute_by_name("div").unwrap().is_some(), true);
    /// assert_eq!(opening_element.find_attribute_by_name("img").unwrap().is_some(), true);
    /// assert_eq!(opening_element.find_attribute_by_name("p").unwrap().is_some(), false);
    /// ```
    ///
    pub fn find_attribute_by_name(
        &self,
        name_to_lookup: &str,
    ) -> SyntaxResult<Option<JsxAttribute>> {
        find_attribute_by_name(self.attributes(), name_to_lookup)
    }
}

impl JsxSelfClosingElement {
    /// Find and return the `JsxAttribute` that matches the given name
    ///
    /// ## Examples
    ///
    /// ```
    ///
    /// use rome_js_factory::make;
    /// use rome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list, jsx_self_closing_element};
    /// use rome_js_syntax::{JsxAnyAttribute, JsxAnyAttributeName, JsxAnyElementName, T};
    ///
    /// let div = JsxAnyAttribute::JsxAttribute(jsx_attribute(
    ///     JsxAnyAttributeName::JsxName(
    ///         jsx_name(ident("div"))
    ///     )
    /// ).build());
    ///
    /// let img = JsxAnyAttribute::JsxAttribute(jsx_attribute(
    ///     JsxAnyAttributeName::JsxName(
    ///         jsx_name(ident("img"))
    ///     )
    /// ).build());
    ///
    /// let attributes = jsx_attribute_list(vec![
    ///     div,
    ///     img
    /// ]);
    ///
    /// let opening_element = jsx_self_closing_element(
    ///     token(T![<]),
    ///     JsxAnyElementName::JsxName(
    ///         jsx_name(ident("Test"))
    ///     ),
    ///     attributes,
    ///     token(T![/]),
    ///     token(T![>]),
    /// ).build();
    ///
    /// assert_eq!(opening_element.find_attribute_by_name("div").unwrap().is_some(), true);
    /// assert_eq!(opening_element.find_attribute_by_name("img").unwrap().is_some(), true);
    /// assert_eq!(opening_element.find_attribute_by_name("p").unwrap().is_some(), false);
    /// ```
    ///
    pub fn find_attribute_by_name(
        &self,
        name_to_lookup: &str,
    ) -> SyntaxResult<Option<JsxAttribute>> {
        find_attribute_by_name(self.attributes(), name_to_lookup)
    }

}


macro_rules! init_none_array {
    (@ALWAYSNONE, $name:literal) => {None};
    (@NAMECOMPARISON, $captures:expr, $i:expr, $att:expr, $element_name:expr, $head:literal,) => {
        if $element_name == $head {
            $captures[$i] = Some($att);
        }
    };
    (@NAMECOMPARISON, $captures:expr, $i:expr, $att:expr, $element_name:expr, $head:literal, $($tail:literal,)*) => {
        if $element_name == $head {
            $captures[$i] = Some($att.clone());
        }

        jsx_attributes!(@NAMECOMPARISON, $captures, $i + 1, $att, $element_name, $($tail,)*);
    };
    ($n:expr) => {{
        use rome_rowan::AstNodeList;
        let mut captures = [$(jsx_attributes!(@ALWAYSNONE, $names),)*];
        for att in $attributes.iter() {
            if let Some(attribute) = $crate::JsxAttribute::cast_ref(att.syntax()) {
                if let Some(name) = attribute.name().ok()
                    .and_then(|x| $crate::JsxName::cast_ref(x.syntax())) 
                    .and_then(|x| x.value_token().ok())
                {
                    let name = name.text_trimmed();
                    jsx_attributes!(@NAMECOMPARISON, captures, 0, attribute, name, $($names,)*);
                }
            }
        };
        captures
    }};
}

impl JsxAttributeList {
    pub fn find_attributes_by_name<const N: usize>( &self, names_to_lookup: [&str; N]) -> [Option<JsxAttribute>; N]
    {
        let mut captures: [Option<JsxAttribute>; N] = unsafe {
            let mut arr: MaybeUninit<_> = MaybeUninit::uninit();
            for i in 0..N {
                (arr.as_mut_ptr() as *mut Option<JsxAttribute>).add(i).write(None);
            }
            arr.assume_init()
        };

        for att in self.iter() {
            if let Some(attribute) = JsxAttribute::cast_ref(att.syntax()) {
                if let Some(name) = attribute.name().ok()
                    .and_then(|x| JsxName::cast_ref(x.syntax())) 
                    .and_then(|x| x.value_token().ok())
                {
                    let name = name.text_trimmed();
                    for (i, name_to_lookup) in names_to_lookup.iter().enumerate() {
                        if captures[i].is_none() && *name_to_lookup == name {
                            captures[i] = Some(attribute.clone());
                        }
                    }
                }
            }
        };

        captures
    }
}

pub fn find_attribute_by_name(
    attributes: JsxAttributeList,
    name_to_lookup: &str,
) -> SyntaxResult<Option<JsxAttribute>> {
    let attribute = attributes.iter().find_map(|attribute| {
        let attribute = JsxAttribute::cast_ref(attribute.syntax())?;
        let name = attribute.name().ok()?;
        let name = JsxName::cast_ref(name.syntax())?;
        if name.value_token().ok()?.text_trimmed() == name_to_lookup {
            Some(attribute)
        } else {
            None
        }
    });

    Ok(attribute)
}

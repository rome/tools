use std::collections::HashSet;

use crate::{
    JsSyntaxToken, JsxAnyAttribute, JsxAnyElementName, JsxAttribute, JsxAttributeList, JsxName,
    JsxOpeningElement, JsxSelfClosingElement, JsxString, TextSize,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList, SyntaxResult, SyntaxTokenText};

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
        self.attributes().find_by_name(name_to_lookup)
    }

    /// It checks if current attribute has a trailing spread props
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make;
    /// use rome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list, jsx_self_closing_element, jsx_spread_attribute, jsx_ident, js_identifier_expression, js_reference_identifier};
    /// use rome_js_syntax::{JsAnyExpression, JsxAnyAttribute, JsxAnyAttributeName, JsxAnyElementName, T};
    ///
    /// let div = JsxAnyAttribute::JsxAttribute(jsx_attribute(
    ///     JsxAnyAttributeName::JsxName(
    ///         jsx_name(ident("div"))
    ///     )
    /// ).build());
    ///
    /// let spread = JsxAnyAttribute::JsxSpreadAttribute(jsx_spread_attribute(
    ///     token(T!['{']),
    ///     token(T![...]),
    ///     JsAnyExpression::JsIdentifierExpression(js_identifier_expression(
    ///         js_reference_identifier(ident("spread"))
    ///     )),
    ///     token(T!['}']),
    /// ));
    ///
    ///
    ///
    /// let attributes = jsx_attribute_list(vec![
    ///     div,
    ///     spread
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
    /// let div = opening_element.find_attribute_by_name("div").unwrap().unwrap();
    /// assert!(opening_element.has_trailing_spread_prop(div.clone()));
    /// ```
    pub fn has_trailing_spread_prop(&self, current_attribute: impl Into<JsxAnyAttribute>) -> bool {
        self.attributes()
            .has_trailing_spread_prop(current_attribute)
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
        self.attributes().find_by_name(name_to_lookup)
    }

    /// It checks if current attribute has a trailing spread props
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make;
    /// use rome_js_factory::make::{ident, jsx_attribute, jsx_name, jsx_opening_element, token, jsx_attribute_list, jsx_self_closing_element, jsx_spread_attribute, jsx_ident, js_identifier_expression, js_reference_identifier};
    /// use rome_js_syntax::{JsAnyExpression, JsxAnyAttribute, JsxAnyAttributeName, JsxAnyElementName, T};
    ///
    /// let div = JsxAnyAttribute::JsxAttribute(jsx_attribute(
    ///     JsxAnyAttributeName::JsxName(
    ///         jsx_name(ident("div"))
    ///     )
    /// ).build());
    ///
    /// let spread = JsxAnyAttribute::JsxSpreadAttribute(jsx_spread_attribute(
    ///     token(T!['{']),
    ///     token(T![...]),
    ///     JsAnyExpression::JsIdentifierExpression(js_identifier_expression(
    ///         js_reference_identifier(ident("spread"))
    ///     )),
    ///     token(T!['}']),
    /// ));
    ///
    ///
    ///
    /// let attributes = jsx_attribute_list(vec![
    ///     div,
    ///     spread
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
    /// let div = opening_element.find_attribute_by_name("div").unwrap().unwrap();
    /// assert!(opening_element.has_trailing_spread_prop(div.clone()));
    /// ```
    pub fn has_trailing_spread_prop(&self, current_attribute: impl Into<JsxAnyAttribute>) -> bool {
        self.attributes()
            .has_trailing_spread_prop(current_attribute)
    }
}

impl JsxAttributeList {
    /// Finds and returns attributes `JsxAttribute` that matches the given names like [Self::find_by_name].
    /// Only attributes with name as [JsxName] can be returned.
    ///
    /// Each name of "names_to_lookup" should be unique.
    ///
    /// Supports maximum of 16 names to avoid stack overflow. Each attribute will consume:
    ///
    /// - 8 bytes for the [Option<JsxAttribute>] result;
    /// - plus 16 bytes for the [&str] argument.
    pub fn find_by_names<const N: usize>(
        &self,
        names_to_lookup: [&str; N],
    ) -> [Option<JsxAttribute>; N] {
        // assert there are no duplicates
        debug_assert!(HashSet::<_>::from_iter(names_to_lookup).len() == N);
        debug_assert!(N <= 16);

        const INIT: Option<JsxAttribute> = None;
        let mut results = [INIT; N];

        let mut missing = N;

        'attributes: for att in self {
            if let Some(attribute) = att.as_jsx_attribute() {
                if let Some(name) = attribute
                    .name()
                    .ok()
                    .and_then(|x| x.as_jsx_name()?.value_token().ok())
                {
                    let name = name.text_trimmed();
                    for i in 0..N {
                        if results[i].is_none() && names_to_lookup[i] == name {
                            results[i] = Some(attribute.clone());
                            if missing == 1 {
                                break 'attributes;
                            } else {
                                missing -= 1;
                                break;
                            }
                        }
                    }
                }
            }
        }

        results
    }

    pub fn find_by_name(&self, name_to_lookup: &str) -> SyntaxResult<Option<JsxAttribute>> {
        let attribute = self.iter().find_map(|attribute| {
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

    pub fn has_trailing_spread_prop(&self, current_attribute: impl Into<JsxAnyAttribute>) -> bool {
        let mut current_attribute_found = false;
        let current_attribute = current_attribute.into();
        for attribute in self {
            if attribute == current_attribute {
                current_attribute_found = true;
                continue;
            }
            if current_attribute_found && attribute.as_jsx_spread_attribute().is_some() {
                return true;
            }
        }
        false
    }
}

declare_node_union! {
    pub JsxAnyElement = JsxOpeningElement | JsxSelfClosingElement
}

impl JsxAnyElement {
    pub fn name_value_token(&self) -> Option<JsSyntaxToken> {
        match self.name().ok()? {
            JsxAnyElementName::JsxMemberName(member) => member.member().ok()?.value_token().ok(),
            JsxAnyElementName::JsxName(name) => name.value_token().ok(),
            JsxAnyElementName::JsxNamespaceName(name) => name.name().ok()?.value_token().ok(),
            JsxAnyElementName::JsxReferenceIdentifier(name) => name.value_token().ok(),
        }
    }

    pub fn attributes(&self) -> JsxAttributeList {
        match self {
            JsxAnyElement::JsxOpeningElement(element) => element.attributes(),
            JsxAnyElement::JsxSelfClosingElement(element) => element.attributes(),
        }
    }

    pub fn name(&self) -> SyntaxResult<JsxAnyElementName> {
        match self {
            JsxAnyElement::JsxOpeningElement(element) => element.name(),
            JsxAnyElement::JsxSelfClosingElement(element) => element.name(),
        }
    }

    pub fn is_custom_component(&self) -> Option<bool> {
        self.name().ok()?.as_jsx_name().map(|_| true)
    }

    pub fn has_trailing_spread_prop(&self, current_attribute: impl Into<JsxAnyAttribute>) -> bool {
        match self {
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.has_trailing_spread_prop(current_attribute)
            }
            JsxAnyElement::JsxOpeningElement(element) => {
                element.has_trailing_spread_prop(current_attribute)
            }
        }
    }

    pub fn find_attribute_by_name(&self, name_to_lookup: &str) -> Option<JsxAttribute> {
        match self {
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.find_attribute_by_name(name_to_lookup).ok()?
            }
            JsxAnyElement::JsxOpeningElement(element) => {
                element.find_attribute_by_name(name_to_lookup).ok()?
            }
        }
    }

    pub fn has_valid_focus_attributes(&self) -> Option<bool> {
        if let Some(on_mouse_over_attribute) = self.find_attribute_by_name("onMouseOver") {
            if !self.has_trailing_spread_prop(on_mouse_over_attribute) {
                let on_focus_attribute = self.find_attribute_by_name("onFocus")?;
                if on_focus_attribute.is_value_undefined_or_null() {
                    return None;
                }
            }
        }
        Some(true)
    }

    pub fn has_valid_blur_attributes(&self) -> Option<bool> {
        if let Some(on_mouse_attribute) = self.find_attribute_by_name("onMouseOut") {
            if !self.has_trailing_spread_prop(on_mouse_attribute) {
                let on_blur_attribute = self.find_attribute_by_name("onBlur")?;
                if on_blur_attribute.is_value_undefined_or_null() {
                    return None;
                }
            }
        }
        Some(true)
    }
}

impl JsxAttribute {
    fn is_value_undefined_or_null(&self) -> bool {
        self.initializer()
            .and_then(|x| {
                let expression = x
                    .value()
                    .ok()?
                    .as_jsx_expression_attribute_value()?
                    .expression()
                    .ok()?;

                if let Some(id) = expression.as_js_identifier_expression() {
                    let name = id.name().ok()?.syntax().text_trimmed();

                    return Some(name == "undefined");
                }

                expression
                    .as_js_any_literal_expression()?
                    .as_js_null_literal_expression()?;

                Some(true)
            })
            .unwrap_or(false)
    }
}

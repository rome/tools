use crate::context::Semicolons;
use crate::prelude::*;
use crate::utils::{FormatSemicolon, JsAnyAssignmentLike};
use rome_formatter::write;
use rome_js_syntax::{
    JsAnyClassMember, JsAnyClassMemberName, JsInitializerClause, JsPropertyClassMember,
    JsSyntaxToken, TsPropertySignatureClassMember,
};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPropertyClassMember;

impl FormatNodeRule<JsPropertyClassMember> for FormatJsPropertyClassMember {
    fn fmt_fields(&self, node: &JsPropertyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let semicolon_token = node.semicolon_token();
        write!(
            f,
            [
                &JsAnyAssignmentLike::from(node.clone()),
                FormatClassPropertySemicolon {
                    semicolon: semicolon_token.as_ref(),
                    property: &JsAnyPropertyClassMember::from(node.clone())
                }
            ]
        )
    }
}

declare_node_union! {
    pub(crate) JsAnyPropertyClassMember = JsPropertyClassMember | TsPropertySignatureClassMember
}

impl JsAnyPropertyClassMember {
    fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
        match self {
            JsAnyPropertyClassMember::JsPropertyClassMember(property) => property.name(),
            JsAnyPropertyClassMember::TsPropertySignatureClassMember(property) => property.name(),
        }
    }

    fn value(&self) -> Option<JsInitializerClause> {
        match self {
            JsAnyPropertyClassMember::JsPropertyClassMember(property) => property.value(),
            JsAnyPropertyClassMember::TsPropertySignatureClassMember(_) => None,
        }
    }

    fn has_property_annotation(&self) -> bool {
        match self {
            JsAnyPropertyClassMember::JsPropertyClassMember(property) => {
                property.property_annotation().is_some()
            }
            JsAnyPropertyClassMember::TsPropertySignatureClassMember(property) => {
                property.property_annotation().is_some()
            }
        }
    }
}

pub(crate) struct FormatClassPropertySemicolon<'a> {
    property: &'a JsAnyPropertyClassMember,
    semicolon: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatClassPropertySemicolon<'a> {
    pub fn new(
        property: &'a JsAnyPropertyClassMember,
        semicolon: Option<&'a JsSyntaxToken>,
    ) -> Self {
        Self {
            property,
            semicolon,
        }
    }
}

impl Format<JsFormatContext> for FormatClassPropertySemicolon<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match f.options().semicolons() {
            Semicolons::Always => FormatSemicolon::new(self.semicolon).fmt(f),
            Semicolons::AsNeeded => match (self.semicolon, needs_semicolon(self.property)?) {
                (None, false) => Ok(()),
                (Some(semicolon), true) => semicolon.format().fmt(f),

                (Some(semicolon), false) => format_removed(semicolon).fmt(f),
                (None, true) => FormatSemicolon::new(self.semicolon).fmt(f),
            },
        }
    }
}

fn needs_semicolon(property: &JsAnyPropertyClassMember) -> SyntaxResult<bool> {
    if let JsAnyClassMemberName::JsLiteralMemberName(name) = property.name()? {
        // `get;`, `set;` or `static`
        if matches!(name.value()?.text_trimmed(), "static" | "get" | "set")
            && property.value().is_none()
            && !property.has_property_annotation()
        {
            return Ok(true);
        }
    }

    let Some(next_member) = property.syntax().next_sibling().and_then(JsAnyClassMember::cast) else { return Ok(false); };

    // a;
    // static b;
    if has_modifiers(&next_member) {
        return Ok(false);
    }

    // a = b;
    // instanceof;
    if property.value().is_some()
        && (next_member.has_name("instanceof")? || next_member.has_name("in")?)
    {
        return Ok(true);
    }

    Ok(match next_member {
        JsAnyClassMember::TsConstructorSignatureClassMember(_)
        | JsAnyClassMember::JsConstructorClassMember(_)
        | JsAnyClassMember::JsEmptyClassMember(_)
        // `get`, `set`, and `static` start with a keyword -> no need for semi
        | JsAnyClassMember::JsStaticInitializationBlockClassMember(_)
        | JsAnyClassMember::JsGetterClassMember(_)
        | JsAnyClassMember::TsGetterSignatureClassMember(_)
        | JsAnyClassMember::TsSetterSignatureClassMember(_)
        | JsAnyClassMember::JsSetterClassMember(_) => false,

        // Computed members may be misinterpreted as array accessors/array types
        member @ JsAnyClassMember::JsPropertyClassMember(_)
        | member @ JsAnyClassMember::TsPropertySignatureClassMember(_) => match member.name()? {
            Some(name) => name.is_computed(),
            None => false,
        },

        // When the name starts with the generator token or `[`
        JsAnyClassMember::JsMethodClassMember(method) => {
            method.async_token().is_none()
                && (method.name()?.is_computed() || method.star_token().is_some())
        }

        // When the name starts with a `[`
        JsAnyClassMember::TsMethodSignatureClassMember(method) => {
            method.async_token().is_none() && method.name()?.is_computed()
        }

        // Keep it, just to be safe
        JsAnyClassMember::JsBogusMember(_) => true,

        JsAnyClassMember::TsIndexSignatureClassMember(_) => true,
    })
}

/// Tests if `member` has any modifiers
fn has_modifiers(member: &JsAnyClassMember) -> bool {
    let is_empty = match member {
        JsAnyClassMember::JsConstructorClassMember(constructor) => {
            constructor.modifiers().is_empty()
        }
        JsAnyClassMember::JsEmptyClassMember(_) => true,
        JsAnyClassMember::JsGetterClassMember(getter) => getter.modifiers().is_empty(),
        JsAnyClassMember::JsMethodClassMember(method) => method.modifiers().is_empty(),
        JsAnyClassMember::JsPropertyClassMember(property) => property.modifiers().is_empty(),
        JsAnyClassMember::JsSetterClassMember(setter) => setter.modifiers().is_empty(),
        JsAnyClassMember::JsStaticInitializationBlockClassMember(_) => true,
        JsAnyClassMember::JsBogusMember(_) => true,
        JsAnyClassMember::TsConstructorSignatureClassMember(constructor) => {
            constructor.modifiers().is_empty()
        }
        JsAnyClassMember::TsGetterSignatureClassMember(getter) => getter.modifiers().is_empty(),
        JsAnyClassMember::TsIndexSignatureClassMember(index) => index.modifiers().is_empty(),
        JsAnyClassMember::TsMethodSignatureClassMember(method) => method.modifiers().is_empty(),
        JsAnyClassMember::TsPropertySignatureClassMember(property) => {
            property.modifiers().is_empty()
        }
        JsAnyClassMember::TsSetterSignatureClassMember(setter) => setter.modifiers().is_empty(),
    };

    !is_empty
}

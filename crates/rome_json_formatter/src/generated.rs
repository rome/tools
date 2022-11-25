//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::{
    AsFormat, FormatNodeRule, FormatUnknownNodeRule, IntoFormat, JsonFormatContext, JsonFormatter,
};
use rome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<rome_json_syntax::JsonRoot> for crate::json::auxiliary::root::FormatJsonRoot {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonRoot, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonRoot>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonRoot {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonRoot,
        crate::json::auxiliary::root::FormatJsonRoot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::auxiliary::root::FormatJsonRoot::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonRoot {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonRoot,
        crate::json::auxiliary::root::FormatJsonRoot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::auxiliary::root::FormatJsonRoot::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonString> for crate::json::value::string::FormatJsonString {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonString, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonString>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonString {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonString,
        crate::json::value::string::FormatJsonString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::string::FormatJsonString::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonString {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonString,
        crate::json::value::string::FormatJsonString,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::string::FormatJsonString::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonBoolean> for crate::json::value::boolean::FormatJsonBoolean {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonBoolean, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonBoolean>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonBoolean {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonBoolean,
        crate::json::value::boolean::FormatJsonBoolean,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::boolean::FormatJsonBoolean::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonBoolean {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonBoolean,
        crate::json::value::boolean::FormatJsonBoolean,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::boolean::FormatJsonBoolean::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonNull> for crate::json::value::null::FormatJsonNull {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonNull, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonNull>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonNull {
    type Format<'a> =
        FormatRefWithRule<'a, rome_json_syntax::JsonNull, crate::json::value::null::FormatJsonNull>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::json::value::null::FormatJsonNull::default())
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonNull {
    type Format =
        FormatOwnedWithRule<rome_json_syntax::JsonNull, crate::json::value::null::FormatJsonNull>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::json::value::null::FormatJsonNull::default())
    }
}
impl FormatRule<rome_json_syntax::JsonNumber> for crate::json::value::number::FormatJsonNumber {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonNumber, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonNumber>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonNumber {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonNumber,
        crate::json::value::number::FormatJsonNumber,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::number::FormatJsonNumber::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonNumber {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonNumber,
        crate::json::value::number::FormatJsonNumber,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::number::FormatJsonNumber::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonArray> for crate::json::value::array::FormatJsonArray {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonArray, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonArray>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonArray {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonArray,
        crate::json::value::array::FormatJsonArray,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::json::value::array::FormatJsonArray::default())
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonArray {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonArray,
        crate::json::value::array::FormatJsonArray,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::json::value::array::FormatJsonArray::default())
    }
}
impl FormatRule<rome_json_syntax::JsonObject> for crate::json::value::object::FormatJsonObject {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonObject, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonObject>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonObject {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonObject,
        crate::json::value::object::FormatJsonObject,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::object::FormatJsonObject::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonObject {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonObject,
        crate::json::value::object::FormatJsonObject,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::object::FormatJsonObject::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonMember> for crate::json::auxiliary::member::FormatJsonMember {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonMember, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonMember,
        crate::json::auxiliary::member::FormatJsonMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::auxiliary::member::FormatJsonMember::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonMember {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonMember,
        crate::json::auxiliary::member::FormatJsonMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::auxiliary::member::FormatJsonMember::default(),
        )
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonArrayElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonArrayElementList,
        crate::json::lists::array_element_list::FormatJsonArrayElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::lists::array_element_list::FormatJsonArrayElementList::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonArrayElementList {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonArrayElementList,
        crate::json::lists::array_element_list::FormatJsonArrayElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::lists::array_element_list::FormatJsonArrayElementList::default(),
        )
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonMemberList {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonMemberList,
        crate::json::lists::member_list::FormatJsonMemberList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::lists::member_list::FormatJsonMemberList::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonMemberList {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonMemberList,
        crate::json::lists::member_list::FormatJsonMemberList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::lists::member_list::FormatJsonMemberList::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonUnknown>
    for crate::json::unknown::unknown::FormatJsonUnknown
{
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonUnknown, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatUnknownNodeRule::<rome_json_syntax::JsonUnknown>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonUnknown {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonUnknown,
        crate::json::unknown::unknown::FormatJsonUnknown,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::unknown::unknown::FormatJsonUnknown::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonUnknown {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonUnknown,
        crate::json::unknown::unknown::FormatJsonUnknown,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::unknown::unknown::FormatJsonUnknown::default(),
        )
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonAnyValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonAnyValue,
        crate::json::any::value::FormatJsonAnyValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::json::any::value::FormatJsonAnyValue::default())
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonAnyValue {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonAnyValue,
        crate::json::any::value::FormatJsonAnyValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::json::any::value::FormatJsonAnyValue::default())
    }
}

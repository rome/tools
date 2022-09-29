use crate::prelude::*;
use rome_js_syntax::{JsAnyFunctionBody, JsSyntaxKind};

pub(crate) struct FormatMaybeCachedFunctionBody<'a> {
    pub body: &'a JsAnyFunctionBody,
    pub lookup_cache: bool,
}

impl Format<JsFormatContext> for FormatMaybeCachedFunctionBody<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        if self.lookup_cache {
            let cached = f.context().get_cached_function_body(self.body);
            debug_assert!(
                cached.is_some(),
                "Expected cache to be initialized for the cases where cache lookup is enabled."
            );

            if let Some(cached) = f.context().get_cached_function_body(self.body) {
                return f.write_element(cached);
            }
        }

        let in_call_arguments = self.body.syntax().grand_parent().map_or(false, |node| {
            node.kind() == JsSyntaxKind::JS_CALL_ARGUMENT_LIST
        });

        if f.context().is_cache_function_bodies_enabled() && in_call_arguments {
            match f.context().get_cached_function_body(self.body) {
                Some(cached) => return f.write_element(cached),
                None => {
                    if let Some(interned) = f.intern(&self.body.format())? {
                        f.context_mut()
                            .insert_cached_function_body(self.body, interned.clone());
                        return f.write_element(interned);
                    }
                }
            }
        }

        self.body.format().fmt(f)
    }
}

use std::iter;

use crate::AnyTsType;

impl AnyTsType {
    /// Try to extract non `TsParenthesizedType` from `AnyTsType`
    pub fn omit_parentheses(self) -> AnyTsType {
        let first = self.as_ts_parenthesized_type().and_then(|x| x.ty().ok());
        iter::successors(first, |x| {
            let parenthesized = x.as_ts_parenthesized_type()?;
            parenthesized.ty().ok()
        })
        .last()
        .unwrap_or(self)
    }
}

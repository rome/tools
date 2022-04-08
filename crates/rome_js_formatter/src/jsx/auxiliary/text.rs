use std::borrow::Cow;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_formatter::Token;
use rome_js_syntax::{JsxText, JsxTextFields};
use rome_rowan::AstNode;

impl ToFormatElement for JsxText {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxTextFields { value_token } = self.as_fields();
        let token = value_token?;
        let new_text = clean_jsx_text(token.text());

        Ok(formatter.format_replaced(
            &token,
            Token::new_dynamic(new_text.to_string(), token.text_trimmed_range()).into(),
        ))
    }
}

fn clean_jsx_text(text: &str) -> Cow<str> {
    if text.len() == 0 {
        Cow::Borrowed(text)
    } else {
        let terminators = [' ', '\n', '\t'];
        let mut result = String::new();
        if text.starts_with(terminators) {
            result.push(' ');
        }
        let split_text = text.split_ascii_whitespace();
        for word in split_text {
            result.reserve(word.len() + 1);
            result.push_str(word);
            result.push(' ');
        }
        if !text.ends_with(terminators) {
            result.pop();
        }

        if result == text || result.is_empty() {
            Cow::Borrowed(text)
        } else {
            Cow::Owned(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::jsx::auxiliary::text::clean_jsx_text;

    #[test]
    fn clean_jsx_text_works() {
        assert_eq!("", clean_jsx_text(""));
        assert_eq!(" ", clean_jsx_text(" "));
        assert_eq!("Foo", clean_jsx_text("Foo"));
        assert_eq!(" Foo", clean_jsx_text(" Foo"));
        assert_eq!(" Foo", clean_jsx_text("\nFoo"));
        assert_eq!(" Foo", clean_jsx_text("\tFoo"));
        assert_eq!(" Foo", clean_jsx_text("\n \t Foo"));
        assert_eq!(" Foo", clean_jsx_text("\n \t \n \t\nFoo"));
        assert_eq!(" Foo bar lorem", clean_jsx_text(" Foo bar lorem"));
        assert_eq!("Foo ", clean_jsx_text("Foo "));
        assert_eq!("Foo ", clean_jsx_text("Foo\n"));
        assert_eq!("Foo ", clean_jsx_text("Foo\t"));
        assert_eq!("Foo ", clean_jsx_text("Foo\t \n "));
        assert_eq!("Foo ", clean_jsx_text("Foo\n \t \n \t\n"));
        assert_eq!("Foo Bar", clean_jsx_text("Foo\n \t\t\n \tBar"));
        assert_eq!(
            " Foo Bar ",
            clean_jsx_text("\n \t\t\n \tFoo\n \t\t\n \tBar\n \t\t\n \t")
        );
    }
}

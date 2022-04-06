use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_formatter::{format_elements, token, Token};
use rome_js_syntax::{AstNode, JsxText, JsxTextFields};
impl ToFormatElement for JsxText {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxTextFields { value_token } = self.as_fields();
        let token = value_token?;
        let new_text = clean_jsx_text(token.text());

        Ok(formatter.format_replaced(
            &token,
            Token::new_dynamic(new_text, token.text_trimmed_range()).into(),
        ))
    }
}

fn clean_jsx_text(text: &str) -> String {
    let words_vec: Vec<_> = text.split_ascii_whitespace().collect();
    let mut words = words_vec.join(" ");
    let mut chars = text.chars();
    if let Some(c) = chars.nth(0) {
        if c.is_ascii_whitespace() {
            words.insert(0, ' ');
        }
    }
    if let Some(c) = chars.last() {
        if c.is_ascii_whitespace() {
            words.push(' ');
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use crate::jsx::auxiliary::text::clean_jsx_text;

    #[test]
    fn clean_jsx_text_works() {
        assert_eq!("Foo", clean_jsx_text("Foo"));
        assert_eq!(" Foo", clean_jsx_text(" Foo"));
        assert_eq!(" Foo", clean_jsx_text("\nFoo"));
        assert_eq!(" Foo", clean_jsx_text("\tFoo"));
        assert_eq!(" Foo", clean_jsx_text("\n \t Foo"));
        assert_eq!(" Foo", clean_jsx_text("\n \t \n \t\nFoo"));
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

use crate::context::{QuoteProperties, QuoteStyle};
use crate::prelude::*;
use crate::utils::string_utils::CharSignal::AlreadyPrinted;
use rome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL;
use rome_js_syntax::{JsSyntaxToken, SourceType};
use std::borrow::Cow;
use unicode_width::UnicodeWidthStr;

pub trait ToAsciiLowercaseCow {
    /// Returns the same value as String::to_lowercase. The only difference
    /// is that this functions returns ```Cow``` and does not allocate
    /// if the string is already in lowercase.
    fn to_ascii_lowercase_cow(&self) -> Cow<str>;
}

impl ToAsciiLowercaseCow for str {
    fn to_ascii_lowercase_cow(&self) -> Cow<str> {
        debug_assert!(self.is_ascii());

        let bytes = self.as_bytes();

        for idx in 0..bytes.len() {
            let chr = bytes[idx];
            if chr != chr.to_ascii_lowercase() {
                let mut s = bytes.to_vec();
                for b in &mut s[idx..] {
                    b.make_ascii_lowercase();
                }
                return Cow::Owned(unsafe { String::from_utf8_unchecked(s) });
            }
        }

        Cow::Borrowed(self)
    }
}

impl ToAsciiLowercaseCow for String {
    #[inline(always)]
    fn to_ascii_lowercase_cow(&self) -> Cow<str> {
        self.as_str().to_ascii_lowercase_cow()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum StringLiteralParentKind {
    /// Variant to track tokens that are inside an expression
    Expression,
    /// Variant to track tokens that are inside a member
    Member,
    /// Variant used when the string literal is inside a directive. This will apply
    /// a simplified logic of normalisation
    Directive,
}

/// Data structure of convenience to format string literals
pub(crate) struct FormatLiteralStringToken<'token> {
    /// The current token
    token: &'token JsSyntaxToken,

    /// The parent that holds the token
    parent_kind: StringLiteralParentKind,
}

impl<'token> FormatLiteralStringToken<'token> {
    pub fn new(token: &'token JsSyntaxToken, parent_kind: StringLiteralParentKind) -> Self {
        Self { token, parent_kind }
    }

    fn token(&self) -> &'token JsSyntaxToken {
        self.token
    }

    pub fn clean_text(&self, context: &JsFormatContext) -> CleanedStringLiteralText {
        let token = self.token();
        debug_assert_eq!(token.kind(), JS_STRING_LITERAL);

        let chosen_quote_style = context.quote_style();
        let chosen_quote_properties = context.quote_properties();

        let mut string_cleaner =
            LiteralStringNormaliser::new(self, chosen_quote_style, chosen_quote_properties);

        let content = string_cleaner.normalise_text(context.source_type().into());
        let normalized_text_width = content.width();

        CleanedStringLiteralText {
            text: content,
            width: normalized_text_width,
            token,
        }
    }
}

pub struct CleanedStringLiteralText<'a> {
    token: &'a JsSyntaxToken,
    text: Cow<'a, str>,
    width: usize,
}

impl CleanedStringLiteralText<'_> {
    pub fn width(&self) -> usize {
        self.width
    }
}

impl Format<JsFormatContext> for CleanedStringLiteralText<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        format_replaced(
            self.token,
            &syntax_token_cow_slice(
                self.text.clone(),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
        )
        .fmt(f)
    }
}

impl Format<JsFormatContext> for FormatLiteralStringToken<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let cleaned = self.clean_text(f.context());

        cleaned.fmt(f)
    }
}

/// Data structure of convenience to store some information about the
/// string that has been processed
struct StringInformation {
    /// This is the quote that the is calculated and eventually used inside the string.
    /// It could be different from the one inside the formatter options
    preferred_quote: QuoteStyle,
    /// It flags if the raw content has quotes (single or double). The raw content is the
    /// content of a string literal without the quotes
    raw_content_has_quotes: bool,
}

impl FormatLiteralStringToken<'_> {
    /// This function determines which quotes should be used inside to enclose the string.
    /// The function take as a input the string **without quotes**.
    ///
    /// # How it works
    ///
    /// The function determines the preferred quote and alternate quote.
    /// The preferred quote is the one that comes from the formatter options. The alternate quote is the other one.
    ///
    /// We check how many preferred quotes we have inside the content. If this number is greater then the
    /// number alternate quotes that we have inside the content,
    /// then we swap them, so we can reduce the number of escaped quotes.
    ///
    /// For example, let's suppose that the preferred quote is double, and we have a string like this:
    /// ```js
    /// (" content \"\"\" don't ")
    /// ```
    /// Excluding the quotes at the start and beginning, we have three double quote and one single quote.
    /// If we decided to keep them like this, we would have three escaped quotes.
    ///
    /// But then, we choose the single quote as preferred quote and we would have only one quote that is escaped,
    /// resulting into a string like this:
    /// ```js
    /// (' content """ dont\'t ')
    /// ```
    /// Like this, we reduced the number of escaped quotes.
    fn compute_string_information(&self, chosen_quote: QuoteStyle) -> StringInformation {
        let literal = self.token().text_trimmed();
        let alternate = chosen_quote.other();

        let char_count = literal.chars().count();

        let (preferred_quotes_count, alternate_quotes_count) = literal.chars().enumerate().fold(
            (0, 0),
            |(preferred_quotes_counter, alternate_quotes_counter), (index, current_character)| {
                if index == 0 || index == char_count - 1 {
                    (preferred_quotes_counter, alternate_quotes_counter)
                } else if current_character == chosen_quote.as_char() {
                    (preferred_quotes_counter + 1, alternate_quotes_counter)
                } else if current_character == alternate.as_char() {
                    (preferred_quotes_counter, alternate_quotes_counter + 1)
                } else {
                    (preferred_quotes_counter, alternate_quotes_counter)
                }
            },
        );

        StringInformation {
            raw_content_has_quotes: preferred_quotes_count > 0 || alternate_quotes_count > 0,
            preferred_quote: if preferred_quotes_count > alternate_quotes_count {
                alternate
            } else {
                chosen_quote
            },
        }
    }
}

/// This signal is used to tell to the next character what it should do
#[derive(Eq, PartialEq)]
enum CharSignal {
    /// There hasn't been any signal
    None,
    /// The function decided to keep the previous character
    Keep,
    /// The function has decided to print the character. Saves the character that was
    /// already written
    AlreadyPrinted(char),
}

/// Struct of convenience used to manipulate the string. It saves some state in order to apply
/// the normalise process.
struct LiteralStringNormaliser<'token> {
    /// The current token
    token: &'token FormatLiteralStringToken<'token>,
    /// The quote that was set inside the configuration
    chosen_quote_style: QuoteStyle,
    /// When properties in objects are quoted that was set inside the configuration
    chosen_quote_properties: QuoteProperties,
}

/// Convenience enum to map [rome_js_syntax::SourceType] by just reading
/// the type of file
#[derive(Eq, PartialEq)]
pub(crate) enum SourceFileKind {
    TypeScript,
    JavaScript,
}

impl From<SourceType> for SourceFileKind {
    fn from(st: SourceType) -> Self {
        if st.language().is_typescript() {
            Self::TypeScript
        } else {
            Self::JavaScript
        }
    }
}

impl<'token> LiteralStringNormaliser<'token> {
    pub fn new(
        token: &'token FormatLiteralStringToken<'_>,
        chosen_quote_style: QuoteStyle,
        chosen_quote_properties: QuoteProperties,
    ) -> Self {
        Self {
            token,
            chosen_quote_style,
            chosen_quote_properties,
        }
    }

    fn normalise_text(&mut self, file_source: SourceFileKind) -> Cow<'token, str> {
        let string_information = self
            .token
            .compute_string_information(self.chosen_quote_style);
        match self.token.parent_kind {
            StringLiteralParentKind::Expression => {
                self.normalise_string_literal(string_information)
            }
            StringLiteralParentKind::Directive => self.normalise_directive(&string_information),
            StringLiteralParentKind::Member => {
                self.normalise_type_member(string_information, file_source)
            }
        }
    }

    fn get_token(&self) -> &'token JsSyntaxToken {
        self.token.token()
    }

    /// A string should be manipulated only if its raw content contains backslash or quotes
    fn should_manipulate_string(&self, string_information: &StringInformation) -> bool {
        let preferred_quote = string_information.preferred_quote;
        let alternate_quote = preferred_quote.other();
        let raw_content = self.raw_content();
        raw_content.contains(['\\', preferred_quote.as_char(), alternate_quote.as_char()])
            || !matches!(self.token.parent_kind, StringLiteralParentKind::Directive)
    }

    fn normalise_directive(&mut self, string_information: &StringInformation) -> Cow<'token, str> {
        let content = self.normalize_string(string_information);
        match content {
            Cow::Borrowed(content) => self.swap_quotes(content, string_information),
            Cow::Owned(content) => Cow::Owned(
                self.swap_quotes(content.as_ref(), string_information)
                    .into_owned(),
            ),
        }
    }

    /// We can change the text only if there are alphanumeric or alphabetic characters, depending on the file source
    fn can_remove_quotes(&self, file_source: SourceFileKind) -> bool {
        if self.chosen_quote_properties == QuoteProperties::Preserve {
            return false;
        }

        let text_to_check = self.raw_content();
        // Text here is quoteless. If it's empty, it means it is an empty string and we can't
        // do any transformation
        if text_to_check.is_empty() {
            return false;
        }

        let mut has_seen_number = false;
        text_to_check.chars().enumerate().all(|(index, c)| {
            if index == 0 && c.is_numeric() {
                // In TypeScript, numbers like members have different meaning from numbers.
                // Hence, if we see a number, we bail straightaway
                if file_source == SourceFileKind::TypeScript {
                    return false;
                } else {
                    has_seen_number = true;
                }
            }

            let is_eligible_character = if has_seen_number {
                // as we've seen a number, now eligible characters can only contain numbers
                c.is_numeric()
            } else {
                c.is_alphanumeric()
            };
            is_eligible_character || matches!(c, '_' | '$')
        })
    }

    fn normalise_type_member(
        &mut self,
        string_information: StringInformation,
        file_source: SourceFileKind,
    ) -> Cow<'token, str> {
        if self.can_remove_quotes(file_source) {
            return Cow::Owned(self.raw_content().to_string());
        }
        self.normalise_string_literal(string_information)
    }

    fn normalise_string_literal(&self, string_information: StringInformation) -> Cow<'token, str> {
        let preferred_quote = string_information.preferred_quote;
        let polished_raw_content = self.normalize_string(&string_information);

        match polished_raw_content {
            Cow::Borrowed(raw_content) => {
                let final_content = self.swap_quotes(raw_content, &string_information);
                match final_content {
                    Cow::Borrowed(final_content) => Cow::Borrowed(final_content),
                    Cow::Owned(final_content) => Cow::Owned(final_content),
                }
            }
            Cow::Owned(s) => {
                // content is owned, meaning we allocated a new string,
                // so we force replacing quotes, regardless
                let final_content = std::format!(
                    "{}{}{}",
                    preferred_quote.as_char(),
                    s.as_str(),
                    preferred_quote.as_char()
                );

                Cow::Owned(final_content)
            }
        }
    }

    /// This function is responsible of:
    ///
    /// - reducing the number of escapes
    /// - normalising the new lines
    ///
    /// # Escaping
    ///
    /// The way it works is the following: we split the content by analyzing all the
    /// characters that are contained inside [CHARACTERS_THAT_COULD_KEEP_THE_ESCAPE].
    ///
    /// Each time we retrieve one of this character, we push inside a new string all the content
    /// found **before** the current character.
    ///
    /// After that the function checks if the current character should be also be printed or not.
    /// These characters (like quotes) can have an escape that might be removed. If that happens,
    /// we use [CharSignal] to tell to the next iteration what it should do with that character.
    ///
    /// For example, let's take this example:
    /// ```js
    /// ("hello! \'")
    /// ```
    ///
    /// Here, we want to remove the backslash (\) from the content. So when we encounter `\`,
    /// the algorithm checks if after `\` there's a `'`, and if so, then we push inside the final string
    /// only `'` and we ignore the backlash. Then we signal the next iteration with [CharSignal::AlreadyPrinted],
    /// so when we process the next `'`, we decide to ignore it and reset the signal.
    ///
    /// Another example is the following:
    ///
    /// ```js
    /// (" \\' ")
    /// ```
    ///
    /// Here, we need to keep all the backslash. We check the first one and we look ahead. We find another
    /// `\`, so we keep it the first and we signal the next iteration with [CharSignal::Keep].
    /// Then the next iteration comes along. We have the second `\`, we look ahead we find a `'`. Although,
    /// as opposed to the previous example, we have a signal that says that we should keep the current
    /// character. Then we do so. The third iteration comes along and we find `'`. We still have the
    /// [CharSignal::Keep]. We do so and then we set the signal to [CharSignal::Idle]
    ///
    /// # Newlines
    ///
    /// By default the formatter uses `\n` as a newline. The function replaces
    /// `\r\n` with `\n`,
    fn normalize_string(&self, string_information: &StringInformation) -> Cow<'token, str> {
        let raw_content = self.raw_content();

        if !self.should_manipulate_string(string_information) {
            return Cow::Borrowed(raw_content);
        }
        let preferred_quote = string_information.preferred_quote;
        let alternate_quote = preferred_quote.other();
        let mut reduced_string = String::new();
        let mut signal = CharSignal::None;

        let mut chars = raw_content.char_indices().peekable();

        while let Some((_, current_char)) = chars.next() {
            let next_character = chars.peek();

            if let AlreadyPrinted(char) = signal {
                if char == current_char {
                    continue;
                }
            }

            match current_char {
                '\\' => {
                    let bytes = raw_content.as_bytes();

                    if let Some((next_index, next_character)) = next_character {
                        // If we encounter an alternate quote that is escaped, we have to
                        // remove the escape from it.
                        // This is done because of how the enclosed strings can change.
                        // Check `computed_preferred_quote` for more details.
                        if *next_character as u8 == alternate_quote.as_bytes()
                                    // This check is a safety net for cases where the backslash is at the end
                                    // of the raw content:
                                    // ("\\")
                                    // The second backslash is at the end.
                                    && *next_index < bytes.len()
                        {
                            match signal {
                                CharSignal::Keep => {
                                    reduced_string.push(current_char);
                                }
                                _ => {
                                    reduced_string.push(alternate_quote.as_char());
                                    signal = AlreadyPrinted(alternate_quote.as_char());
                                }
                            }
                        } else if signal == CharSignal::Keep {
                            reduced_string.push(current_char);
                        }
                        // The next character is another backslash, or
                        // a character that should be kept in the next iteration
                        else if matches!(
                            next_character,
                            '\\' | 'v' | 'b' | 'f' | 'n' | 't' | 'r' | 'u' | 'x'
                        ) {
                            signal = CharSignal::Keep;
                            // fallback, keep the backslash
                            reduced_string.push(current_char);
                        }
                        // these are character that should stay, but
                        // the next iteration should decide if to keep them or not
                        else if !next_character.is_alphabetic()
                            && *next_character != alternate_quote.as_char()
                            && *next_character != preferred_quote.as_char()
                        {
                            reduced_string.push(current_char);
                        } else {
                            // these, usually characters that can have their
                            // escape removed: "\a" => "a"
                            // So we ignore the current slash and we continue
                            // to the next iteration
                            continue;
                        }
                    } else {
                        // fallback, keep the backslash
                        reduced_string.push(current_char);
                    }
                }
                '\n' | '\t' => {
                    if let AlreadyPrinted(the_char) = signal {
                        if matches!(the_char, '\n' | '\t') {
                            signal = CharSignal::None
                        }
                    } else {
                        reduced_string.push(current_char);
                    }
                }
                // If the current character is \r and the
                // next is \n, skip over the entire sequence
                '\r' if next_character.map_or(false, |(_, c)| *c == '\n') => {
                    reduced_string.push('\n');
                    signal = AlreadyPrinted('\n');
                }
                _ => {
                    // If we encounter a preferred quote and it's not escaped, we have to replace it with
                    // an escaped version.
                    // This is done because of how the enclosed strings can change.
                    // Check `computed_preferred_quote` for more details.
                    if current_char == preferred_quote.as_char() {
                        let last_char = &reduced_string.chars().last();
                        if let Some('\\') = last_char {
                            reduced_string.push(preferred_quote.as_char());
                        } else {
                            reduced_string.push_str(preferred_quote.as_escaped());
                        }
                    } else if current_char == alternate_quote.as_char() {
                        match signal {
                            CharSignal::None => {
                                reduced_string.push(alternate_quote.as_char());
                            }
                            CharSignal::Keep => {
                                reduced_string.push(alternate_quote.as_char());
                                signal = CharSignal::None;
                            }
                            AlreadyPrinted(the_char) => {
                                if the_char == alternate_quote.as_char() {
                                    signal = CharSignal::None;
                                    continue;
                                }
                            }
                        }
                    } else {
                        reduced_string.push(current_char);
                    }
                }
            }
        }

        // Don't allocate a new string of this is empty
        if reduced_string.is_empty() {
            Cow::Borrowed(raw_content)
        } else {
            // don't allocate a new string if the new string is still equals to the input string
            if reduced_string == raw_content {
                Cow::Borrowed(raw_content)
            } else {
                Cow::Owned(reduced_string)
            }
        }
    }

    fn raw_content(&self) -> &'token str {
        let content = self.get_token().text_trimmed();
        &content[1..content.len() - 1]
    }

    fn swap_quotes<S: Into<&'token str>>(
        &self,
        content_to_use: S,
        string_information: &StringInformation,
    ) -> Cow<'token, str> {
        let original_content = self.get_token().text_trimmed();
        let preferred_quote = string_information.preferred_quote;
        let other_quote = preferred_quote.other().as_char();

        let raw_content_has_quotes = string_information.raw_content_has_quotes;

        if raw_content_has_quotes {
            Cow::Borrowed(original_content)
        } else if original_content.starts_with(other_quote) {
            Cow::Owned(std::format!(
                "{}{}{}",
                preferred_quote.as_char(),
                content_to_use.into(),
                preferred_quote.as_char()
            ))
        } else {
            Cow::Borrowed(original_content)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::QuoteStyle;
    use crate::utils::quickcheck_utils::*;
    use crate::utils::FormatLiteralStringToken;
    use quickcheck_macros::*;
    use rome_js_factory::JsSyntaxTreeBuilder;
    use rome_js_syntax::JsSyntaxKind::{JS_STRING_LITERAL, JS_STRING_LITERAL_EXPRESSION};
    use rome_js_syntax::{JsStringLiteralExpression, JsSyntaxToken};
    use rome_rowan::AstNode;
    use std::borrow::Cow;

    #[quickcheck]
    fn to_ascii_lowercase_cow_always_returns_same_value_as_string_to_lowercase(txt: AsciiString) {
        assert_eq!(
            txt.to_lowercase(),
            txt.to_ascii_lowercase_cow().into_owned()
        );
    }

    #[quickcheck]
    fn to_ascii_lowercase_cow_returns_borrowed_when_all_chars_are_lowercase(txt: AsciiString) {
        let txt = txt.to_lowercase();
        assert!(matches!(txt.to_ascii_lowercase_cow(), Cow::Borrowed(s) if s == txt));
    }

    #[quickcheck]
    fn to_ascii_lowercase_cow_returns_owned_when_some_chars_are_not_lowercase(txt: AsciiString) {
        let txt = std::format!("{}A", txt); //guarantees at least one uppercase letter
        assert!(matches!(txt.to_ascii_lowercase_cow(), Cow::Owned(s) if s == txt.to_lowercase()));
    }

    fn generate_syntax_token(input: &str) -> JsSyntaxToken {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        tree_builder.start_node(JS_STRING_LITERAL_EXPRESSION);
        tree_builder.token(JS_STRING_LITERAL, input);
        tree_builder.finish_node();

        let root = tree_builder.finish();

        JsStringLiteralExpression::cast(root)
            .unwrap()
            .value_token()
            .unwrap()
    }

    enum AsToken {
        Directive,
        String,
        Member,
    }

    impl AsToken {
        fn into_token(self, token: &JsSyntaxToken) -> FormatLiteralStringToken {
            match self {
                AsToken::Directive => {
                    FormatLiteralStringToken::new(token, StringLiteralParentKind::Directive)
                }
                AsToken::String => {
                    FormatLiteralStringToken::new(token, StringLiteralParentKind::Expression)
                }
                AsToken::Member => {
                    FormatLiteralStringToken::new(token, StringLiteralParentKind::Member)
                }
            }
        }
    }

    fn assert_borrowed_token(
        input: &str,
        quote: QuoteStyle,
        quote_properties: QuoteProperties,
        as_token: AsToken,
        source: SourceFileKind,
    ) {
        let token = generate_syntax_token(input);
        let string_token = as_token.into_token(&token);
        let mut string_cleaner =
            LiteralStringNormaliser::new(&string_token, quote, quote_properties);
        let content = string_cleaner.normalise_text(source);
        assert_eq!(content, Cow::Borrowed(input))
    }

    fn assert_owned_token(
        input: &str,
        output: &str,
        quote: QuoteStyle,
        quote_properties: QuoteProperties,
        as_token: AsToken,
        source: SourceFileKind,
    ) {
        let token = generate_syntax_token(input);
        let string_token = as_token.into_token(&token);
        let mut string_cleaner =
            LiteralStringNormaliser::new(&string_token, quote, quote_properties);
        let content = string_cleaner.normalise_text(source);
        let owned: Cow<str> = Cow::Owned(output.to_string());
        assert_eq!(content, owned)
    }

    #[test]
    fn string_borrowed() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [r#""content""#, r#""content with single ' quote ""#];
        for input in inputs {
            assert_borrowed_token(
                input,
                quote,
                quote_properties,
                AsToken::String,
                SourceFileKind::JavaScript,
            )
        }
    }

    #[test]
    fn string_owned() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [
            (r#"" content '' \"\"\" ""#, r#"' content \'\' """ '"#),
            (r#"" content \"\"\"\" '' ""#, r#"' content """" \'\' '"#),
            (r#"" content ''''' \" ""#, r#"" content ''''' \" ""#),
            (r#"" content \'\' \" ""#, r#"" content '' \" ""#),
            (r#"" content \\' \" ""#, r#"" content \\' \" ""#),
        ];
        for (input, output) in inputs {
            assert_owned_token(
                input,
                output,
                quote,
                quote_properties,
                AsToken::String,
                SourceFileKind::JavaScript,
            )
        }
    }

    #[test]
    fn directive_borrowed() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [r#""use strict '""#];
        for input in inputs {
            assert_borrowed_token(
                input,
                quote,
                quote_properties,
                AsToken::Directive,
                SourceFileKind::JavaScript,
            )
        }
    }

    #[test]
    fn directive_owned() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [(r#"' use strict '"#, r#"" use strict ""#)];
        for (input, output) in inputs {
            assert_owned_token(
                input,
                output,
                quote,
                quote_properties,
                AsToken::Directive,
                SourceFileKind::JavaScript,
            )
        }
    }

    #[test]
    fn member_borrowed() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [r#""cant @ be moved""#, r#""1674""#, r#""33n""#];
        for input in inputs {
            assert_borrowed_token(
                input,
                quote,
                quote_properties,
                AsToken::Member,
                SourceFileKind::TypeScript,
            )
        }
    }

    #[test]
    fn member_owned() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [(r#""string""#, r#"string"#)];
        for (input, output) in inputs {
            assert_owned_token(
                input,
                output,
                quote,
                quote_properties,
                AsToken::Member,
                SourceFileKind::TypeScript,
            )
        }
    }
}

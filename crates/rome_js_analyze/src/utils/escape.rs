pub fn escape<'a>(
    unescaped_string: &'a str,
    needs_escaping: &[&str],
    escaping_char: char,
) -> std::borrow::Cow<'a, str> {
    let mut escaped = String::new();
    let escaping_char_len = escaping_char.len_utf8();
    let mut iter = unescaped_string.char_indices();
    let mut escape_count = 0;
    'unescaped: while let Some((idx, chr)) = iter.next() {
        if chr == escaping_char {
            escape_count += 1;
        }

        'candidates: for candidate in needs_escaping {
            if escape_count % 2 != 0
                && unescaped_string[idx + escaping_char_len..].starts_with(candidate)
            {
                for _ in candidate.chars() {
                    let _ = iter.next();
                }
                escape_count = 0;
                continue 'candidates;
            }

            if unescaped_string[idx..].starts_with(candidate) {
                if escaped.is_empty() {
                    escaped = String::with_capacity(unescaped_string.len() * 2);
                    escaped.push_str(&unescaped_string[0..idx]);
                }
                escaped.push(escaping_char);
                escaped.push_str(candidate);
                for _ in candidate.chars().skip(1) {
                    let _ = iter.next();
                }
                continue 'unescaped;
            }
        }

        if !escaped.is_empty() {
            escaped.push(chr);
        }
    }

    if escaped.is_empty() {
        std::borrow::Cow::Borrowed(unescaped_string)
    } else {
        std::borrow::Cow::Owned(escaped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_escape_dollar_signs_and_backticks() {
        assert_eq!(escape("abc", &["${"], '\\'), "abc");
        assert_eq!(escape("abc $ bca", &["${"], '\\'), "abc $ bca");
        assert_eq!(escape("abc ${a} bca", &["${"], '\\'), r#"abc \${a} bca"#);
        assert_eq!(
            escape("abc ${} ${} bca", &["${"], '\\'),
            r#"abc \${} \${} bca"#
        );

        assert_eq!(escape(r#"\`"#, &["`"], '\\'), r#"\`"#);
        assert_eq!(escape(r#"\${}"#, &["${"], '\\'), r#"\${}"#);
        assert_eq!(escape(r#"\\`"#, &["`"], '\\'), r#"\\\`"#);
        assert_eq!(escape(r#"\\${}"#, &["${"], '\\'), r#"\\\${}"#);
        assert_eq!(escape(r#"\\\`"#, &["`"], '\\'), r#"\\\`"#);
        assert_eq!(escape(r#"\\\${}"#, &["${"], '\\'), r#"\\\${}"#);

        assert_eq!(escape("abc", &["${", "`"], '\\'), "abc");
        assert_eq!(escape("${} `", &["${", "`"], '\\'), r#"\${} \`"#);
        assert_eq!(
            escape(r#"abc \${a} \`bca"#, &["${", "`"], '\\'),
            r#"abc \${a} \`bca"#
        );
        assert_eq!(
            escape(r#"abc \${bca}"#, &["${", "`"], '\\'),
            r#"abc \${bca}"#
        );
        assert_eq!(escape(r#"abc \`bca"#, &["${", "`"], '\\'), r#"abc \`bca"#);
    }
}

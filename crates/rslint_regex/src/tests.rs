//! in part taken from <https://github.com/denoland/deno_lint/blob/master/src/js_regex/mod.rs>

use crate::unicode::EcmaVersion;

struct EcmaRegexValidator {
    ecma_version: EcmaVersion,
}

impl EcmaRegexValidator {
    pub fn new(ecma_version: EcmaVersion) -> Self {
        Self { ecma_version }
    }

    pub fn validate_pattern(&self, pattern: &str, u_flag: bool) -> Result<(), String> {
        let mut pat = String::from("/");
        pat.push_str(pattern);
        pat.push('/');
        if u_flag {
            pat.push('u');
        }
        let parser = crate::Parser::new(&pat, 0, 0, self.ecma_version, u_flag).unwrap();
        parser.parse().map(|_| ()).map_err(|e| {
            let offset = e.span.abs_end();
            format!("{} at position {}", e.message, offset)
        })
    }
}

#[test]
fn validate_pattern_test() {
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_eq!(validator.validate_pattern("", false), Ok(()));
    assert_eq!(validator.validate_pattern("(2)\\1", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\d{1}", false), Ok(()));
    assert_eq!(validator.validate_pattern("[abc]de|fg", false), Ok(()));
    assert_eq!(validator.validate_pattern("[abc]de|fg", true), Ok(()));
    assert_eq!(validator.validate_pattern("^.$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^.$", true), Ok(()));
    assert_eq!(validator.validate_pattern("foo\\[bar", false), Ok(()));
    assert_eq!(validator.validate_pattern("foo\\[bar", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\w+\\s", false), Ok(()));
    assert_eq!(validator.validate_pattern("(\\w+), (\\w+)", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("\\/\\/.*|\\/\\*[^]*\\*\\/", false),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("(\\d{1,2})", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("(?:\\d{3}|\\(\\d{3}\\))([-\\/\\.])\\d{3}\\1\\d{4}", false),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("https?:\\/\\/(www\\.)?[-a-zA-Z0-9@:%._\\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b([-a-zA-Z0-9()@:%_\\+.~#?&//=]*)", false), Ok(()));

    assert_eq!(
        validator.validate_pattern("\\p{Script=Greek}", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("\\p{Alphabetic}", true), Ok(()));

    assert_ne!(validator.validate_pattern("\\", false), Ok(()));
    assert_ne!(validator.validate_pattern("a**", false), Ok(()));
    assert_ne!(validator.validate_pattern("++a", false), Ok(()));
    assert_ne!(validator.validate_pattern("?a", false), Ok(()));
    assert_ne!(validator.validate_pattern("a***", false), Ok(()));
    assert_ne!(validator.validate_pattern("a++", false), Ok(()));
    assert_ne!(validator.validate_pattern("a+++", false), Ok(()));
    assert_ne!(validator.validate_pattern("a???", false), Ok(()));
    assert_ne!(validator.validate_pattern("a????", false), Ok(()));
    assert_ne!(validator.validate_pattern("*a", false), Ok(()));
    assert_ne!(validator.validate_pattern("**a", false), Ok(()));
    assert_ne!(validator.validate_pattern("+a", false), Ok(()));
    assert_ne!(validator.validate_pattern("[{-z]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[a--z]", false), Ok(()));

    assert_ne!(validator.validate_pattern("0{2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("x{1}{1,}", false), Ok(()));
    assert_ne!(validator.validate_pattern("x{1,2}{1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("x{1,}{1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("x{0,1}{1,}", false), Ok(()));

    assert_ne!(validator.validate_pattern("\\1(\\P{P\0[}()/", true), Ok(()));
}

#[test]
fn character_range_order() {
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("^[z-a]$", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-ac-e]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[c-eb-a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[a-dc-b]", false), Ok(()));

    assert_ne!(validator.validate_pattern("[\\10b-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\ad-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\bd-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\Bd-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\db-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\Db-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\sb-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\Sb-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\wb-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\Wb-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\0b-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\td-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\nd-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\vd-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\fd-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\rd-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c0001d-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\x0061d-G]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0061d-G]", false), Ok(()));

    assert_ne!(validator.validate_pattern("[b-G\\10]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\b]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\B]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-G\\d]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-G\\D]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-G\\s]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-G\\S]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-G\\w]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-G\\W]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-G\\0]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\t]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\n]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\v]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\f]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\r]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\c0001]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\x0061]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[d-G\\u0061]", false), Ok(()));
}

#[test]
fn unicode_quantifier_without_atom() {
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("*", true), Ok(()));
    assert_ne!(validator.validate_pattern("+", true), Ok(()));
    assert_ne!(validator.validate_pattern("?", true), Ok(()));
    assert_ne!(validator.validate_pattern("{1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("{1,}", true), Ok(()));
    assert_ne!(validator.validate_pattern("{1,2}", true), Ok(()));

    assert_ne!(validator.validate_pattern("*?", true), Ok(()));
    assert_ne!(validator.validate_pattern("+?", true), Ok(()));
    assert_ne!(validator.validate_pattern("??", true), Ok(()));
    assert_ne!(validator.validate_pattern("{1}?", true), Ok(()));
    assert_ne!(validator.validate_pattern("{1,}?", true), Ok(()));
    assert_ne!(validator.validate_pattern("{1,2}?", true), Ok(()));
}

#[test]
fn unicode_incomplete_quantifier() {
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("a{", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,2", true), Ok(()));

    assert_ne!(validator.validate_pattern("{", true), Ok(()));
    assert_ne!(validator.validate_pattern("{1", true), Ok(()));
    assert_ne!(validator.validate_pattern("{1,", true), Ok(()));
    assert_ne!(validator.validate_pattern("{1,2", true), Ok(()));
}

#[test]
fn unicode_single_bracket() {
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("(", true), Ok(()));
    assert_ne!(validator.validate_pattern(")", true), Ok(()));
    assert_ne!(validator.validate_pattern("[", true), Ok(()));
    assert_ne!(validator.validate_pattern("]", true), Ok(()));
    assert_ne!(validator.validate_pattern("{", true), Ok(()));
    assert_ne!(validator.validate_pattern("}", true), Ok(()));
}

#[test]
fn unicode_escapes() {
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_eq!(validator.validate_pattern("\\u{10ffff}", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u{110000}", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{110000}", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("foo\\ud803\\ude6dbar", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(\u{12345}|\u{23456}).\\1", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("\u{12345}{3}", true), Ok(()));

    // unicode escapes in character classes
    assert_eq!(
        validator.validate_pattern("[\\u0062-\\u0066]oo", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("[\\u0062-\\u0066]oo", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("[\\u{0062}-\\u{0066}]oo", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("[\\u{62}-\\u{00000066}]oo", true),
        Ok(())
    );

    // invalid escapes
    assert_eq!(
        validator.validate_pattern("first\\u\\x\\z\\8\\9second", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("[\\u\\x\\z\\8\\9]", false),
        Ok(())
    );
    assert_ne!(validator.validate_pattern("/\\u/u", true), Ok(()));
    assert_ne!(validator.validate_pattern("/\\u12/u", true), Ok(()));
    assert_ne!(validator.validate_pattern("/\\ufoo/u", true), Ok(()));
    assert_ne!(validator.validate_pattern("/\\x/u", true), Ok(()));
    assert_ne!(validator.validate_pattern("/\\xfoo/u", true), Ok(()));
    assert_ne!(validator.validate_pattern("/\\z/u", true), Ok(()));
    assert_ne!(validator.validate_pattern("/\\8/u", true), Ok(()));
    assert_ne!(validator.validate_pattern("/\\9/u", true), Ok(()));
}

#[test]
fn basic_valid() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/visitor/full.json
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_eq!(validator.validate_pattern("foo", false), Ok(()));
    assert_eq!(validator.validate_pattern("foo|bar", false), Ok(()));
    assert_eq!(validator.validate_pattern("||||", false), Ok(()));
    assert_eq!(validator.validate_pattern("^|$|\\b|\\B", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=)", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=foo)", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?!)", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?!foo)", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=a)*", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=a)+", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=a)?", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=a){", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=a){}", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=a){a}", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=a){1}", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=a){1,}", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?=a){1,2}", false), Ok(()));
    assert_eq!(validator.validate_pattern("a*", false), Ok(()));
    assert_eq!(validator.validate_pattern("a+", false), Ok(()));
    assert_eq!(validator.validate_pattern("a?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{}", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{a}", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1}", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,}", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,2}", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,2", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{2,1", false), Ok(()));
    assert_eq!(validator.validate_pattern("a*?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a+?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a??", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{}?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{a}?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1}?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,}?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,2}?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,2?", false), Ok(()));
    assert_eq!(validator.validate_pattern("a{2,1?", false), Ok(()));
    assert_eq!(validator.validate_pattern("👍🚀❇️", false), Ok(()));
    assert_eq!(validator.validate_pattern("^", false), Ok(()));
    assert_eq!(validator.validate_pattern("$", false), Ok(()));
    assert_eq!(validator.validate_pattern(".", false), Ok(()));
    assert_eq!(validator.validate_pattern("]", false), Ok(()));
    assert_eq!(validator.validate_pattern("{", false), Ok(()));
    assert_eq!(validator.validate_pattern("}", false), Ok(()));
    assert_eq!(validator.validate_pattern("|", false), Ok(()));
    assert_eq!(validator.validate_pattern("${1,2", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\1", false), Ok(()));
    assert_eq!(validator.validate_pattern("(a)\\1", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\1(a)", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?:a)\\1", false), Ok(()));
    assert_eq!(validator.validate_pattern("(a)\\2", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?:a)\\2", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("(a)(a)(a)(a)(a)(a)(a)(a)(a)(a)\\10", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(a)(a)(a)(a)(a)(a)(a)(a)(a)(a)\\11", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(a)(a)(a)(a)(a)(a)(a)(a)(a)(a)(a)\\11", false),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("(?:a)", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\d", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\D", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\s", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\S", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\w", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\W", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\f", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\n", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\r", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\t", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\v", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\cA", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\cz", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\c1", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\c", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\0", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u1", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u12", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u123", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u1234", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u12345", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{z", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{a}", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{20", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{20}", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{10FFFF}", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{110000}", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{00000001}", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\377", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\400", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\^", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\$", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\.", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\+", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\?", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\(", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\)", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\[", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\]", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\{", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\}", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\|", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\/", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\a", false), Ok(()));
    assert_eq!(validator.validate_pattern("[]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[^-a-b-]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[-]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[a]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[--]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[-a]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[-a-]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[a-]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[a-b]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[-a-b-]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[---]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[a-b--/]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\b-\\n]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[b\\-a]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\d]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\D]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\s]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\S]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\w]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\W]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\f]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\n]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\r]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\t]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\v]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\cA]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\cz]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\c1]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\c]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\0]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\x]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\xz]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\x1]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\x12]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\x123]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u1]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u12]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u123]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u1234]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u12345]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{z]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{a}]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{20]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{20}]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{10FFFF}]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{110000}]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{00000001}]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\77]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\377]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\400]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\^]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\$]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\.]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\+]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\?]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\(]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\)]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\[]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\]]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\{]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\}]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\|]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\/]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\a]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\d-\\uFFFF]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\D-\\uFFFF]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\s-\\uFFFF]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\S-\\uFFFF]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\w-\\uFFFF]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\W-\\uFFFF]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u0000-\\d]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u0000-\\D]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u0000-\\s]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u0000-\\S]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u0000-\\w]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u0000-\\W]", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("[\\u0000-\\u0001]", false),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("[\\u{2-\\u{1}]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\a-\\z]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[0-9--/]", false), Ok(()));
    assert_eq!(validator.validate_pattern("[\\c_]", false), Ok(()));
    assert_eq!(validator.validate_pattern("^[0-9]*$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^[0-9]+$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^[a-zA-Z]*$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^[a-zA-Z]+$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^[0-9a-zA-Z]*$", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("^[a-zA-Z0-9!-/:-@\\[-`{-~]*$", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("^([a-zA-Z0-9]{8,})$", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("^([a-zA-Z0-9]{6,8})$", false),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("^([0-9]{0,8})$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^[0-9]{8}$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^https?:\\/\\/", false), Ok(()));
    assert_eq!(validator.validate_pattern("^\\d{3}-\\d{4}$", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("^\\d{1,3}(.\\d{1,3}){3}$", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("^([1-9][0-9]*|0)(\\.[0-9]+)?$", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("^-?([1-9][0-9]*|0)(\\.[0-9]+)?$", false),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("^[ぁ-んー]*$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^[ァ-ンヴー]*$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^[ｧ-ﾝﾞﾟ\\-]*$", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("^[^\\x20-\\x7e]*$", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern(
            "^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\\.[a-zA-Z0-9-]+)*$",
            false
        ),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("^((4\\d{3})|(5[1-5]\\d{2})|(6011))([- ])?\\d{4}([- ])?\\d{4}([- ])?\\d{4}|3[4,7]\\d{13}$", false), Ok(()));
    assert_eq!(validator.validate_pattern("^\\s*|\\s*$", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("[\\d][\\12-\\14]{1,}[^\\d]", false),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("([a ]\\b)*\\b", false), Ok(()));
    assert_eq!(validator.validate_pattern("foo", true), Ok(()));
    assert_eq!(validator.validate_pattern("foo|bar", true), Ok(()));
    assert_eq!(validator.validate_pattern("||||", true), Ok(()));
    assert_eq!(validator.validate_pattern("^|$|\\b|\\B", true), Ok(()));
    assert_eq!(validator.validate_pattern("(?=)", true), Ok(()));
    assert_eq!(validator.validate_pattern("(?=foo)", true), Ok(()));
    assert_eq!(validator.validate_pattern("(?!)", true), Ok(()));
    assert_eq!(validator.validate_pattern("(?!foo)", true), Ok(()));
    assert_eq!(validator.validate_pattern("a*", true), Ok(()));
    assert_eq!(validator.validate_pattern("a+", true), Ok(()));
    assert_eq!(validator.validate_pattern("a?", true), Ok(()));
    assert_eq!(validator.validate_pattern("a{1}", true), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,}", true), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,2}", true), Ok(()));
    assert_eq!(validator.validate_pattern("a*?", true), Ok(()));
    assert_eq!(validator.validate_pattern("a+?", true), Ok(()));
    assert_eq!(validator.validate_pattern("a??", true), Ok(()));
    assert_eq!(validator.validate_pattern("a{1}?", true), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,}?", true), Ok(()));
    assert_eq!(validator.validate_pattern("a{1,2}?", true), Ok(()));
    assert_eq!(validator.validate_pattern("👍🚀❇️", true), Ok(()));
    assert_eq!(validator.validate_pattern("^", true), Ok(()));
    assert_eq!(validator.validate_pattern("$", true), Ok(()));
    assert_eq!(validator.validate_pattern(".", true), Ok(()));
    assert_eq!(validator.validate_pattern("|", true), Ok(()));
    assert_eq!(validator.validate_pattern("(a)\\1", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\1(a)", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("(a)(a)(a)(a)(a)(a)(a)(a)(a)(a)\\10", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(a)(a)(a)(a)(a)(a)(a)(a)(a)(a)(a)\\11", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("(?:a)", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\d", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\D", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\s", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\S", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\w", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\W", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\f", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\n", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\r", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\t", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\v", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\cA", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\cz", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\0", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\u1234", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\u12345", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{a}", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{20}", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{10FFFF}", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\u{00000001}", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\^", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\$", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\.", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\+", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\?", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\(", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\)", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\[", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\]", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\{", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\}", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\|", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\/", true), Ok(()));
    assert_eq!(validator.validate_pattern("[]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[^-a-b-]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[-]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[a]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[--]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[-a]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[-a-]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[a-]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[a-b]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[-a-b-]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[---]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[a-b--/]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\b-\\n]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\d]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\D]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\s]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\S]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\w]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\W]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\f]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\n]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\r]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\t]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\v]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\cA]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\cz]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\0]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\x12]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\x123]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u1234]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u12345]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{a}]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{20}]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{10FFFF}]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\u{00000001}]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\^]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\$]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\.]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\+]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\?]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\(]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\)]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\[]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\]]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\{]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\}]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\|]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[\\/]", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("[\\u0000-\\u0001]", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("[\\u{1}-\\u{2}]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[0-9--/]", true), Ok(()));
    assert_eq!(validator.validate_pattern("[🌷-🌸]", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("[\\u0000-🌸-\\u0000]", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("[\\u0000-\\u{1f338}-\\u0000]", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("[\\u0000-\\ud83c\\udf38-\\u0000]", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("[\\uD834\\uDF06-\\uD834\\uDF08a-z]", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("^[0-9]*$", true), Ok(()));
    assert_eq!(validator.validate_pattern("^[0-9]+$", true), Ok(()));
    assert_eq!(validator.validate_pattern("^[a-zA-Z]*$", true), Ok(()));
    assert_eq!(validator.validate_pattern("^[a-zA-Z]+$", true), Ok(()));
    assert_eq!(validator.validate_pattern("^[0-9a-zA-Z]*$", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("^[a-zA-Z0-9!-/:-@\\[-`{-~]*$", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("^([a-zA-Z0-9]{8,})$", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("^([a-zA-Z0-9]{6,8})$", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("^([0-9]{0,8})$", true), Ok(()));
    assert_eq!(validator.validate_pattern("^[0-9]{8}$", true), Ok(()));
    assert_eq!(validator.validate_pattern("^https?:\\/\\/", true), Ok(()));
    assert_eq!(validator.validate_pattern("^\\d{3}-\\d{4}$", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("^\\d{1,3}(.\\d{1,3}){3}$", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("^([1-9][0-9]*|0)(\\.[0-9]+)?$", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("^-?([1-9][0-9]*|0)(\\.[0-9]+)?$", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("^[ぁ-んー]*$", true), Ok(()));
    assert_eq!(validator.validate_pattern("^[ァ-ンヴー]*$", true), Ok(()));
    assert_eq!(validator.validate_pattern("^[ｧ-ﾝﾞﾟ\\-]*$", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("^[^\\x20-\\x7e]*$", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern(
            "^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\\.[a-zA-Z0-9-]+)*$",
            true
        ),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("^((4\\d{3})|(5[1-5]\\d{2})|(6011))([- ])?\\d{4}([- ])?\\d{4}([- ])?\\d{4}|3[4,7]\\d{13}$", true), Ok(()));
    assert_eq!(validator.validate_pattern("^\\s*|\\s*$", true), Ok(()));
    assert_eq!(validator.validate_pattern("(?<=a)", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?<=a)", true), Ok(()));
    assert_eq!(validator.validate_pattern("(?<!a)", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?<!a)", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("(?<=(?<a>\\w){3})f", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("((?<=\\w{3}))f", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("(?<a>(?<=\\w{3}))f", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<!(?<a>\\d){3})f", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<!(?<a>\\D){3})f|f", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<a>(?<!\\D{3}))f|f", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<=(?<a>\\w){3})f", false),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("((?<=\\w{3}))f", false), Ok(()));
    assert_eq!(
        validator.validate_pattern("(?<a>(?<=\\w{3}))f", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<!(?<a>\\d){3})f", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<a>(?<!\\D{3}))f|f", false),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<=(?<fst>.)|(?<snd>.))", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("(a)", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?<a>)", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\k", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\k<a>", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?<a>a)\\k<a>", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?<a>a)\\k<a>", true), Ok(()));
    assert_eq!(validator.validate_pattern("(?<a>a)\\1", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?<a>a)\\1", true), Ok(()));
    assert_eq!(validator.validate_pattern("(?<a>a)\\2", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?<a>a)(?<b>a)", false), Ok(()));
    assert_eq!(validator.validate_pattern("(?<a>a)(?<b>a)", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\k<a>(?<a>a)", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\k<a>(?<a>a)", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\1(?<a>a)", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\1(?<a>a)", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("(?<$abc>a)\\k<$abc>", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("(?<あ>a)\\k<あ>", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("(?<𠮷>a)\\k<\\u{20bb7}>", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<\\uD842\\uDFB7>a)\\k<\\u{20bb7}>", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<\\u{20bb7}>a)\\k<\\uD842\\uDFB7>", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<abc>a)\\k<\\u0061\\u0062\\u0063>", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("(?<\\u0061\\u0062\\u0063>a)\\k<abc>", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern(
            "(?<\\u0061\\u0062\\u0063>a)\\k<\\u{61}\\u{62}\\u{63}>",
            true
        ),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("(?<a1>a)\\k<a1>", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\p", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\p{", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\p{ASCII", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\p{ASCII}", false), Ok(()));
    assert_eq!(validator.validate_pattern("\\p{ASCII}", true), Ok(()));
    assert_eq!(validator.validate_pattern("\\p{Emoji}", true), Ok(()));
    assert_eq!(
        validator.validate_pattern("\\p{General_Category=Letter}", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("\\p{Script=Hiragana}", true),
        Ok(())
    );
    assert_eq!(
        validator.validate_pattern("[\\p{Script=Hiragana}\\-\\p{Script=Katakana}]", true),
        Ok(())
    );
    assert_eq!(validator.validate_pattern("\\P{Letter}", true), Ok(()));
}

#[test]
fn basic_invalid() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/basic-invalid.json
    let validator = EcmaRegexValidator::new(EcmaVersion::ES5);
    assert_ne!(validator.validate_pattern("(", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?=", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?=foo", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?!", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?!foo", false), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("(a{2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(*)", false), Ok(()));
    assert_ne!(validator.validate_pattern("+", false), Ok(()));
    assert_ne!(validator.validate_pattern("?", false), Ok(()));
    assert_ne!(validator.validate_pattern(")", false), Ok(()));
    assert_ne!(validator.validate_pattern("[", false), Ok(()));
    assert_ne!(validator.validate_pattern("^*", false), Ok(()));
    assert_ne!(validator.validate_pattern("$*", false), Ok(()));
    assert_ne!(validator.validate_pattern("${1,2}", false), Ok(()));
    assert_ne!(validator.validate_pattern("${2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("\\2(a)(", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?:", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(:a", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[a-b--+]", false), Ok(()));
    assert_ne!(
        validator.validate_pattern("[\\u0001-\\u0000]", false),
        Ok(())
    );
    assert_ne!(validator.validate_pattern("[\\u{1}-\\u{2}]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{2}-\\u{1}]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\z-\\a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[0-9--+]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c-a]", false), Ok(()));
}

#[test]
fn basic_invalid_2015() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/basic-invalid-2015.json
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2015);
    assert_ne!(validator.validate_pattern("(", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?=", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?=foo", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?!", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?!foo", false), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("(a{2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(*)", false), Ok(()));
    assert_ne!(validator.validate_pattern("+", false), Ok(()));
    assert_ne!(validator.validate_pattern("?", false), Ok(()));
    assert_ne!(validator.validate_pattern(")", false), Ok(()));
    assert_ne!(validator.validate_pattern("[", false), Ok(()));
    assert_ne!(validator.validate_pattern("^*", false), Ok(()));
    assert_ne!(validator.validate_pattern("$*", false), Ok(()));
    assert_ne!(validator.validate_pattern("${1,2}", false), Ok(()));
    assert_ne!(validator.validate_pattern("${2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("\\2(a)(", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?:", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(:a", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[a-b--+]", false), Ok(()));
    assert_ne!(
        validator.validate_pattern("[\\u0001-\\u0000]", false),
        Ok(())
    );
    assert_ne!(validator.validate_pattern("[\\u{1}-\\u{2}]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{2}-\\u{1}]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\z-\\a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[0-9--+]", false), Ok(()));
}

#[test]
fn basic_invalid_2015_unicode() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/basic-invalid-2015-u.json
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2015);
    assert_ne!(validator.validate_pattern("(", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=foo", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?!", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?!foo", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a)*", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a)+", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a)?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){a}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){1,}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){1,2}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{a}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,2", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1", true), Ok(()));
    assert_ne!(validator.validate_pattern("(a{2,1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{}?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{a}?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,2?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(*)", true), Ok(()));
    assert_ne!(validator.validate_pattern("+", true), Ok(()));
    assert_ne!(validator.validate_pattern("?", true), Ok(()));
    assert_ne!(validator.validate_pattern(")", true), Ok(()));
    assert_ne!(validator.validate_pattern("[", true), Ok(()));
    assert_ne!(validator.validate_pattern("]", true), Ok(()));
    assert_ne!(validator.validate_pattern("{", true), Ok(()));
    assert_ne!(validator.validate_pattern("}", true), Ok(()));
    assert_ne!(validator.validate_pattern("^*", true), Ok(()));
    assert_ne!(validator.validate_pattern("$*", true), Ok(()));
    assert_ne!(validator.validate_pattern("${1,2", true), Ok(()));
    assert_ne!(validator.validate_pattern("${1,2}", true), Ok(()));
    assert_ne!(validator.validate_pattern("${2,1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\1", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\2(a)(", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a)\\1", true), Ok(()));
    assert_ne!(validator.validate_pattern("(a)\\2", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a)\\2", true), Ok(()));
    assert_ne!(
        validator.validate_pattern("(a)(a)(a)(a)(a)(a)(a)(a)(a)(a)\\11", true),
        Ok(())
    );
    assert_ne!(validator.validate_pattern("(?a", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?:", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a", true), Ok(()));
    assert_ne!(validator.validate_pattern("(:a", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\c1", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\c", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u1", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u12", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u123", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u{", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u{z", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u{20", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u{110000}", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\377", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\400", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\a", true), Ok(()));
    assert_ne!(validator.validate_pattern("[b-a]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[a-b--+]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c1]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\x]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\xz]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\x1]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u1]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u12]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u123]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{z]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{20]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{110000}]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\77]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\377]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\400]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\a]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\d-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\D-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\s-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\S-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\w-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\W-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\d]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\D]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\s]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\S]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\w]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\W]", true), Ok(()));
    assert_ne!(
        validator.validate_pattern("[\\u0001-\\u0000]", true),
        Ok(())
    );
    assert_ne!(validator.validate_pattern("[\\u{2}-\\u{1}]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{2-\\u{1}]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\a-\\z]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\z-\\a]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[0-9--+]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c-a]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c0-]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c_]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[🌸-🌷]", true), Ok(()));
    assert_ne!(
        validator.validate_pattern("[\\d][\\12-\\14]{1,}[^\\d]", true),
        Ok(())
    );
}

#[test]
fn lookbehind_assertion_invalid_2017() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/lookbehind-assertion-invalid-2017.json
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2017);
    assert_ne!(validator.validate_pattern("(?<a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)", true), Ok(()));
}

#[test]
fn lookbehind_assertion_invalid_2018() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/lookbehind-assertion-invalid-2018.json
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("(?<a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)+", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)+", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)*", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)*", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a){1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a){1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)+", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)+", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)*", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)*", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a){1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a){1}", true), Ok(()));
}

#[test]
fn named_capturing_group_invalid_2018() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/named-capturing-group-invalid-2018.json
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("(?a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("\\k", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\k<a>", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<a", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\2", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<b>", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<b>", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)(?<a>a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)(?<a>a)", true), Ok(()));
    assert_ne!(
        validator.validate_pattern("(?<a>a)(?<\\u{61}>a)", true),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("(?<a>a)(?<\\u0061>a)", true),
        Ok(())
    );
    assert_ne!(validator.validate_pattern("(?<☀>a)\\k<☀>", true), Ok(()));
    assert_ne!(
        validator.validate_pattern("(?<\\u0020>a)\\k<\\u0020>", true),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("(?<\\u0061\\u0062\\u0063>a)\\k<abd>", true),
        Ok(())
    );
    assert_ne!(validator.validate_pattern("(?<11>a)\\k<11>", true), Ok(()));
}

#[test]
fn unicode_group_names_invalid_2020() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/unicode-group-names-invalid.json
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2020);
    assert_ne!(
        validator.validate_pattern("(?<\\ud83d\\ude80>.)", false),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("(?<\\ud83d\\ude80>.)", true),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("(?<\\u{1f680}>.)", false),
        Ok(())
    );
    assert_ne!(validator.validate_pattern("(?<\\u{1f680}>.)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<🚀>.)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<🚀>.)", true), Ok(()));
}

#[test]
fn unicode_property_escape_invalid_2018() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/unicode-property-escape-invalid-2018.json
    let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("\\p", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{ASCII", true), Ok(()));
    assert_ne!(
        validator.validate_pattern("\\p{General_Category}", true),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("\\p{General_Category=}", true),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("\\p{General_Category", true),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("\\p{General_Category=", true),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("\\p{General_Category=Letter", true),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("\\p{General_Category=Hiragana}", true),
        Ok(())
    );
    assert_ne!(
        validator.validate_pattern("[\\p{Script=Hiragana}-\\p{Script=Katakana}]", true),
        Ok(())
    );
}

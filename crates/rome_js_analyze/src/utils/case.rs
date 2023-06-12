#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Case {
    /// Unknown case
    #[default]
    Unknown,
    /// camelCase
    Camel,
    // CONSTANT_CASE
    Constant,
    /// kebab-case
    Kebab,
    /// lowercase
    Lower,
    /// A, B1, C42
    NumberableCapital,
    /// PascalCase
    Pascal,
    /// snake_case
    Snake,
    /// UPPERCASE
    Upper,
}

impl Case {
    /// Returns the case of `s` or `None` if the case is unknown.
    pub(crate) fn identify(s: &str, strict: bool) -> Case {
        let mut chars = s.chars();
        let Some(first_char) = chars.next() else {
            return Case::Unknown;
        };
        if !first_char.is_alphanumeric() {
            return Case::Unknown;
        }
        let mut result = if first_char.is_uppercase() {
            Case::NumberableCapital
        } else {
            Case::Lower
        };
        let mut previous_char = first_char;
        let mut has_consecutive_uppercase = false;
        for current_char in chars {
            result = match current_char {
                '-' => match result {
                    Case::Kebab | Case::Lower => Case::Kebab,
                    _ => return Case::Unknown,
                },
                '_' => match result {
                    Case::Constant | Case::NumberableCapital | Case::Upper => Case::Constant,
                    Case::Lower | Case::Snake => Case::Snake,
                    Case::Camel | Case::Kebab | Case::Pascal | Case::Unknown => {
                        return Case::Unknown
                    }
                },
                _ if current_char.is_uppercase() => {
                    has_consecutive_uppercase |= previous_char.is_uppercase();
                    match result {
                        Case::Camel | Case::Pascal if strict && has_consecutive_uppercase => {
                            return Case::Unknown
                        }
                        Case::Camel | Case::Constant | Case::Pascal => result,
                        Case::Lower => Case::Camel,
                        Case::NumberableCapital | Case::Upper => Case::Upper,
                        Case::Kebab | Case::Snake | Case::Unknown => return Case::Unknown,
                    }
                }
                _ if current_char.is_lowercase() => match result {
                    Case::Camel | Case::Kebab | Case::Lower | Case::Snake => result,
                    Case::Pascal | Case::NumberableCapital => Case::Pascal,
                    Case::Upper if !strict || !has_consecutive_uppercase => Case::Pascal,
                    Case::Constant | Case::Upper | Case::Unknown => return Case::Unknown,
                },
                _ if current_char.is_numeric() => result, // Figures don't change the case.
                _ => return Case::Unknown,
            };
            previous_char = current_char;
        }
        result
    }

    pub(crate) const fn to_str(self) -> &'static str {
        match self {
            Case::Unknown => "unknown case",
            Case::Camel => "camelCase",
            Case::Constant => "CONSTANT_CASE",
            Case::Kebab => "kebab-case",
            Case::Lower => "lowercase",
            Case::NumberableCapital => "<Capital>[number]",
            Case::Pascal => "PascalCase",
            Case::Snake => "snake_case",
            Case::Upper => "UPPERCASE",
        }
    }

    /// Returns true if a name that respects `self` also respects `other`.
    ///
    /// For example, a name in _lowercase_ is also in _camelCase_.
    pub(crate) fn is_compatible_with(self, other: Case) -> bool {
        self == other
            || matches!(other, Case::Unknown)
            || matches!((self, other), |(
                Case::Lower,
                Case::Camel | Case::Kebab | Case::Snake,
            )| (
                Case::NumberableCapital,
                Case::Constant | Case::Pascal | Case::Upper
            ) | (
                Case::Upper,
                Case::Constant
            ))
    }

    pub(crate) fn convert(self, input: &str) -> String {
        if input.is_empty() || matches!(self, Case::Unknown) {
            return input.to_string();
        }
        let mut word_separator = matches!(self, Case::Pascal);
        let last_i = input.len() - 1;
        let mut output = String::with_capacity(input.len());
        let mut first_alphanumeric_i = 0;
        for ((i, current), next) in input
            .char_indices()
            .zip(input.chars().skip(1).map(Some).chain(Some(None)))
        {
            if (i == 0 || (i == last_i)) && (current == '_' || current == '$') {
                output.push(current);
                first_alphanumeric_i = 1;
                continue;
            }
            if !current.is_alphanumeric() {
                word_separator = true;
                continue;
            }
            if let Some(next) = next {
                if i != first_alphanumeric_i && current.is_uppercase() && next.is_lowercase() {
                    word_separator = true;
                }
            }
            if word_separator {
                match self {
                    Case::Camel
                    | Case::Lower
                    | Case::NumberableCapital
                    | Case::Pascal
                    | Case::Unknown
                    | Case::Upper => (),
                    Case::Constant | Case::Snake => {
                        output.push('_');
                    }
                    Case::Kebab => {
                        output.push('-');
                    }
                }
            }
            match self {
                Case::Camel | Case::Pascal => {
                    if word_separator {
                        output.extend(current.to_uppercase())
                    } else {
                        output.extend(current.to_lowercase())
                    }
                }
                Case::Constant | Case::Upper => output.extend(current.to_uppercase()),
                Case::NumberableCapital => {
                    if i == first_alphanumeric_i {
                        output.extend(current.to_uppercase());
                    }
                }
                Case::Kebab | Case::Snake | Case::Lower => output.extend(current.to_lowercase()),
                Case::Unknown => (),
            }
            word_separator = false;
            if let Some(next) = next {
                if current.is_lowercase() && next.is_uppercase() {
                    word_separator = true;
                }
            }
        }
        output
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Decomposed<'a> {
    pub(crate) prefix: &'a str,
    pub(crate) main: &'a str,
    pub(crate) suffix: &'a str,
}

impl<'a> Decomposed<'a> {
    pub(crate) fn from(s: &'a str) -> Self {
        let main = s.trim_start_matches(|c: char| !c.is_alphanumeric());
        let prefix = &s[..s.len() - main.len()];
        let main = main.trim_end_matches(|c: char| !c.is_alphanumeric());
        let suffix = &s[prefix.len() + main.len()..];
        Self {
            prefix,
            main,
            suffix,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_identify() {
        assert_eq!(Case::identify("strictCamelCase", true), Case::Camel,);
        assert_eq!(Case::identify("camelCASE", true), Case::Unknown);
        assert_eq!(Case::identify("strictCamelCase", false), Case::Camel,);
        assert_eq!(Case::identify("camelCASE", false), Case::Camel);

        assert_eq!(Case::identify("CONSTANT_CASE", true), Case::Constant);
        assert_eq!(Case::identify("CONSTANT_CASE", false), Case::Constant);

        assert_eq!(Case::identify("kebab-case", true), Case::Kebab);
        assert_eq!(Case::identify("kebab-case", false), Case::Kebab);

        assert_eq!(Case::identify("lowercase", true), Case::Lower);
        assert_eq!(Case::identify("lowercase", true), Case::Lower);

        assert_eq!(Case::identify("T", true), Case::NumberableCapital);
        assert_eq!(Case::identify("T", false), Case::NumberableCapital);
        assert_eq!(Case::identify("T1", true), Case::NumberableCapital);
        assert_eq!(Case::identify("T1", false), Case::NumberableCapital);

        assert_eq!(Case::identify("V8Engine", true), Case::Pascal);
        assert_eq!(Case::identify("V8Engine", false), Case::Pascal);
        assert_eq!(Case::identify("StrictPascalCase", true), Case::Pascal);
        assert_eq!(Case::identify("StrictPascalCase", false), Case::Pascal);
        assert_eq!(Case::identify("PascalCASE", true), Case::Unknown);
        assert_eq!(Case::identify("PascalCASE", false), Case::Pascal);

        assert_eq!(Case::identify("snake_case", true), Case::Snake);
        assert_eq!(Case::identify("snake_case", false), Case::Snake);

        assert_eq!(Case::identify("UPPERCASE", true), Case::Upper);
        assert_eq!(Case::identify("UPPERCASE", false), Case::Upper);

        assert_eq!(Case::identify("unknown_Case", false), Case::Unknown);
        assert_eq!(Case::identify("unknown-Case", false), Case::Unknown);
        assert_eq!(Case::identify("symbol@", false), Case::Unknown);
    }

    #[test]
    fn test_case_convert() {
        assert_eq!(Case::Camel.convert("camelCase"), "camelCase");
        assert_eq!(Case::Camel.convert("CONSTANT_CASE"), "constantCase");
        assert_eq!(Case::Camel.convert("kebab-case"), "kebabCase");
        assert_eq!(Case::Camel.convert("PascalCase"), "pascalCase");
        assert_eq!(Case::Camel.convert("snake_case"), "snakeCase");
        assert_eq!(Case::Camel.convert("Unknown_Style"), "unknownStyle");

        assert_eq!(Case::Constant.convert("camelCase"), "CAMEL_CASE");
        assert_eq!(Case::Constant.convert("CONSTANT_CASE"), "CONSTANT_CASE");
        assert_eq!(Case::Constant.convert("kebab-case"), "KEBAB_CASE");
        assert_eq!(Case::Constant.convert("PascalCase"), "PASCAL_CASE");
        assert_eq!(Case::Constant.convert("snake_case"), "SNAKE_CASE");
        assert_eq!(Case::Constant.convert("Unknown_Style"), "UNKNOWN_STYLE");

        assert_eq!(Case::Kebab.convert("camelCase"), "camel-case");
        assert_eq!(Case::Kebab.convert("CONSTANT_CASE"), "constant-case");
        assert_eq!(Case::Kebab.convert("kebab-case"), "kebab-case");
        assert_eq!(Case::Kebab.convert("PascalCase"), "pascal-case");
        assert_eq!(Case::Kebab.convert("snake_case"), "snake-case");
        assert_eq!(Case::Kebab.convert("Unknown_Style"), "unknown-style");

        assert_eq!(Case::Lower.convert("camelCase"), "camelcase");
        assert_eq!(Case::Lower.convert("CONSTANT_CASE"), "constantcase");
        assert_eq!(Case::Lower.convert("kebab-case"), "kebabcase");
        assert_eq!(Case::Lower.convert("PascalCase"), "pascalcase");
        assert_eq!(Case::Lower.convert("snake_case"), "snakecase");
        assert_eq!(Case::Lower.convert("Unknown_Style"), "unknownstyle");

        assert_eq!(Case::NumberableCapital.convert("LONG"), "L");

        assert_eq!(Case::Pascal.convert("camelCase"), "CamelCase");
        assert_eq!(Case::Pascal.convert("CONSTANT_CASE"), "ConstantCase");
        assert_eq!(Case::Pascal.convert("kebab-case"), "KebabCase");
        assert_eq!(Case::Pascal.convert("PascalCase"), "PascalCase");
        assert_eq!(Case::Pascal.convert("V8Engine"), "V8Engine");
        assert_eq!(Case::Pascal.convert("snake_case"), "SnakeCase");
        assert_eq!(Case::Pascal.convert("Unknown_Style"), "UnknownStyle");

        assert_eq!(Case::Snake.convert("camelCase"), "camel_case");
        assert_eq!(Case::Snake.convert("CONSTANT_CASE"), "constant_case");
        assert_eq!(Case::Snake.convert("kebab-case"), "kebab_case");
        assert_eq!(Case::Snake.convert("PascalCase"), "pascal_case");
        assert_eq!(Case::Snake.convert("snake_case"), "snake_case");
        assert_eq!(Case::Snake.convert("Unknown_Style"), "unknown_style");

        assert_eq!(Case::Upper.convert("camelCase"), "CAMELCASE");
        assert_eq!(Case::Upper.convert("CONSTANT_CASE"), "CONSTANTCASE");
        assert_eq!(Case::Upper.convert("kebab-case"), "KEBABCASE");
        assert_eq!(Case::Upper.convert("PascalCase"), "PASCALCASE");
        assert_eq!(Case::Upper.convert("snake_case"), "SNAKECASE");
        assert_eq!(Case::Upper.convert("Unknown_Style"), "UNKNOWNSTYLE");

        assert_eq!(Case::Unknown.convert("Unknown_Style"), "Unknown_Style");
    }

    #[test]
    fn test_decomposed_from() {
        assert_eq!(
            Decomposed::from("_"),
            Decomposed {
                prefix: "_",
                main: "",
                suffix: ""
            }
        );
        assert_eq!(
            Decomposed::from("$"),
            Decomposed {
                prefix: "$",
                main: "",
                suffix: ""
            }
        );
        assert_eq!(
            Decomposed::from("___"),
            Decomposed {
                prefix: "___",
                main: "",
                suffix: ""
            }
        );
        assert_eq!(
            Decomposed::from("$$$"),
            Decomposed {
                prefix: "$$$",
                main: "",
                suffix: ""
            }
        );

        assert_eq!(
            Decomposed::from("__CONSTANT_CASE__"),
            Decomposed {
                prefix: "__",
                main: "CONSTANT_CASE",
                suffix: "__"
            }
        );
        assert_eq!(
            Decomposed::from("CONSTANT_CASE$"),
            Decomposed {
                prefix: "",
                main: "CONSTANT_CASE",
                suffix: "$"
            }
        );
    }
}

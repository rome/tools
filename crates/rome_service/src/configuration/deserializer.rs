use crate::Configuration;
use convert_case::{Case, Casing};
use rome_console::{fmt, markup, MarkupBuf};
use rome_diagnostics::v2::{
    Advices, Diagnostic, LogCategory, MessageAndDescription, Resource, Visit,
};
use rome_rowan::{TextRange, TextSize};
use serde::de::{
    DeserializeSeed, Deserializer, EnumAccess, Error, IntoDeserializer, MapAccess, StdError,
    VariantAccess, Visitor,
};
use serde::{forward_to_deserialize_any, Deserialize};
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "flags/invalid",
    severity = Error
)]
pub struct CliDeserializeDiagnostic {
    #[location(resource)]
    path: Resource<&'static str>,
    #[location(source_code)]
    source_code: String,
    #[location(span)]
    span: Option<TextRange>,
    #[message]
    #[description]
    message: MessageAndDescription,
    #[advice]
    advice: CliDeserializeDiagnosticAdvice,
}

#[derive(Debug, Default)]
struct CliDeserializeDiagnosticAdvice {
    alternatives: Vec<MarkupBuf>,
}

impl Advices for CliDeserializeDiagnosticAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        if self.alternatives.len() > 0 {
            visitor.record_log(
                LogCategory::Info,
                &markup! {"Valid alternatives:"}.to_owned(),
            )?;

            let list: Vec<_> = self
                .alternatives
                .iter()
                .map(|s| s as &dyn fmt::Display)
                .collect();
            visitor.record_list(list.as_slice())
        } else {
            Ok(())
        }
    }
}

impl StdError for CliDeserializeDiagnostic {}

impl Display for CliDeserializeDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl serde::de::Error for CliDeserializeDiagnostic {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            path: Resource::Argv,
            span: None,
            source_code: String::new(),
            message: MessageAndDescription::from_display(msg),
            advice: CliDeserializeDiagnosticAdvice::default(),
        }
    }
}

impl CliDeserializeDiagnostic {
    pub fn new(
        source_code: impl Into<String>,
        message: impl rome_console::fmt::Display,
        range: TextRange,
    ) -> Self {
        Self {
            path: Resource::Argv,
            source_code: source_code.into(),
            message: MessageAndDescription::from_console_display(message),
            span: Some(range),
            advice: CliDeserializeDiagnosticAdvice::default(),
        }
    }

    pub fn with_alternatives<'de>(
        mut self,
        alternatives: &'de [&'de str],
        previous_field: Option<&'de str>,
    ) -> Self {
        self.advice = CliDeserializeDiagnosticAdvice {
            alternatives: alternatives
                .iter()
                .map(|s| {
                    if let Some(previous_field) = previous_field {
                        (markup! {
                            <Emphasis>{previous_field}"-"{s.to_case(Case::Kebab)}</Emphasis>
                        })
                        .to_owned()
                    } else {
                        (markup! {
                            <Emphasis>{s.to_case(Case::Kebab)}</Emphasis>
                        })
                        .to_owned()
                    }
                })
                .collect(),
        };
        self
    }
}

struct CliDeserializer<'de> {
    input: &'de str,
    source: &'de str,
    arguments: Peekable<std::slice::Iter<'de, OsString>>,
    current_fields: &'de [&'de str],
    root: bool,
    previous_field: &'de str,
}

impl<'de> CliDeserializer<'de> {
    // pub fn from_str(source: &'de str) -> Self {
    //     Self {
    //         input: source,
    //         source,
    //         arguments: vec![OsString::from(source)].iter(),
    //         current_fields: &[],
    //         root: true,
    //         previous_field: "",
    //     }
    // }

    pub fn from_args(source: &'de [OsString]) -> Self {
        Self {
            input: "",
            source: "",
            arguments: source.iter().peekable(),
            current_fields: &[],
            root: true,
            previous_field: "",
        }
    }

    fn next_argv(&mut self) {
        let argv = self.arguments.next();
        if let Some(argv) = argv {
            if let Some(argv) = argv.to_str() {
                self.source = argv;
                self.input = argv;
            }
        }
    }

    pub fn parse_argument_key(&mut self) -> Result<&'de str, CliDeserializeDiagnostic> {
        self.next_argv();
        let mut chars = self.source.chars().enumerate();
        if self.root {
            let first_dash = chars.next();
            let second_dash = chars.next();
            match (first_dash, second_dash) {
                (Some((_, first_char)), Some((second_char_index, second_char))) => {
                    if first_char == '-' && second_char == '-' {
                        let mut last_index = second_char_index + 1;
                        loop {
                            let next = chars.next();
                            if let Some((index, char)) = next {
                                last_index = index;
                                if char == '=' || char == ' ' || char == '-' {
                                    break;
                                } else {
                                    continue;
                                }
                            } else {
                                break;
                            }
                        }
                        let value = &self.source[2..last_index];
                        self.previous_field = &self.input[..self.previous_field.len() + last_index];

                        self.source = &self.source[last_index..];

                        Ok(value)
                    } else {
                        Err(Error::custom("first argument must start with --"))
                    }
                }
                _ => Err(Error::custom("end of file")),
            }
        } else if self.source.starts_with("-") {
            let mut chars = self.source.chars().enumerate();
            chars.next();
            let mut last_index = 1;
            while let Some((index, _)) = chars.next() {
                let value = &self.source[1..index];
                if self
                    .current_fields
                    .contains(&value.to_case(Case::Camel).as_ref())
                {
                    last_index = index;
                    break;
                }
            }
            let field_name = &self.source[1..last_index];
            let field_name = field_name.to_case(Case::Camel);
            let mut index = 0;
            for f in self.current_fields {
                if f == &field_name {
                    break;
                } else {
                    index += 1;
                }
            }
            let field = self.current_fields.get(index);
            if let Some(field) = field {
                self.source = &self.source[last_index..];
                self.previous_field = &self.input[..self.previous_field.len() + last_index];
                Ok(*field)
            } else {
                Err(self.error_wrong_chunk())
            }
        } else {
            Err(Error::custom("incorrect parsing"))
        }
    }

    pub fn parse_argument_value(&mut self) -> Result<Option<&'de str>, CliDeserializeDiagnostic> {
        if self.source.starts_with("=") {
            let mut chars = self.source.chars().enumerate();
            chars.next();
            let mut last_index = 1;
            loop {
                let char = chars.next();
                if let Some((index, char)) = char {
                    if char != ' ' {
                        last_index = index;
                        continue;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            let value = &self.source[1..last_index + 1];
            self.source = &self.source[last_index + 1..];
            Ok(Some(value))
        } else if self.source.starts_with("-") {
            let mut chars = self.source.chars().enumerate();
            chars.next();
            let mut last_index = 1;
            loop {
                let char = chars.next();
                if let Some((index, char)) = char {
                    if char != '=' {
                        last_index = index;
                        continue;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            let value = &self.source[1..last_index + 1];
            self.source = &self.source[last_index + 1..];
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub fn get_range_from_within_input(&self, value: &str) -> TextRange {
        let start = self.input.find(value).unwrap();
        TextRange::new(
            TextSize::from(start as u32),
            TextSize::from((start + value.len()) as u32),
        )
    }

    pub fn get_input_range(&self) -> TextRange {
        TextRange::new(TextSize::from(0), TextSize::from(self.input.len() as u32))
    }

    pub fn error_number<M: Display>(&self, value: &str, max_value: M) -> CliDeserializeDiagnostic {
        let message = format!(
            "The value of the argument exceeds the maximum allowed, {}",
            max_value
        );
        let range = self.get_range_from_within_input(value);
        CliDeserializeDiagnostic::new(self.input, message, range)
    }

    pub fn error_wrong_chunk(&self) -> CliDeserializeDiagnostic {
        let message = format!("The argument is incorrect");
        let range = self.get_input_range();
        CliDeserializeDiagnostic::new(self.input, message, range)
            .with_alternatives(self.current_fields, Some(self.previous_field))
    }

    pub fn error_not_a_value(&self) -> CliDeserializeDiagnostic {
        CliDeserializeDiagnostic::new(
            self.input,
            "This argument should have a value, but none have been found",
            self.get_input_range(),
        )
    }

    pub fn error_bool(&self, value: &str) -> CliDeserializeDiagnostic {
        CliDeserializeDiagnostic::new(
            self.input,
            "Expected a boolean, but found ",
            self.get_range_from_within_input(value),
        )
        .with_alternatives(&["true", "false"], None)
    }
}

impl<'a, 'de> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = CliDeserializeDiagnostic;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        unimplemented!()
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut CliDeserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut CliDeserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'a, 'de> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = CliDeserializeDiagnostic;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut CliDeserializer<'de> {
    type Error = CliDeserializeDiagnostic;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!("_any")
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.parse_argument_value()?.unwrap();
        if value == "true" {
            visitor.visit_bool(true)
        } else if value == "false" {
            visitor.visit_bool(false)
        } else {
            Err(self.error_bool(value))
        }
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.parse_argument_value()?.unwrap();
        visitor.visit_u8(
            value
                .parse::<u8>()
                .map_err(|_| self.error_number(value, u8::MAX))?,
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.parse_argument_value()?.unwrap();
        visitor.visit_u16(
            value
                .parse::<u16>()
                .map_err(|_| Error::custom("value exceeded value"))?,
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.parse_argument_value()?.unwrap();
        visitor.visit_u32(
            value
                .parse::<u32>()
                .map_err(|_| Error::custom("value exceeded value"))?,
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.parse_argument_value()?.unwrap();
        visitor.visit_u64(
            value
                .parse::<u64>()
                .map_err(|_| Error::custom("value exceeded value"))?,
        )
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.parse_argument_value()?.unwrap();
        visitor.visit_f32(
            value
                .parse::<f32>()
                .map_err(|_| Error::custom("value exceeded value"))?,
        )
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = self.parse_argument_value()?.unwrap();
        visitor.visit_f64(
            value
                .parse::<f64>()
                .map_err(|_| Error::custom("value exceeded value"))?,
        )
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_argument_value()?.unwrap())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // let value = self.parse_argument()?.unwrap();
        // let value = visitor.visit_string(value)?;
        // // dbg!(&value);
        // Ok(value)
        // visitor.visit_string(self.parse_str()?)
        todo!("deserialize_string")
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.source.starts_with("-") {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.current_fields = fields;
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = visitor.visit_enum(Enum::new(&mut *self))?;
        Ok(value)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

impl Configuration {
    pub fn deserialize_from_arguments<'a>(
        s: &[OsString],
    ) -> Result<Self, CliDeserializeDiagnostic> {
        let mut deserializer = CliDeserializer::from_args(s);
        let configuration = Self::deserialize(&mut deserializer)?;
        if deserializer.arguments.peek().is_none() {
            Ok(configuration)
        } else {
            Err(Error::custom("error"))
        }
    }
}

impl<'de> MapAccess<'de> for CliDeserializer<'de> {
    type Error = CliDeserializeDiagnostic;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.arguments.peek().is_none() && self.source.is_empty() {
            return Ok(None);
        }
        let key = self.parse_argument_key()?;
        self.root = false;
        // seed.deserialize(&mut *self.de).map(Some)
        seed.deserialize(key.into_deserializer()).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // let value = self.parse_argument_value()?;
        // dbg!(&value);
        // if let Some(value) = value {
        //     seed.deserialize(value.into_deserializer())
        // } else {
        seed.deserialize(&mut *self)
        // }
    }
}

struct ParsableStringDeserializer<'a>(&'a str);

impl<'de> Deserializer<'de> for ParsableStringDeserializer<'de> {
    type Error = CliDeserializeDiagnostic;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.into_deserializer().deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        map
        struct
        seq
        option
        char
        str
        string
        unit
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        identifier
        tuple
        ignored_any
        enum
        bool
        u8
        u16
        u32
        u64
        i8
        i16
        i32
        i64
        f32
        f64
    }
}

#[cfg(test)]
mod test {
    use crate::configuration::deserializer::CliDeserializeDiagnostic;
    use crate::configuration::FormatterConfiguration;
    use crate::Configuration;
    use rome_console::fmt::{Formatter, Termcolor};
    use rome_console::markup;
    use rome_diagnostics::v2::PrintDiagnostic;
    use rome_formatter::LineWidth;
    use std::ffi::OsString;
    use std::str::FromStr;

    fn print_diagnostic(err: CliDeserializeDiagnostic) {
        let mut write = rome_diagnostics::termcolor::Buffer::no_color();
        Formatter::new(&mut Termcolor(&mut write))
            .write_markup(markup! {
                {PrintDiagnostic(&err)}
            })
            .expect("failed to emit diagnostic");
        print!(
            "{}",
            std::str::from_utf8(write.as_slice()).expect("non utf8 in error buffer")
        );
    }

    #[test]
    fn incorrect_number() {
        let source = OsString::from("--formatter-indent-size=666");
        let result = Configuration::deserialize_from_arguments(&[source]);
        // dbg!(&result);
        match result {
            Ok(_) => panic!("Should error"),
            Err(err) => {
                print_diagnostic(err);
            }
        }
    }

    #[test]
    fn incorrect_nested_field() {
        let source = OsString::from("--javascript-formatter-ahahah=double");
        let result = Configuration::deserialize_from_arguments(&[source]);

        match result {
            Ok(_) => panic!("Should error"),
            Err(err) => {
                print_diagnostic(err);
            }
        }
    }

    #[test]
    fn incorrect_second_argument() {
        let source = [
            OsString::from("--javascript-formatter-quote-style=double"),
            OsString::from("--javascript-baluba"),
        ];
        let result = Configuration::deserialize_from_arguments(&source);

        match result {
            Ok(_) => panic!("Should error"),
            Err(err) => {
                print_diagnostic(err);
            }
        }
    }

    #[test]
    fn incorrect_boolean() {
        let source = [OsString::from("--formatter-enabled=double")];
        let result = Configuration::deserialize_from_arguments(&source);

        match result {
            Ok(_) => panic!("Should error"),
            Err(err) => {
                print_diagnostic(err);
            }
        }
    }

    #[test]
    fn line_with() {
        let source = [OsString::from("--formatter-line-width=120")];
        let result = Configuration::deserialize_from_arguments(&source);

        match result {
            Ok(c) => assert_eq!(
                c.formatter,
                Some(FormatterConfiguration {
                    line_width: LineWidth::from_str("120").unwrap(),
                    ..FormatterConfiguration::default()
                })
            ),
            Err(err) => {
                panic!("should not err {:?}", err)
            }
        }
    }
}

use crate::{TextRange, TextSize};
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};

impl JsonSchema for TextSize {
    fn schema_name() -> String {
        String::from("TextSize")
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        <u32>::json_schema(gen)
    }
}

impl JsonSchema for TextRange {
    fn schema_name() -> String {
        String::from("TextRange")
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        <(TextSize, TextSize)>::json_schema(gen)
    }
}

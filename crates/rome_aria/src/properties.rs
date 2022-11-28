use crate::generated::AriaPropertyTypeEnum;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use std::slice::Iter;
use std::str::FromStr;

/// A collection of ARIA properties with their metadata, necessary to perform various operations.
#[derive(Debug)]
pub struct AriaProperties(FxHashMap<&'static str, Box<dyn AriaPropertyDefinition>>);

impl Default for AriaProperties {
    fn default() -> Self {
        let list = FxHashMap::default();
        Self(list).add("aria-current", AriaCurrent)
    }
}

impl AriaProperties {
    fn add(
        mut self,
        aria_property_name: &'static str,
        definition: impl AriaPropertyDefinition + 'static,
    ) -> Self {
        self.0.insert(aria_property_name, Box::new(definition));
        self
    }

    pub fn get_property(&self, property_name: &str) -> Option<&dyn AriaPropertyDefinition> {
        self.0.get(property_name).map(|prop| prop.as_ref())
    }
}

pub trait AriaPropertyDefinition: Debug {
    /// Returns the allowed values by this property
    fn values<'a>(&self) -> Iter<'a, &str>;

    /// Returns the property type
    fn property_type(&self) -> AriaPropertyTypeEnum;

    /// It checks if a value is complaint for the current ARIA property.
    ///
    /// [Source](https://www.w3.org/TR/wai-aria-1.1/#propcharacteristic_value)
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_aria::AriaProperties;
    ///
    /// let aria_properties = AriaProperties::default();
    ///
    /// let aria_current = aria_properties.get_property("aria-current").unwrap();
    ///
    /// assert!(aria_current.contains_correct_value("true"));
    /// assert!(aria_current.contains_correct_value("false"));
    /// assert!(aria_current.contains_correct_value("step"));
    /// assert!(!aria_current.contains_correct_value("something_not_allowed"));
    /// ```
    fn contains_correct_value(&self, input_value: &str) -> bool {
        match self.property_type() {
            AriaPropertyTypeEnum::String | AriaPropertyTypeEnum::Id => {
                input_value.parse::<f32>().is_err()
            }
            AriaPropertyTypeEnum::Idlist => input_value
                .split(' ')
                .any(|piece| piece.parse::<f32>().is_err()),
            // A numerical value without a fractional component.
            AriaPropertyTypeEnum::Integer => input_value.parse::<u32>().is_ok(),
            AriaPropertyTypeEnum::Number => input_value.parse::<f32>().is_ok(),
            AriaPropertyTypeEnum::Boolean => {
                matches!(input_value, "false" | "true")
            }
            AriaPropertyTypeEnum::Token => self
                .values()
                .any(|allowed_token| *allowed_token == input_value),
            AriaPropertyTypeEnum::Tokenlist => input_value.split(' ').all(|input_token| {
                self.values()
                    .any(|allowed_token| *allowed_token == input_token)
            }),
            AriaPropertyTypeEnum::Tristate => {
                matches!(input_value, "false" | "true" | "mixed")
            }
        }
    }
}

#[derive(Debug)]
struct AriaCurrent;

impl AriaCurrent {
    const PROPERTY_TYPE: &'static str = "token";
    const VALUES: [&'static str; 7] = ["page", "step", "location", "date", "time", "true", "false"];
}

impl AriaPropertyDefinition for AriaCurrent {
    fn values<'a>(&self) -> Iter<'a, &str> {
        AriaCurrent::VALUES.iter()
    }

    fn property_type(&self) -> AriaPropertyTypeEnum {
        // SAFETY: PROPERTY_TYPE is internal and should not contain extraneous properties
        AriaPropertyTypeEnum::from_str(AriaCurrent::PROPERTY_TYPE).unwrap()
    }
}

extern crate core;

use crate::constants::{
    ARIA_ABSTRACT_ROLES, ARIA_DOCUMENT_STRUCTURE_ROLES, ARIA_PROPERTIES, ARIA_PROPERTY_TYPE,
    ARIA_WIDGET_ROLES,
};
use crate::generated::{
    AriaAbstractRolesEnum, AriaDocumentStructureRolesEnum, AriaPropertiesEnum,
    AriaPropertyTypeEnum, AriaWidgetRolesEnum,
};
use rustc_hash::FxHashMap;

#[rustfmt::skip]
mod generated;

pub mod constants;

#[derive(Debug)]
pub enum AriaRole {
    Widget(AriaWidgetRolesEnum),
    Document(AriaDocumentStructureRolesEnum),
    Abstract(AriaAbstractRolesEnum),
}

impl From<AriaRole> for &str {
    fn from(s: AriaRole) -> Self {
        match s {
            AriaRole::Widget(widget) => widget.into(),
            AriaRole::Document(document) => document.into(),
            AriaRole::Abstract(abs) => abs.into(),
        }
    }
}

impl From<&str> for AriaRole {
    fn from(s: &str) -> Self {
        if ARIA_WIDGET_ROLES.binary_search(&s).is_ok() {
            Self::Widget(AriaWidgetRolesEnum::from(s))
        } else if ARIA_ABSTRACT_ROLES.binary_search(&s).is_ok() {
            Self::Abstract(AriaAbstractRolesEnum::from(s))
        } else if ARIA_DOCUMENT_STRUCTURE_ROLES.binary_search(&s).is_ok() {
            Self::Document(AriaDocumentStructureRolesEnum::from(s))
        } else {
            unreachable!("Should not come here")
        }
    }
}

/// Table reference example: https://www.w3.org/TR/wai-aria-1.1/#checkbox
#[derive(Debug, Default)]
pub struct AriaRoleDefinition {
    properties: Vec<AriaRoleProperty>,
    super_class_role: Vec<AriaRole>,
}

#[derive(Debug)]
pub struct AriaRoleProperty {
    pub property: AriaPropertiesEnum,
    pub required: bool,
}

impl AriaRoleDefinition {
    fn with_prop(mut self, property_name: &str, required: bool) -> Self {
        self.properties.push(AriaRoleProperty {
            property: AriaPropertiesEnum::from(property_name),
            required,
        });

        self
    }

    fn with_roles(mut self, roles: &[&str]) -> Self {
        for role_name in roles {
            self.super_class_role.push(AriaRole::from(*role_name))
        }

        self
    }

    /// Given a [aria property](ARIA_PROPERTIES) as input, it checks if it's required
    /// for the current role.
    ///
    /// If the property doesn't exist for the current role, [false] is returned.
    ///
    /// ## Examples
    ///
    /// ```
    ///
    /// use rome_aria::AriaRoles;
    /// let roles = AriaRoles::default();
    ///
    /// let checkbox_role = roles.get_role("checkbox").unwrap();
    ///
    /// assert_eq!(checkbox_role.is_property_required("aria-readonly"), false);
    /// assert_eq!(checkbox_role.is_property_required("aria-checked"), true);
    ///
    /// ```
    pub fn is_property_required(&self, property_to_check: &str) -> bool {
        if is_aria_property_valid(property_to_check) {
            let property_to_check = AriaPropertiesEnum::from(property_to_check);
            for role_property in &self.properties {
                if role_property.property == property_to_check {
                    return role_property.required;
                }
            }
        }
        false
    }

    /// It returns an iterator over the properties of the current role
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_aria::AriaRoles;
    /// let roles = AriaRoles::default();
    ///
    /// let checkbox_role = roles.get_role("checkbox").unwrap();
    ///
    /// let properties = checkbox_role.properties();
    /// assert_eq!(properties.len(), 2);
    /// ```
    pub fn properties(&self) -> std::slice::Iter<AriaRoleProperty> {
        self.properties.iter()
    }
}

/// A collection of ARIA roles with their metadata, necessary to perform various operations.
#[derive(Debug)]
pub struct AriaRoles(FxHashMap<&'static str, AriaRoleDefinition>);

impl Default for AriaRoles {
    fn default() -> Self {
        Self::new()
    }
}

impl AriaRoles {
    /// It instantiate the ARIA roles
    pub fn new() -> Self {
        let mut hash_map = FxHashMap::default();
        // https://www.w3.org/TR/wai-aria-1.1/#button
        hash_map.insert(
            "button",
            AriaRoleDefinition::default()
                .with_prop("aria-expanded", false)
                .with_prop("aria-pressed", false)
                .with_roles(&["roletype", "widget", "command"]),
        );
        // https://www.w3.org/TR/wai-aria-1.1/#checkbox
        hash_map.insert(
            "checkbox",
            AriaRoleDefinition::default()
                .with_prop("aria-checked", true)
                .with_prop("aria-readonly", false)
                .with_roles(&["switch", "menuitemcheckbox", "widget"]),
        );

        Self(hash_map)
    }

    /// It returns the metadata of a role, if it exits.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_aria::AriaRoles;
    /// let roles = AriaRoles::default();
    ///
    ///
    /// let button_role = roles.get_role("button");
    /// let made_up_role = roles.get_role("made-up");
    ///
    /// assert!(button_role.is_some());
    /// assert!(made_up_role.is_none());
    /// ```
    pub fn get_role(&self, role: &str) -> Option<&AriaRoleDefinition> {
        self.0.get(role)
    }
}

/// A collection of ARIA properties with their metadata, necessary to perform various operations.
#[derive(Debug)]
pub struct AriaProperties(FxHashMap<&'static str, AriaPropertyDefinition>);

impl Default for AriaProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl AriaProperties {
    pub fn new() -> Self {
        let mut list = FxHashMap::default();

        list.insert(
            "aria-current",
            AriaPropertyDefinition::new("token")
                .with_values(&["page", "step", "location", "date", "time", "true", "false"]),
        );

        Self(list)
    }

    pub fn get_property(&self, property_name: &str) -> Option<&AriaPropertyDefinition> {
        self.0.get(property_name)
    }
}

#[derive(Debug)]
pub struct AriaPropertyDefinition {
    property_type: AriaPropertyTypeEnum,
    values: Vec<String>,
}

impl AriaPropertyDefinition {
    fn new(property_type: &str) -> Self {
        Self {
            property_type: AriaPropertyTypeEnum::from(property_type),
            values: vec![],
        }
    }

    fn with_values(mut self, values: &[&str]) -> Self {
        self.values = values.iter().map(|value| value.to_string()).collect();
        self
    }

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
    pub fn contains_correct_value(&self, input_value: &str) -> bool {
        match self.property_type {
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
                .values
                .iter()
                .any(|allowed_token| allowed_token == input_value),
            AriaPropertyTypeEnum::Tokenlist => input_value.split(' ').all(|input_token| {
                self.values
                    .iter()
                    .any(|allowed_token| allowed_token == input_token)
            }),
            AriaPropertyTypeEnum::Tristate => {
                matches!(input_value, "false" | "true" | "mixed")
            }
        }
    }
    pub fn get_allowed_values(&self) -> &Vec<String> {
        &self.values
    }
}

/// It checks if an ARIA property is valid
///
/// ## Examples
///
/// ```
/// use rome_aria::is_aria_property_valid;
///
/// assert!(!is_aria_property_valid("aria-blabla"));
/// assert!(is_aria_property_valid("aria-checked"));
/// ```
pub fn is_aria_property_valid(property: &str) -> bool {
    ARIA_PROPERTIES.binary_search(&property).is_ok()
}

/// It checks if an ARIA property type is valid
///
/// ## Examples
///
/// ```
/// use rome_aria::is_aria_property_type_valid;
///
/// assert!(is_aria_property_type_valid("string"));
/// assert!(!is_aria_property_type_valid("bogus"));
/// ```
pub fn is_aria_property_type_valid(property_type: &str) -> bool {
    ARIA_PROPERTY_TYPE.binary_search(&property_type).is_ok()
}

#[cfg(test)]
mod test {
    use crate::AriaRoles;

    #[test]
    fn property_is_required() {
        let roles = AriaRoles::default();

        let role = roles.get_role("checkbox");

        assert!(role.is_some());

        let role = role.unwrap();

        assert!(role.is_property_required("aria-checked"));
        assert!(!role.is_property_required("aria-sort"));
        assert!(!role.is_property_required("aria-bnlabla"));
    }
}

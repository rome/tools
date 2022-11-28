use crate::generated::{
    AriaAbstractRolesEnum, AriaDocumentStructureRolesEnum, AriaPropertiesEnum, AriaWidgetRolesEnum,
};
use crate::is_aria_property_valid;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use std::slice::Iter;
use std::str::FromStr;

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

impl FromStr for AriaRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AriaWidgetRolesEnum::from_str(s)
            .map(Self::Widget)
            .or_else(|_| {
                AriaAbstractRolesEnum::from_str(s)
                    .map(Self::Abstract)
                    .or_else(|_| AriaDocumentStructureRolesEnum::from_str(s).map(Self::Document))
            })
    }
}

pub trait AriaRoleDefinition: Debug {
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
    fn properties<'a>(&self) -> Iter<'a, (&str, bool)>;

    /// It returns an iterator over the possible roles of this definition
    fn roles<'a>(&self) -> Iter<'a, &str>;

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
    fn is_property_required(&self, property_to_check: &str) -> bool {
        if is_aria_property_valid(property_to_check) {
            let property_to_check = AriaPropertiesEnum::from_str(property_to_check);
            if let Ok(property_to_check) = property_to_check {
                for (property, required) in self.properties() {
                    let property = AriaPropertiesEnum::from_str(property).unwrap();
                    if property == property_to_check {
                        return *required;
                    }
                }
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct AriaRoleProperty {
    pub property: AriaPropertiesEnum,
    pub required: bool,
}

/// A collection of ARIA roles with their metadata, necessary to perform various operations.
#[derive(Debug)]
pub struct AriaRoles(FxHashMap<&'static str, Box<dyn AriaRoleDefinition>>);

impl Default for AriaRoles {
    fn default() -> Self {
        let hash_map = FxHashMap::default();
        Self(hash_map)
            // https://www.w3.org/TR/wai-aria-1.1/#button
            .add("button", ButtonRole)
            // https://www.w3.org/TR/wai-aria-1.1/#checkbox
            .add("checkbox", CheckboxRole)
    }
}

#[derive(Debug)]
struct ButtonRole;

impl ButtonRole {
    const PROPS: [(&'static str, bool); 2] = [("aria-expanded", false), ("aria-expanded", false)];
    const ROLES: [&'static str; 3] = ["roletype", "widget", "command"];
}

impl AriaRoleDefinition for ButtonRole {
    fn properties<'a>(&self) -> Iter<'a, (&str, bool)> {
        ButtonRole::PROPS.iter()
    }

    fn roles<'a>(&self) -> Iter<'a, &str> {
        ButtonRole::ROLES.iter()
    }
}

#[derive(Debug)]
struct CheckboxRole;

impl CheckboxRole {
    const PROPS: [(&'static str, bool); 2] = [("aria-checked", true), ("aria-readonly", false)];
    const ROLES: [&'static str; 3] = ["switch", "menuitemcheckbox", "widget"];
}

impl AriaRoleDefinition for CheckboxRole {
    fn properties<'a>(&self) -> Iter<'a, (&str, bool)> {
        CheckboxRole::PROPS.iter()
    }

    fn roles<'a>(&self) -> Iter<'a, &str> {
        CheckboxRole::ROLES.iter()
    }
}

impl AriaRoles {
    /// It adds a new role
    fn add(
        mut self,
        role_name: &'static str,
        definition: impl AriaRoleDefinition + 'static,
    ) -> Self {
        self.0.insert(role_name, Box::new(definition));
        self
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
    pub fn get_role(&self, role: &str) -> Option<&Box<dyn AriaRoleDefinition>> {
        self.0.get(role)
    }
}

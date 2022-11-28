use crate::constants::{ARIA_PROPERTIES, ARIA_PROPERTY_TYPE};

#[rustfmt::skip]
mod generated;

pub mod constants;
pub mod properties;
pub mod roles;

pub use properties::AriaProperties;
pub use roles::AriaRoles;

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
    use crate::roles::AriaRoles;

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

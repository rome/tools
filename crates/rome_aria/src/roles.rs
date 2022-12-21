use crate::{define_role, is_aria_property_valid};
use rome_aria_metadata::AriaPropertiesEnum;
use std::fmt::Debug;
use std::slice::Iter;
use std::str::FromStr;

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
    fn properties(&self) -> Iter<(&str, bool)>;

    /// It returns an iterator over the possible roles of this definition
    fn roles(&self) -> Iter<&str>;

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

    /// Whether the current role is interactive
    fn is_interactive(&self) -> bool {
        self.roles().any(|role| *role == "widget")
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#button
    ButtonRole {
        PROPS: [("aria-expanded", false), ("aria-expanded", false)],
        ROLES: ["roletype", "widget", "command"],
        CONCEPTS: &[("button", &[]), ("input", &[("type", "button")])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#checkbox
    CheckboxRole {
        PROPS: [("aria-checked", true), ("aria-readonly", false)],
        ROLES: ["switch", "menuitemcheckbox", "widget"],
        CONCEPTS: &[("input", &[("type", "checkbox")])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#radio
    RadioRole {
        PROPS: [("aria-checked", true), ("aria-readonly", false)],
        ROLES: ["menuitemradio", "widget"],
        CONCEPTS: &[("input", &[("type", "radio")])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#switch
    SwitchRole {
        PROPS: [("aria-checked", true)],
        ROLES: ["checkbox", "widget"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#option
    OptionRole {
        PROPS: [("aria-selected", true)],
        ROLES: ["treeitem", "widget"],
        CONCEPTS: &[("option", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#combobox
    ComboBoxRole {
        PROPS: [("aria-controls", true), ("aria-expanded", true)],
        ROLES: ["select", "widget"],
        CONCEPTS: &[("select", &[])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#heading
    HeadingRole {
        PROPS:  [("aria-level", true)],
        ROLES:  ["sectionhead"],
        CONCEPTS: &[("h1", &[]), ("h2", &[]), ("h3", &[]), ("h4", &[]), ("h5", &[]), ("h6", &[])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#spinbutton
    SpinButtonRole {
        PROPS:  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ],
        ROLES: ["composite", "input", "range", "widget"],
        CONCEPTS: &[("hr", &[])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#checkbox
    SliderRole {
        PROPS:  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ],
        ROLES: ["input", "range", "widget"],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#separator
    SeparatorRole {
        PROPS:  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ],
        ROLES: ["structure", "widget"],
        CONCEPTS: &[("hr", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#scrollbar
    ScollbarRole {
        PROPS:  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
            ("aria-orientation", true),
            ("aria-controls", true),
        ],
        ROLES: ["range", "widget"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#article
    ArticleRole {
        PROPS: [],
        ROLES: ["document"],
        CONCEPTS: &[("article", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#dialog
    DialogRole {
        PROPS: [("aria-label", false), ("aria-labelledby", false)],
        ROLES: ["window"],
        CONCEPTS: &[("dialog", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#alert
    AlertRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("alert", &[])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#alertdialog
    AlertDialogRole {
        PROPS: [],
        ROLES: ["structure"],
        CONCEPTS: &[("alert", &[])],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#application
    ApplicationRole {
        PROPS: [],
        ROLES: ["alert", "dialog"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#banner
    BannerRole {
        PROPS: [],
        ROLES: ["landmark"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#cell
    CellRole {
        PROPS: [
            ("aria-colindex", false),
            ("aria-colspan", false),
            ("aria-rowindex", false),
            ("aria-rowspan", false),
        ],
        ROLES: ["section"],
        CONCEPTS: &[("td", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#columnheader
    ColumnHeaderRole {
        PROPS: [("aria-sort", false)],
        ROLES: ["cell", "gridcell", "sectionhead"],
        CONCEPTS: &[("th", &[("scope", "col")])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#definition
    DefinitionRole {
        PROPS: [("aria-labelledby", false)],
        ROLES: ["section"],
        CONCEPTS: &[("dd", &[]), ("dfn", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#feed
    FeedRole {
        PROPS: [("aria-labelledby", false), ("aria-setsize", false)],
        ROLES: ["section"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#figure
    FigureRole {
        PROPS: [("aria-label", false), ("aria-labelledby", false)],
        ROLES: ["section"],
        CONCEPTS: &[("figure", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#form
    FormRole {
        PROPS: [("aria-label", false), ("aria-labelledby", false)],
        ROLES: ["section"],
        CONCEPTS: &[("form", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#grid
    GridRole {
        PROPS: [("aria-level", false), ("aria-multiselectable", false), ("aria-readonly", false)],
        ROLES: ["composite", "table"],
        CONCEPTS: &[("table", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#gridcell
    GridCellRole {
        PROPS: [("aria-readonly", false), ("aria-required", false), ("aria-selected", false)],
        ROLES: ["cell", "widget"],
        CONCEPTS: &[("td", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#group
    GroupRole {
        PROPS: [("aria-activedescendant", false)],
        ROLES: ["row", "select", "toolbar"],
        CONCEPTS: &[("fieldset", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#img
    ImgRole {
        PROPS: [("aria-activedescendant", false)],
        ROLES: ["section"],
        CONCEPTS: &[("img", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#link
    LinkRole {
        PROPS: [("aria-expanded", false)],
        ROLES: ["command"],
        CONCEPTS: &[("a", &[]), ("link", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#list
    ListRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("ol", &[]), ("ul", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#listbox
    ListBoxRole {
        PROPS: [],
        ROLES: ["select"],
        CONCEPTS: &[("select", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#listitem
    ListItemRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("li", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#log
    LogRole {
        PROPS: [],
        ROLES: ["section"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#main
    MainRole {
        PROPS: [],
        ROLES: ["landmark"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menubar
    MenubarRole {
        PROPS: [],
        ROLES: ["toolbar"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menu
    MenuItem {
        PROPS: [("aria-posinset", false), ("aria-setsize", false)],
        ROLES: ["command"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menuitemcheckbox
    MenuItemCheckboxRole {
        PROPS: [("aria-checked", true)],
        ROLES: ["checkbox", "menuitem"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menuitemradio
    MenuItemRadioRole {
        PROPS: [("aria-checked", true)],
        ROLES: ["radio", "menuitemcheckbox"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#navigation
    NavigationRole {
        PROPS: [],
        ROLES: ["landmark"],
        CONCEPTS: &[("nav", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#progressbar
    ProgressBarRole {
        PROPS: [("aria-valuenow", true), ("aria-valuemin", true), ("aria-valuemax", true)],
        ROLES: ["range"],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#radiogroup
    RadiogroupRole {
        PROPS: [("aria-readonly", false), ("aria-required", false)],
        ROLES: ["range"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#row
    RowRole {
        PROPS: [("aria-colindex", false), ("aria-level", false), ("aria-rowindex", false), ("aria-selected", false)],
        ROLES: ["group", "widget"],
        CONCEPTS: &[("tr", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#rowgroup
    RowGroupRole {
        PROPS: [],
        ROLES: ["structure"],
        CONCEPTS: &[("tbody", &[]), ("tfoot", &[]), ("thead", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#rowheader
    RowHeaderRole {
        PROPS: [("aria-sort", false)],
        ROLES: ["cell", "gridcell", "sectionhead"],
        CONCEPTS: &[("th", &[("scope", "row")])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#searchbox
    SearchboxRole {
        PROPS: [
            ("aria-activedescendant", false),
            ("aria-autocomplete", false),
            ("aria-multiline", false),
            ("aria-placeholder", false),
            ("aria-readonly", false),
            ("aria-required", false),
        ],
        ROLES: ["textbox"],
        CONCEPTS: &[("input", &[("type", "search")])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#tab
    TabRole {
        PROPS: [("aria-posinset", false), ("aria-selected", false), ("aria-setsize", false)],
        ROLES: ["sectionhead", "widget"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#table
    TableRole {
        PROPS: [("aria-colcount", false), ("aria-rowcount", false)],
        ROLES: ["section"],
        CONCEPTS: &[("table", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#tablelist
    TableListRole {
        PROPS: [("aria-level", false), ("aria-multiselectable", false), ("aria-orientation", false)],
        ROLES: ["composite"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#term
    TermRole {
        PROPS: [],
        ROLES: ["section"],
        CONCEPTS: &[("dt", &[])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#textbox
    TextboxRole {
        PROPS: [
            ("aria-activedescendant", false),
            ("aria-autocomplete", false),
            ("aria-multiline", false),
            ("aria-placeholder", false),
            ("aria-readonly", false),
            ("aria-required", false),
        ],
        ROLES: ["input"],
        CONCEPTS: &[("textarea", &[]), ("input", &[("type", "search")])],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#toolbar
    ToolbarRole {
        PROPS: [("aria-orientation", false)],
        ROLES: ["group"],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#tree
    TreeRole {
        PROPS: [("aria-multiselectable", false), ("aria-required", false)],
        ROLES: ["select"],
    }
}

impl<'a> AriaRoles {
    /// These are roles that will contain "concepts".
    pub(crate) const ROLE_WITH_CONCEPTS: &'a [&'a str] = &[
        "checkbox",
        "radio",
        "option",
        "combobox",
        "heading",
        "separator",
        "button",
        "article",
        "dialog",
        "alert",
        "alertdialog",
        "cell",
        "columnheader",
        "definition",
        "figure",
        "form",
        "grid",
        "gridcell",
        "group",
        "img",
        "link",
        "list",
        "listbox",
        "listitem",
        "navigation",
        "row",
        "rowgroup",
        "rowheader",
        "searchbox",
        "table",
        "term",
        "textbox",
    ];

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
    pub fn get_role(&self, role: &str) -> Option<&'static dyn AriaRoleDefinition> {
        let result = match role {
            "button" => &ButtonRole as &dyn AriaRoleDefinition,
            "checkbox" => &CheckboxRole as &dyn AriaRoleDefinition,
            "radio" => &RadioRole as &dyn AriaRoleDefinition,
            "switch" => &SwitchRole as &dyn AriaRoleDefinition,
            "option" => &OptionRole as &dyn AriaRoleDefinition,
            "combobox" => &ComboBoxRole as &dyn AriaRoleDefinition,
            "heading" => &HeadingRole as &dyn AriaRoleDefinition,
            "spinbutton" => &SpinButtonRole as &dyn AriaRoleDefinition,
            "slider" => &SliderRole as &dyn AriaRoleDefinition,
            "separator" => &SeparatorRole as &dyn AriaRoleDefinition,
            "scrollbar" => &ScollbarRole as &dyn AriaRoleDefinition,
            "article" => &ArticleRole as &dyn AriaRoleDefinition,
            "dialog" => &DialogRole as &dyn AriaRoleDefinition,
            "alert" => &AlertRole as &dyn AriaRoleDefinition,
            "alertdialog" => &AlertDialogRole as &dyn AriaRoleDefinition,
            "application" => &ApplicationRole as &dyn AriaRoleDefinition,
            "banner" => &BannerRole as &dyn AriaRoleDefinition,
            "cell" => &CellRole as &dyn AriaRoleDefinition,
            "columnheader" => &ColumnHeaderRole as &dyn AriaRoleDefinition,
            "definition" => &DefinitionRole as &dyn AriaRoleDefinition,
            "feed" => &FeedRole as &dyn AriaRoleDefinition,
            "figure" => &FigureRole as &dyn AriaRoleDefinition,
            "form" => &FormRole as &dyn AriaRoleDefinition,
            "grid" => &GridRole as &dyn AriaRoleDefinition,
            "gridcell" => &GridCellRole as &dyn AriaRoleDefinition,
            "group" => &GroupRole as &dyn AriaRoleDefinition,
            "img" => &ImgRole as &dyn AriaRoleDefinition,
            "link" => &LinkRole as &dyn AriaRoleDefinition,
            "list" => &ListRole as &dyn AriaRoleDefinition,
            "listbox" => &ListBoxRole as &dyn AriaRoleDefinition,
            "listitem" => &ListItemRole as &dyn AriaRoleDefinition,
            "log" => &LogRole as &dyn AriaRoleDefinition,
            "main" => &MainRole as &dyn AriaRoleDefinition,
            "menubar" => &MenubarRole as &dyn AriaRoleDefinition,
            "menu" => &MenuItem as &dyn AriaRoleDefinition,
            "menuitemcheckbox" => &MenuItemCheckboxRole as &dyn AriaRoleDefinition,
            "menuitemradio" => &MenuItemRadioRole as &dyn AriaRoleDefinition,
            "navigation" => &NavigationRole as &dyn AriaRoleDefinition,
            "progressbar" => &ProgressBarRole as &dyn AriaRoleDefinition,
            "radiogroup" => &RadiogroupRole as &dyn AriaRoleDefinition,
            "row" => &RowRole as &dyn AriaRoleDefinition,
            "rowgroup" => &RowGroupRole as &dyn AriaRoleDefinition,
            "rowheader" => &RowHeaderRole as &dyn AriaRoleDefinition,
            "searchbox" => &SearchboxRole as &dyn AriaRoleDefinition,
            "tab" => &TabRole as &dyn AriaRoleDefinition,
            "table" => &TableRole as &dyn AriaRoleDefinition,
            "tablelist" => &TableListRole as &dyn AriaRoleDefinition,
            "term" => &TermRole as &dyn AriaRoleDefinition,
            "textbox" => &TextboxRole as &dyn AriaRoleDefinition,
            "toolbar" => &ToolbarRole as &dyn AriaRoleDefinition,
            "tree" => &TreeRole as &dyn AriaRoleDefinition,
            _ => return None,
        };
        Some(result)
    }

    /// Given a role, it return whether this role is interactive
    pub fn is_role_interactive(&self, role: &str) -> bool {
        let role = self.get_role(role);
        if let Some(role) = role {
            role.is_interactive()
        } else {
            false
        }
    }

    /// Given the name of element, the function tell whether it's interactive
    pub fn is_not_interactive_element(&self, element_name: &str) -> bool {
        // <header> elements do not technically have semantics, unless the
        // element is a direct descendant of <body>, and this plugin cannot
        // reliably test that.
        //
        // Check: https://www.w3.org/TR/wai-aria-practices/examples/landmarks/banner.html
        if element_name == "header" {
            return false;
        }
        for element in Self::ROLE_WITH_CONCEPTS {
            let role = match *element {
                "checkbox" => &CheckboxRole as &dyn AriaRoleDefinitionWithConcepts,
                "radio" => &RadioRole as &dyn AriaRoleDefinitionWithConcepts,
                "option" => &OptionRole as &dyn AriaRoleDefinitionWithConcepts,
                "combobox" => &ComboBoxRole as &dyn AriaRoleDefinitionWithConcepts,
                "heading" => &HeadingRole as &dyn AriaRoleDefinitionWithConcepts,
                "separator" => &SeparatorRole as &dyn AriaRoleDefinitionWithConcepts,
                "button" => &ButtonRole as &dyn AriaRoleDefinitionWithConcepts,
                "article" => &ArticleRole as &dyn AriaRoleDefinitionWithConcepts,
                "dialog" => &DialogRole as &dyn AriaRoleDefinitionWithConcepts,
                "alert" => &AlertRole as &dyn AriaRoleDefinitionWithConcepts,
                "alertdialog" => &AlertDialogRole as &dyn AriaRoleDefinitionWithConcepts,
                "cell" => &CellRole as &dyn AriaRoleDefinitionWithConcepts,
                "columnheader" => &ColumnHeaderRole as &dyn AriaRoleDefinitionWithConcepts,
                "definition" => &DefinitionRole as &dyn AriaRoleDefinitionWithConcepts,
                "figure" => &FigureRole as &dyn AriaRoleDefinitionWithConcepts,
                "form" => &FormRole as &dyn AriaRoleDefinitionWithConcepts,
                "grid" => &GridRole as &dyn AriaRoleDefinitionWithConcepts,
                "gridcell" => &GridCellRole as &dyn AriaRoleDefinitionWithConcepts,
                "group" => &GroupRole as &dyn AriaRoleDefinitionWithConcepts,
                "img" => &ImgRole as &dyn AriaRoleDefinitionWithConcepts,
                "link" => &LinkRole as &dyn AriaRoleDefinitionWithConcepts,
                "list" => &ListRole as &dyn AriaRoleDefinitionWithConcepts,
                "listbox" => &ListBoxRole as &dyn AriaRoleDefinitionWithConcepts,
                "listitem" => &ListItemRole as &dyn AriaRoleDefinitionWithConcepts,
                "navigation" => &NavigationRole as &dyn AriaRoleDefinitionWithConcepts,
                "row" => &RowRole as &dyn AriaRoleDefinitionWithConcepts,
                "rowgroup" => &RowGroupRole as &dyn AriaRoleDefinitionWithConcepts,
                "rowheader" => &RowHeaderRole as &dyn AriaRoleDefinitionWithConcepts,
                "searchbox" => &SearchboxRole as &dyn AriaRoleDefinitionWithConcepts,
                "table" => &TableRole as &dyn AriaRoleDefinitionWithConcepts,
                "term" => &TermRole as &dyn AriaRoleDefinitionWithConcepts,
                "textbox" => &TextboxRole as &dyn AriaRoleDefinitionWithConcepts,
                _ => return false,
            };
            if let Some(mut concepts) = role.concepts_by_element_name(element_name) {
                if concepts.any(|(name, _)| *name == element_name) {
                    if !role.is_interactive() {
                        return true;
                    }
                }
            }
        }

        false
    }
}

type ElementsAndAttributes<'a> = Option<Iter<'a, (&'a str, &'a [(&'a str, &'a str)])>>;

pub trait AriaRoleDefinitionWithConcepts: AriaRoleDefinition {
    fn concepts_by_element_name<'a>(&self, _element_name: &str) -> ElementsAndAttributes<'a> {
        None
    }
}

/// Convenient type to retrieve metadata regarding ARIA roles
#[derive(Debug, Default)]
pub struct AriaRoles;

#[cfg(test)]
mod test {
    use crate::AriaRoles;

    #[test]
    fn should_be_interactive() {
        let aria_roles = AriaRoles {};

        assert!(!aria_roles.is_not_interactive_element("header"));
        assert!(aria_roles.is_not_interactive_element("h1"));
        assert!(aria_roles.is_not_interactive_element("h2"));
        assert!(aria_roles.is_not_interactive_element("h3"));
        assert!(aria_roles.is_not_interactive_element("h4"));
        assert!(aria_roles.is_not_interactive_element("h5"));
        assert!(aria_roles.is_not_interactive_element("h6"));
    }
}

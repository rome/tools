use crate::generated::{
    AriaAbstractRolesEnum, AriaDocumentStructureRolesEnum, AriaPropertiesEnum, AriaWidgetRolesEnum,
};
use crate::{define_role, is_aria_property_valid};
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
                for (property, required) in self.properties().as_ref() {
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

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#button
    ButtonRole {
        PROPS: [2, &[("aria-expanded", false), ("aria-expanded", false)]],
        ROLES: [3, &["roletype", "widget", "command"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#checkbox
    CheckboxRole {
        PROPS: [2, &[("aria-checked", true), ("aria-readonly", false)]],
        ROLES: [3, &["switch", "menuitemcheckbox", "widget"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#radio
    RadioRole {
        PROPS: [2, &[("aria-checked", true), ("aria-readonly", false)]],
        ROLES: [2, &["menuitemradio", "widget"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#switch
    SwitchRole {
        PROPS: [1, &[("aria-checked", true)]],
        ROLES: [2, &["checkbox", "widget"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#option
    OptionRole {
        PROPS: [1, &[("aria-selected", true)]],
        ROLES: [2, &["treeitem", "widget"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#combobox
    ComboBoxRole {
        PROPS: [2, &[("aria-controls", true), ("aria-expanded", true)]],
        ROLES: [2, &["select", "widget"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#heading
    HeadingRole {
        PROPS: [1,  &[("aria-level", true)]],
        ROLES: [1,  &["sectionhead"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#spinbutton
    SpinButtonRole {
        PROPS: [3,  &[
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ]],
        ROLES: [4, &["composite", "input", "range", "widget"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#checkbox
    SliderRole {
        PROPS: [3,  &[
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ]],
        ROLES: [3, &["input", "range", "widget"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#separator
    SeparatorRole {
        PROPS: [3,  &[
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ]],
        ROLES: [2, &["structure", "widget"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#scrollbar
    ScollbarRole {
        PROPS: [5,  &[
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
            ("aria-orientation", true),
            ("aria-controls", true),
        ]],
        ROLES: [2, &["range", "widget"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#article
    ArticleRole {
        PROPS: [0, &[]],
        ROLES: [1, &["document"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#dialog
    DialogRole {
        PROPS: [2, &[("aria-label", false), ("aria-labelledby", false)]],
        ROLES: [1, &["window"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#alert
    AlertRole {
        PROPS: [0, &[]],
        ROLES: [1, &["section"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#alertdialog
    AlertDialogRole {
        PROPS: [0, &[]],
        ROLES: [1, &["structure"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#application
    ApplicationRole {
        PROPS: [0, &[]],
        ROLES: [2, &["alert", "dialog"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#banner
    BannerRole {
        PROPS: [0, &[]],
        ROLES: [1, &["landmark"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#cell
    CellRole {
        PROPS: [4, &[
            ("aria-colindex", false),
            ("aria-colspan", false),
            ("aria-rowindex", false),
            ("aria-rowspan", false),
        ]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#columnheader
    ColumnHeaderRole {
        PROPS: [1, &[("aria-sort", false)]],
        ROLES: [3, &["cell", "gridcell", "sectionhead"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#definition
    DefinitionRole {
        PROPS: [1, &[("aria-labelledby", false)]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#feed
    FeedRole {
        PROPS: [2, &[("aria-labelledby", false), ("aria-setsize", false)]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#figure
    FigureRole {
        PROPS: [2, &[("aria-label", false), ("aria-labelledby", false)]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#form
    FormRole {
        PROPS: [2, &[("aria-label", false), ("aria-labelledby", false)]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#grid
    GridRole {
        PROPS: [3, &[("aria-level", false), ("aria-multiselectable", false), ("aria-readonly", false)]],
        ROLES: [2, &["composite", "table"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#gridcell
    GridCellRole {
        PROPS: [3, &[("aria-readonly", false), ("aria-required", false), ("aria-selected", false)]],
        ROLES: [2, &["cell", "widget"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#group
    GroupRole {
        PROPS: [1, &[("aria-activedescendant", false)]],
        ROLES: [3, &["row", "select", "toolbar"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#img
    ImgRole {
        PROPS: [1, &[("aria-activedescendant", false)]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#link
    LinkRole {
        PROPS: [1, &[("aria-expanded", false)]],
        ROLES: [1, &["command"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#list
    ListRole {
        PROPS: [0, &[]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#listbox
    ListBoxRole {
        PROPS: [0, &[]],
        ROLES: [1, &["select"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#listitem
    ListItemRole {
        PROPS: [0, &[]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#log
    LogRole {
        PROPS: [0, &[]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#main
    MainRole {
        PROPS: [0, &[]],
        ROLES: [1, &["landmark"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menubar
    MenubarRole {
        PROPS: [0, &[]],
        ROLES: [1, &["toolbar"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menu
    MenuItem {
        PROPS: [2, &[("aria-posinset", false), ("aria-setsize", false)]],
        ROLES: [1, &["command"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menuitemcheckbox
    MenuItemCheckboxRole {
        PROPS: [1, &[("aria-checked", true)]],
        ROLES: [2, &["checkbox", "menuitem"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#menuitemradio
    MenuItemRadioRole {
        PROPS: [1, &[("aria-checked", true)]],
        ROLES: [2, &["radio", "menuitemcheckbox"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#navigation
    NavigationRole {
        PROPS: [0, &[]],
        ROLES: [1, &["landmark"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#progressbar
    ProgressBarRole {
        PROPS: [3, &[("aria-valuenow", true), ("aria-valuemin", true), ("aria-valuemax", true)]],
        ROLES: [1, &["range"]],
    }
}
define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#radiogroup
    RadiogroupRole {
        PROPS: [2, &[("aria-readonly", false), ("aria-required", false)]],
        ROLES: [1, &["range"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#row
    RowRole {
        PROPS: [4, &[("aria-colindex", false), ("aria-level", false), ("aria-rowindex", false), ("aria-selected", false)]],
        ROLES: [2, &["group", "widget"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#rowgroup
    RowGroupRole {
        PROPS: [0, &[]],
        ROLES: [1, &["structure"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#rowheader
    RowHeaderRole {
        PROPS: [1, &[("aria-sort", false)]],
        ROLES: [3, &["cell", "gridcell", "sectionhead"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#searchbox
    SearchboxRole {
        PROPS: [6, &[
            ("aria-activedescendant", false),
            ("aria-autocomplete", false),
            ("aria-multiline", false),
            ("aria-placeholder", false),
            ("aria-readonly", false),
            ("aria-required", false),
        ]],
        ROLES: [1, &["textbox"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#tab
    TabRole {
        PROPS: [3, &[("aria-posinset", false), ("aria-selected", false), ("aria-setsize", false)]],
        ROLES: [2, &["sectionhead", "widget"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#table
    TableRole {
        PROPS: [2, &[("aria-colcount", false), ("aria-rowcount", false)]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#tablelist
    TableListRole {
        PROPS: [3, &[("aria-level", false), ("aria-multiselectable", false), ("aria-orientation", false)]],
        ROLES: [1, &["composite"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#term
    TermRole {
        PROPS: [0, &[]],
        ROLES: [1, &["section"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#textbox
    TextboxRole {
        PROPS: [6, &[
            ("aria-activedescendant", false),
            ("aria-autocomplete", false),
            ("aria-multiline", false),
            ("aria-placeholder", false),
            ("aria-readonly", false),
            ("aria-required", false),
        ]],
        ROLES: [1, &["input"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#toolbar
    ToolbarRole {
        PROPS: [1, &[("aria-orientation", false)]],
        ROLES: [1, &["group"]],
    }
}

define_role! {
    /// https://www.w3.org/TR/wai-aria-1.1/#tree
    TreeRole {
        PROPS: [2, &[("aria-multiselectable", false), ("aria-required", false)]],
        ROLES: [1, &["select"]],
    }
}

impl AriaRoles {
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
}

/// Convenient type to retrieve metadata regarding ARIA roles
#[derive(Debug, Default)]
pub struct AriaRoles;

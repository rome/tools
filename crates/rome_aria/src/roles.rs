use crate::generated::{
    AriaAbstractRolesEnum, AriaDocumentStructureRolesEnum, AriaPropertiesEnum, AriaWidgetRolesEnum,
};
use crate::{define_role, is_aria_property_valid};
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

define_role! {
    ButtonRole {
        PROPS: [2, [("aria-expanded", false), ("aria-expanded", false)]],
        ROLES: [3, ["roletype", "widget", "command"]],
    }
}
define_role! {
    CheckboxRole {
        PROPS: [2, [("aria-checked", true), ("aria-readonly", false)]],
        ROLES: [3, ["switch", "menuitemcheckbox", "widget"]],
    }
}
define_role! {
    RadioRole {
        PROPS: [2, [("aria-checked", true), ("aria-readonly", false)]],
        ROLES: [2,  ["menuitemradio", "widget"]],
    }
}
define_role! {
    SwitchRole {
        PROPS: [1, [("aria-checked", true)]],
        ROLES: [2,   ["checkbox", "widget"]],
    }
}

define_role! {
    OptionRole {
        PROPS: [1, [("aria-selected", true)]],
        ROLES: [2,  ["treeitem", "widget"]],
    }
}

define_role! {
    ComboBoxRole {
        PROPS: [2, [("aria-controls", true), ("aria-expanded", true)]],
        ROLES: [2,  ["select", "widget"]],
    }
}
define_role! {
    HeadingRole {
        PROPS: [1,  [("aria-level", true)]],
        ROLES: [1,  ["sectionhead"]],
    }
}
define_role! {
    SpinButtonRole {
        PROPS: [3,  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ]],
        ROLES: [4,  ["composite", "input", "range", "widget"]],
    }
}
define_role! {
    SliderRole {
        PROPS: [3,  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ]],
        ROLES: [3, ["input", "range", "widget"]],
    }
}
define_role! {
    SeparatorRole {
        PROPS: [3,  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
        ]],
        ROLES: [2, ["structure", "widget"]],
    }
}

define_role! {
    ScollbarRole {
        PROPS: [5,  [
            ("aria-valuemax", true),
            ("aria-valuemin", true),
            ("aria-valuenow", true),
            ("aria-orientation", true),
            ("aria-controls", true),
        ]],
        ROLES: [2, ["range", "widget"]],
    }
}

define_role! {
    ArticleRole {
        PROPS: [0, []],
        ROLES: [1, ["document"]],
    }
}

define_role! {
    DialogRole {
        PROPS: [2, [("aria-label", false), ("aria-labelledby", false)]],
        ROLES: [1, ["window"]],
    }
}

define_role! {
    AlertRole {
        PROPS: [0, []],
        ROLES: [1, ["section"]],
    }
}
define_role! {
    AlertDialogRole {
        PROPS: [0, []],
        ROLES: [1, ["structure"]],
    }
}
define_role! {
    ApplicationRole {
        PROPS: [0, []],
        ROLES: [2, ["alert", "dialog"]],
    }
}

define_role! {
    BannerRole {
        PROPS: [0, []],
        ROLES: [1, ["landmark"]],
    }
}

define_role! {
    CellRole {
        PROPS: [4, [
            ("aria-colindex", false),
            ("aria-colspan", false),
            ("aria-rowindex", false),
            ("aria-rowspan", false),
        ]],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    ColumnHeaderRole {
        PROPS: [1, [("aria-sort", false)]],
        ROLES: [3, ["cell", "gridcell", "sectionhead"]],
    }
}

define_role! {
    DefinitionRole {
        PROPS: [1, [("aria-labelledby", false)]],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    FeedRole {
        PROPS: [2, [("aria-labelledby", false), ("aria-setsize", false)]],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    FigureRole {
        PROPS: [2, [("aria-label", false), ("aria-labelledby", false)]],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    FormRole {
        PROPS: [2, [("aria-label", false), ("aria-labelledby", false)]],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    GridRole {
        PROPS: [3, [("aria-level", false), ("aria-multiselectable", false), ("aria-readonly", false)]],
        ROLES: [2, ["composite", "table"]],
    }
}

define_role! {
    GridCellRole {
        PROPS: [3, [("aria-readonly", false), ("aria-required", false), ("aria-selected", false)]],
        ROLES: [2, ["cell", "widget"]],
    }
}

define_role! {
    GroupRole {
        PROPS: [1, [("aria-activedescendant", false)]],
        ROLES: [3, ["row", "select", "toolbar"]],
    }
}

define_role! {
    ImgRole {
        PROPS: [1, [("aria-activedescendant", false)]],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    LinkRole {
        PROPS: [1, [("aria-expanded", false)]],
        ROLES: [1, ["command"]],
    }
}

define_role! {
    ListRole {
        PROPS: [0, []],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    ListBoxRole {
        PROPS: [0, []],
        ROLES: [1, ["select"]],
    }
}

define_role! {
    ListItemRole {
        PROPS: [0, []],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    LogRole {
        PROPS: [0, []],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    MainRole {
        PROPS: [0, []],
        ROLES: [1, ["landmark"]],
    }
}

define_role! {
    MenubarRole {
        PROPS: [0, []],
        ROLES: [1, ["toolbar"]],
    }
}

define_role! {
    MenuItem {
        PROPS: [2, [("aria-posinset", false), ("aria-setsize", false)]],
        ROLES: [1, ["command"]],
    }
}

define_role! {
    MenuItemCheckboxRole {
        PROPS: [1, [("aria-checked", true)]],
        ROLES: [2, ["checkbox", "menuitem"]],
    }
}

define_role! {
    MenuItemRadioRole {
        PROPS: [1, [("aria-checked", true)]],
        ROLES: [2, ["radio", "menuitemcheckbox"]],
    }
}

define_role! {
    NavigationRole {
        PROPS: [0, []],
        ROLES: [1, ["landmark"]],
    }
}

define_role! {
    ProgressBarRole {
        PROPS: [3, [("aria-valuenow", true), ("aria-valuemin", true), ("aria-valuemax", true)]],
        ROLES: [1, ["range"]],
    }
}
define_role! {
    RadiogroupRole {
        PROPS: [2, [("aria-readonly", false), ("aria-required", false)]],
        ROLES: [1, ["range"]],
    }
}

define_role! {
    RowRole {
        PROPS: [4, [("aria-colindex", false), ("aria-level", false), ("aria-rowindex", false), ("aria-selected", false)]],
        ROLES: [2, ["group", "widget"]],
    }
}

define_role! {
    RowGroupRole {
        PROPS: [0, []],
        ROLES: [1, ["structure"]],
    }
}

define_role! {
    RowHeaderRole {
        PROPS: [1, [("aria-sort", false)]],
        ROLES: [3, ["cell", "gridcell", "sectionhead"]],
    }
}

define_role! {
    SearchboxRole {
        PROPS: [6, [
            ("aria-activedescendant", false),
            ("aria-autocomplete", false),
            ("aria-multiline", false),
            ("aria-placeholder", false),
            ("aria-readonly", false),
            ("aria-required", false),
        ]],
        ROLES: [1, ["textbox"]],
    }
}

define_role! {
    TabRole {
        PROPS: [3, [("aria-posinset", false), ("aria-selected", false), ("aria-setsize", false)]],
        ROLES: [2, ["sectionhead", "widget"]],
    }
}

define_role! {
    TableRole {
        PROPS: [2, [("aria-colcount", false), ("aria-rowcount", false)]],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    TableListRole {
        PROPS: [3, [("aria-level", false), ("aria-multiselectable", false), ("aria-orientation", false)]],
        ROLES: [1, ["composite"]],
    }
}

define_role! {
    TermRole {
        PROPS: [0, []],
        ROLES: [1, ["section"]],
    }
}

define_role! {
    TextboxRole {
        PROPS: [6, [
            ("aria-activedescendant", false),
            ("aria-autocomplete", false),
            ("aria-multiline", false),
            ("aria-placeholder", false),
            ("aria-readonly", false),
            ("aria-required", false),
        ]],
        ROLES: [1, ["input"]],
    }
}

define_role! {
    ToolbarRole {
        PROPS: [1, [("aria-orientation", false)]],
        ROLES: [1, ["group"]],
    }
}

define_role! {
    TreeRole {
        PROPS: [2, [("aria-multiselectable", false), ("aria-required", false)]],
        ROLES: [1, ["select"]],
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
    pub fn get_role(&self, role: &str) -> Option<&dyn AriaRoleDefinition> {
        self.0.get(role).map(|value| value.as_ref())
    }
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
            // https://www.w3.org/TR/wai-aria-1.1/#radio
            .add("radio", RadioRole)
            // https://www.w3.org/TR/wai-aria-1.1/#switch
            .add("switch", SwitchRole)
            // https://www.w3.org/TR/wai-aria-1.1/#option
            .add("option", OptionRole)
            // https://www.w3.org/TR/wai-aria-1.1/#combobox
            .add("combobox", ComboBoxRole)
            // https://www.w3.org/TR/wai-aria-1.1/#heading
            .add("heading", HeadingRole)
            // https://www.w3.org/TR/wai-aria-1.1/#spinbutton
            .add("spinbutton", SpinButtonRole)
            // https://www.w3.org/TR/wai-aria-1.1/#slider
            .add("slider", SliderRole)
            // https://www.w3.org/TR/wai-aria-1.1/#separator
            .add("separator", SeparatorRole)
            // https://www.w3.org/TR/wai-aria-1.1/#scrollbar
            .add("scrollbar", ScollbarRole)
            // https://www.w3.org/TR/wai-aria-1.1/#article
            .add("article", ArticleRole)
            // https://www.w3.org/TR/wai-aria-1.1/#dialog
            .add("dialog", DialogRole)
            // https://www.w3.org/TR/wai-aria-1.1/#alert
            .add("alert", AlertRole)
            // https://www.w3.org/TR/wai-aria-1.1/#alertdialog
            .add("alertdialog", AlertDialogRole)
            // https://www.w3.org/TR/wai-aria-1.1/#application
            .add("application", ApplicationRole)
            // https://www.w3.org/TR/wai-aria-1.1/#banner
            .add("banner", BannerRole)
            // https://www.w3.org/TR/wai-aria-1.1/#cell
            .add("cell", CellRole)
            // https://www.w3.org/TR/wai-aria-1.1/#columnheader
            .add("columnheader", ColumnHeaderRole)
            // https://www.w3.org/TR/wai-aria-1.1/#definition
            .add("definition", DefinitionRole)
            // https://www.w3.org/TR/wai-aria-1.1/#feed
            .add("feed", FeedRole)
            // https://www.w3.org/TR/wai-aria-1.1/#figure
            .add("figure", FigureRole)
            // https://www.w3.org/TR/wai-aria-1.1/#form
            .add("form", FormRole)
            // https://www.w3.org/TR/wai-aria-1.1/#grid
            .add("grid", GridRole)
            // https://www.w3.org/TR/wai-aria-1.1/#gridcell
            .add("gridcell", GridCellRole)
            // https://www.w3.org/TR/wai-aria-1.1/#group
            .add("group", GroupRole)
            // https://www.w3.org/TR/wai-aria-1.1/#img
            .add("img", ImgRole)
            // https://www.w3.org/TR/wai-aria-1.1/#link
            .add("link", LinkRole)
            // https://www.w3.org/TR/wai-aria-1.1/#list
            .add("list", ListRole)
            // https://www.w3.org/TR/wai-aria-1.1/#listbox
            .add("listbox", ListBoxRole)
            // https://www.w3.org/TR/wai-aria-1.1/#listitem
            .add("listitem", ListItemRole)
            // https://www.w3.org/TR/wai-aria-1.1/#log
            .add("log", LogRole)
            // https://www.w3.org/TR/wai-aria-1.1/#main
            .add("main", MainRole)
            // https://www.w3.org/TR/wai-aria-1.1/#menubar
            .add("menubar", MenubarRole)
            // https://www.w3.org/TR/wai-aria-1.1/#menu
            .add("menu", MenuItem)
            // https://www.w3.org/TR/wai-aria-1.1/#menuitemcheckbox
            .add("menuitemcheckbox", MenuItemCheckboxRole)
            // https://www.w3.org/TR/wai-aria-1.1/#menuitemradio
            .add("menuitemradio", MenuItemRadioRole)
            // https://www.w3.org/TR/wai-aria-1.1/#navigation
            .add("navigation", NavigationRole)
            // https://www.w3.org/TR/wai-aria-1.1/#progressbar
            .add("progressbar", ProgressBarRole)
            // https://www.w3.org/TR/wai-aria-1.1/#radiogroup
            .add("radiogroup", RadiogroupRole)
            // https://www.w3.org/TR/wai-aria-1.1/#row
            .add("row", RowRole)
            // https://www.w3.org/TR/wai-aria-1.1/#rowgroup
            .add("rowgroup", RowGroupRole)
            // https://www.w3.org/TR/wai-aria-1.1/#rowheader
            .add("rowheader", RowHeaderRole)
            // https://www.w3.org/TR/wai-aria-1.1/#searchbox
            .add("searchbox", SearchboxRole)
            // https://www.w3.org/TR/wai-aria-1.1/#tab
            .add("tab", TabRole)
            // https://www.w3.org/TR/wai-aria-1.1/#table
            .add("table", TableRole)
            // https://www.w3.org/TR/wai-aria-1.1/#tablelist
            .add("tablelist", TableListRole)
            // https://www.w3.org/TR/wai-aria-1.1/#term
            .add("term", TermRole)
            // https://www.w3.org/TR/wai-aria-1.1/#textbox
            .add("textbox", TextboxRole)
            // https://www.w3.org/TR/wai-aria-1.1/#toolbar
            .add("toolbar", ToolbarRole)
            // https://www.w3.org/TR/wai-aria-1.1/#tree
            .add("tree", TreeRole)
    }
}

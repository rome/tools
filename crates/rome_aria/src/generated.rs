//! Generated file, do not edit by hand, see `xtask/codegen`
#![allow(clippy::enum_variant_names)]

use crate::constants::{
    ARIA_ABSTRACT_ROLES, ARIA_DOCUMENT_STRUCTURE_ROLES, ARIA_PROPERTIES, ARIA_PROPERTY_TYPE,
    ARIA_WIDGET_ROLES,
};
#[derive(Debug, Eq, PartialEq)]
pub enum AriaPropertiesEnum {
    AriaActivedescendant,
    AriaAtomic,
    AriaAutocomplete,
    AriaBusy,
    AriaChecked,
    AriaColcount,
    AriaColindex,
    AriaColspan,
    AriaControls,
    AriaCurrent,
    AriaDescribedby,
    AriaDetails,
    AriaDisabled,
    AriaDropeffect,
    AriaErrormessage,
    AriaExpanded,
    AriaFlowto,
    AriaGrabbed,
    AriaHaspopup,
    AriaHidden,
    AriaInvalid,
    AriaKeyshortcuts,
    AriaLabel,
    AriaLabelledby,
    AriaLevel,
    AriaLive,
    AriaModal,
    AriaMultiline,
    AriaMultiselectable,
    AriaOrientation,
    AriaOwns,
    AriaPlaceholder,
    AriaPosinset,
    AriaPressed,
    AriaReadonly,
    AriaRelevant,
    AriaRequired,
    AriaRoledescription,
    AriaRowcount,
    AriaRowindex,
    AriaRowspan,
    AriaSelected,
    AriaSetsize,
    AriaSort,
    AriaValuemax,
    AriaValuemin,
    AriaValuenow,
    AriaValuetext,
}
impl From<AriaPropertiesEnum> for &str {
    fn from(property: AriaPropertiesEnum) -> Self {
        match property {
            AriaPropertiesEnum::AriaActivedescendant => ARIA_PROPERTIES[0],
            AriaPropertiesEnum::AriaAtomic => ARIA_PROPERTIES[1],
            AriaPropertiesEnum::AriaAutocomplete => ARIA_PROPERTIES[2],
            AriaPropertiesEnum::AriaBusy => ARIA_PROPERTIES[3],
            AriaPropertiesEnum::AriaChecked => ARIA_PROPERTIES[4],
            AriaPropertiesEnum::AriaColcount => ARIA_PROPERTIES[5],
            AriaPropertiesEnum::AriaColindex => ARIA_PROPERTIES[6],
            AriaPropertiesEnum::AriaColspan => ARIA_PROPERTIES[7],
            AriaPropertiesEnum::AriaControls => ARIA_PROPERTIES[8],
            AriaPropertiesEnum::AriaCurrent => ARIA_PROPERTIES[9],
            AriaPropertiesEnum::AriaDescribedby => ARIA_PROPERTIES[10],
            AriaPropertiesEnum::AriaDetails => ARIA_PROPERTIES[11],
            AriaPropertiesEnum::AriaDisabled => ARIA_PROPERTIES[12],
            AriaPropertiesEnum::AriaDropeffect => ARIA_PROPERTIES[13],
            AriaPropertiesEnum::AriaErrormessage => ARIA_PROPERTIES[14],
            AriaPropertiesEnum::AriaExpanded => ARIA_PROPERTIES[15],
            AriaPropertiesEnum::AriaFlowto => ARIA_PROPERTIES[16],
            AriaPropertiesEnum::AriaGrabbed => ARIA_PROPERTIES[17],
            AriaPropertiesEnum::AriaHaspopup => ARIA_PROPERTIES[18],
            AriaPropertiesEnum::AriaHidden => ARIA_PROPERTIES[19],
            AriaPropertiesEnum::AriaInvalid => ARIA_PROPERTIES[20],
            AriaPropertiesEnum::AriaKeyshortcuts => ARIA_PROPERTIES[21],
            AriaPropertiesEnum::AriaLabel => ARIA_PROPERTIES[22],
            AriaPropertiesEnum::AriaLabelledby => ARIA_PROPERTIES[23],
            AriaPropertiesEnum::AriaLevel => ARIA_PROPERTIES[24],
            AriaPropertiesEnum::AriaLive => ARIA_PROPERTIES[25],
            AriaPropertiesEnum::AriaModal => ARIA_PROPERTIES[26],
            AriaPropertiesEnum::AriaMultiline => ARIA_PROPERTIES[27],
            AriaPropertiesEnum::AriaMultiselectable => ARIA_PROPERTIES[28],
            AriaPropertiesEnum::AriaOrientation => ARIA_PROPERTIES[29],
            AriaPropertiesEnum::AriaOwns => ARIA_PROPERTIES[30],
            AriaPropertiesEnum::AriaPlaceholder => ARIA_PROPERTIES[31],
            AriaPropertiesEnum::AriaPosinset => ARIA_PROPERTIES[32],
            AriaPropertiesEnum::AriaPressed => ARIA_PROPERTIES[33],
            AriaPropertiesEnum::AriaReadonly => ARIA_PROPERTIES[34],
            AriaPropertiesEnum::AriaRelevant => ARIA_PROPERTIES[35],
            AriaPropertiesEnum::AriaRequired => ARIA_PROPERTIES[36],
            AriaPropertiesEnum::AriaRoledescription => ARIA_PROPERTIES[37],
            AriaPropertiesEnum::AriaRowcount => ARIA_PROPERTIES[38],
            AriaPropertiesEnum::AriaRowindex => ARIA_PROPERTIES[39],
            AriaPropertiesEnum::AriaRowspan => ARIA_PROPERTIES[40],
            AriaPropertiesEnum::AriaSelected => ARIA_PROPERTIES[41],
            AriaPropertiesEnum::AriaSetsize => ARIA_PROPERTIES[42],
            AriaPropertiesEnum::AriaSort => ARIA_PROPERTIES[43],
            AriaPropertiesEnum::AriaValuemax => ARIA_PROPERTIES[44],
            AriaPropertiesEnum::AriaValuemin => ARIA_PROPERTIES[45],
            AriaPropertiesEnum::AriaValuenow => ARIA_PROPERTIES[46],
            AriaPropertiesEnum::AriaValuetext => ARIA_PROPERTIES[47],
        }
    }
}
impl From<&str> for AriaPropertiesEnum {
    fn from(s: &str) -> Self {
        let index = ARIA_PROPERTIES
            .binary_search(&s)
            .unwrap_or_else(|_| panic!("aria property not implemented {s:?}"));
        match index {
            0 => AriaPropertiesEnum::AriaActivedescendant,
            1 => AriaPropertiesEnum::AriaAtomic,
            2 => AriaPropertiesEnum::AriaAutocomplete,
            3 => AriaPropertiesEnum::AriaBusy,
            4 => AriaPropertiesEnum::AriaChecked,
            5 => AriaPropertiesEnum::AriaColcount,
            6 => AriaPropertiesEnum::AriaColindex,
            7 => AriaPropertiesEnum::AriaColspan,
            8 => AriaPropertiesEnum::AriaControls,
            9 => AriaPropertiesEnum::AriaCurrent,
            10 => AriaPropertiesEnum::AriaDescribedby,
            11 => AriaPropertiesEnum::AriaDetails,
            12 => AriaPropertiesEnum::AriaDisabled,
            13 => AriaPropertiesEnum::AriaDropeffect,
            14 => AriaPropertiesEnum::AriaErrormessage,
            15 => AriaPropertiesEnum::AriaExpanded,
            16 => AriaPropertiesEnum::AriaFlowto,
            17 => AriaPropertiesEnum::AriaGrabbed,
            18 => AriaPropertiesEnum::AriaHaspopup,
            19 => AriaPropertiesEnum::AriaHidden,
            20 => AriaPropertiesEnum::AriaInvalid,
            21 => AriaPropertiesEnum::AriaKeyshortcuts,
            22 => AriaPropertiesEnum::AriaLabel,
            23 => AriaPropertiesEnum::AriaLabelledby,
            24 => AriaPropertiesEnum::AriaLevel,
            25 => AriaPropertiesEnum::AriaLive,
            26 => AriaPropertiesEnum::AriaModal,
            27 => AriaPropertiesEnum::AriaMultiline,
            28 => AriaPropertiesEnum::AriaMultiselectable,
            29 => AriaPropertiesEnum::AriaOrientation,
            30 => AriaPropertiesEnum::AriaOwns,
            31 => AriaPropertiesEnum::AriaPlaceholder,
            32 => AriaPropertiesEnum::AriaPosinset,
            33 => AriaPropertiesEnum::AriaPressed,
            34 => AriaPropertiesEnum::AriaReadonly,
            35 => AriaPropertiesEnum::AriaRelevant,
            36 => AriaPropertiesEnum::AriaRequired,
            37 => AriaPropertiesEnum::AriaRoledescription,
            38 => AriaPropertiesEnum::AriaRowcount,
            39 => AriaPropertiesEnum::AriaRowindex,
            40 => AriaPropertiesEnum::AriaRowspan,
            41 => AriaPropertiesEnum::AriaSelected,
            42 => AriaPropertiesEnum::AriaSetsize,
            43 => AriaPropertiesEnum::AriaSort,
            44 => AriaPropertiesEnum::AriaValuemax,
            45 => AriaPropertiesEnum::AriaValuemin,
            46 => AriaPropertiesEnum::AriaValuenow,
            47 => AriaPropertiesEnum::AriaValuetext,
            _ => panic!("aria property not implemented"),
        }
    }
}
impl AriaPropertiesEnum {
    pub fn as_str(&self) -> &str {
        match self {
            AriaPropertiesEnum::AriaActivedescendant => ARIA_PROPERTIES[0],
            AriaPropertiesEnum::AriaAtomic => ARIA_PROPERTIES[1],
            AriaPropertiesEnum::AriaAutocomplete => ARIA_PROPERTIES[2],
            AriaPropertiesEnum::AriaBusy => ARIA_PROPERTIES[3],
            AriaPropertiesEnum::AriaChecked => ARIA_PROPERTIES[4],
            AriaPropertiesEnum::AriaColcount => ARIA_PROPERTIES[5],
            AriaPropertiesEnum::AriaColindex => ARIA_PROPERTIES[6],
            AriaPropertiesEnum::AriaColspan => ARIA_PROPERTIES[7],
            AriaPropertiesEnum::AriaControls => ARIA_PROPERTIES[8],
            AriaPropertiesEnum::AriaCurrent => ARIA_PROPERTIES[9],
            AriaPropertiesEnum::AriaDescribedby => ARIA_PROPERTIES[10],
            AriaPropertiesEnum::AriaDetails => ARIA_PROPERTIES[11],
            AriaPropertiesEnum::AriaDisabled => ARIA_PROPERTIES[12],
            AriaPropertiesEnum::AriaDropeffect => ARIA_PROPERTIES[13],
            AriaPropertiesEnum::AriaErrormessage => ARIA_PROPERTIES[14],
            AriaPropertiesEnum::AriaExpanded => ARIA_PROPERTIES[15],
            AriaPropertiesEnum::AriaFlowto => ARIA_PROPERTIES[16],
            AriaPropertiesEnum::AriaGrabbed => ARIA_PROPERTIES[17],
            AriaPropertiesEnum::AriaHaspopup => ARIA_PROPERTIES[18],
            AriaPropertiesEnum::AriaHidden => ARIA_PROPERTIES[19],
            AriaPropertiesEnum::AriaInvalid => ARIA_PROPERTIES[20],
            AriaPropertiesEnum::AriaKeyshortcuts => ARIA_PROPERTIES[21],
            AriaPropertiesEnum::AriaLabel => ARIA_PROPERTIES[22],
            AriaPropertiesEnum::AriaLabelledby => ARIA_PROPERTIES[23],
            AriaPropertiesEnum::AriaLevel => ARIA_PROPERTIES[24],
            AriaPropertiesEnum::AriaLive => ARIA_PROPERTIES[25],
            AriaPropertiesEnum::AriaModal => ARIA_PROPERTIES[26],
            AriaPropertiesEnum::AriaMultiline => ARIA_PROPERTIES[27],
            AriaPropertiesEnum::AriaMultiselectable => ARIA_PROPERTIES[28],
            AriaPropertiesEnum::AriaOrientation => ARIA_PROPERTIES[29],
            AriaPropertiesEnum::AriaOwns => ARIA_PROPERTIES[30],
            AriaPropertiesEnum::AriaPlaceholder => ARIA_PROPERTIES[31],
            AriaPropertiesEnum::AriaPosinset => ARIA_PROPERTIES[32],
            AriaPropertiesEnum::AriaPressed => ARIA_PROPERTIES[33],
            AriaPropertiesEnum::AriaReadonly => ARIA_PROPERTIES[34],
            AriaPropertiesEnum::AriaRelevant => ARIA_PROPERTIES[35],
            AriaPropertiesEnum::AriaRequired => ARIA_PROPERTIES[36],
            AriaPropertiesEnum::AriaRoledescription => ARIA_PROPERTIES[37],
            AriaPropertiesEnum::AriaRowcount => ARIA_PROPERTIES[38],
            AriaPropertiesEnum::AriaRowindex => ARIA_PROPERTIES[39],
            AriaPropertiesEnum::AriaRowspan => ARIA_PROPERTIES[40],
            AriaPropertiesEnum::AriaSelected => ARIA_PROPERTIES[41],
            AriaPropertiesEnum::AriaSetsize => ARIA_PROPERTIES[42],
            AriaPropertiesEnum::AriaSort => ARIA_PROPERTIES[43],
            AriaPropertiesEnum::AriaValuemax => ARIA_PROPERTIES[44],
            AriaPropertiesEnum::AriaValuemin => ARIA_PROPERTIES[45],
            AriaPropertiesEnum::AriaValuenow => ARIA_PROPERTIES[46],
            AriaPropertiesEnum::AriaValuetext => ARIA_PROPERTIES[47],
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum AriaPropertyTypeEnum {
    String,
    Id,
    Idlist,
    Integer,
    Number,
    Boolean,
    Token,
    Tokenlist,
    Tristate,
}
impl From<AriaPropertyTypeEnum> for &str {
    fn from(property: AriaPropertyTypeEnum) -> Self {
        match property {
            AriaPropertyTypeEnum::String => ARIA_PROPERTY_TYPE[0],
            AriaPropertyTypeEnum::Id => ARIA_PROPERTY_TYPE[1],
            AriaPropertyTypeEnum::Idlist => ARIA_PROPERTY_TYPE[2],
            AriaPropertyTypeEnum::Integer => ARIA_PROPERTY_TYPE[3],
            AriaPropertyTypeEnum::Number => ARIA_PROPERTY_TYPE[4],
            AriaPropertyTypeEnum::Boolean => ARIA_PROPERTY_TYPE[5],
            AriaPropertyTypeEnum::Token => ARIA_PROPERTY_TYPE[6],
            AriaPropertyTypeEnum::Tokenlist => ARIA_PROPERTY_TYPE[7],
            AriaPropertyTypeEnum::Tristate => ARIA_PROPERTY_TYPE[8],
        }
    }
}
impl From<&str> for AriaPropertyTypeEnum {
    fn from(s: &str) -> Self {
        let index = ARIA_PROPERTY_TYPE
            .binary_search(&s)
            .unwrap_or_else(|_| panic!("aria property not implemented {s:?}"));
        match index {
            0 => AriaPropertyTypeEnum::String,
            1 => AriaPropertyTypeEnum::Id,
            2 => AriaPropertyTypeEnum::Idlist,
            3 => AriaPropertyTypeEnum::Integer,
            4 => AriaPropertyTypeEnum::Number,
            5 => AriaPropertyTypeEnum::Boolean,
            6 => AriaPropertyTypeEnum::Token,
            7 => AriaPropertyTypeEnum::Tokenlist,
            8 => AriaPropertyTypeEnum::Tristate,
            _ => panic!("aria property not implemented"),
        }
    }
}
impl AriaPropertyTypeEnum {
    pub fn as_str(&self) -> &str {
        match self {
            AriaPropertyTypeEnum::String => ARIA_PROPERTY_TYPE[0],
            AriaPropertyTypeEnum::Id => ARIA_PROPERTY_TYPE[1],
            AriaPropertyTypeEnum::Idlist => ARIA_PROPERTY_TYPE[2],
            AriaPropertyTypeEnum::Integer => ARIA_PROPERTY_TYPE[3],
            AriaPropertyTypeEnum::Number => ARIA_PROPERTY_TYPE[4],
            AriaPropertyTypeEnum::Boolean => ARIA_PROPERTY_TYPE[5],
            AriaPropertyTypeEnum::Token => ARIA_PROPERTY_TYPE[6],
            AriaPropertyTypeEnum::Tokenlist => ARIA_PROPERTY_TYPE[7],
            AriaPropertyTypeEnum::Tristate => ARIA_PROPERTY_TYPE[8],
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum AriaWidgetRolesEnum {
    Alert,
    Alertdialog,
    Button,
    Checkbox,
    Dialog,
    Gridcell,
    Link,
    Log,
    Marquee,
    Menuitem,
    Menuitemcheckbox,
    Menuitemradio,
    Option,
    Progressbar,
    Radio,
    Scrollbar,
    Searchbox,
    Slider,
    Spinbutton,
    Status,
    Switch,
    Tab,
    Tabpanel,
    Textbox,
    Timer,
    Tooltip,
    Treeitem,
}
impl From<AriaWidgetRolesEnum> for &str {
    fn from(property: AriaWidgetRolesEnum) -> Self {
        match property {
            AriaWidgetRolesEnum::Alert => ARIA_WIDGET_ROLES[0],
            AriaWidgetRolesEnum::Alertdialog => ARIA_WIDGET_ROLES[1],
            AriaWidgetRolesEnum::Button => ARIA_WIDGET_ROLES[2],
            AriaWidgetRolesEnum::Checkbox => ARIA_WIDGET_ROLES[3],
            AriaWidgetRolesEnum::Dialog => ARIA_WIDGET_ROLES[4],
            AriaWidgetRolesEnum::Gridcell => ARIA_WIDGET_ROLES[5],
            AriaWidgetRolesEnum::Link => ARIA_WIDGET_ROLES[6],
            AriaWidgetRolesEnum::Log => ARIA_WIDGET_ROLES[7],
            AriaWidgetRolesEnum::Marquee => ARIA_WIDGET_ROLES[8],
            AriaWidgetRolesEnum::Menuitem => ARIA_WIDGET_ROLES[9],
            AriaWidgetRolesEnum::Menuitemcheckbox => ARIA_WIDGET_ROLES[10],
            AriaWidgetRolesEnum::Menuitemradio => ARIA_WIDGET_ROLES[11],
            AriaWidgetRolesEnum::Option => ARIA_WIDGET_ROLES[12],
            AriaWidgetRolesEnum::Progressbar => ARIA_WIDGET_ROLES[13],
            AriaWidgetRolesEnum::Radio => ARIA_WIDGET_ROLES[14],
            AriaWidgetRolesEnum::Scrollbar => ARIA_WIDGET_ROLES[15],
            AriaWidgetRolesEnum::Searchbox => ARIA_WIDGET_ROLES[16],
            AriaWidgetRolesEnum::Slider => ARIA_WIDGET_ROLES[17],
            AriaWidgetRolesEnum::Spinbutton => ARIA_WIDGET_ROLES[18],
            AriaWidgetRolesEnum::Status => ARIA_WIDGET_ROLES[19],
            AriaWidgetRolesEnum::Switch => ARIA_WIDGET_ROLES[20],
            AriaWidgetRolesEnum::Tab => ARIA_WIDGET_ROLES[21],
            AriaWidgetRolesEnum::Tabpanel => ARIA_WIDGET_ROLES[22],
            AriaWidgetRolesEnum::Textbox => ARIA_WIDGET_ROLES[23],
            AriaWidgetRolesEnum::Timer => ARIA_WIDGET_ROLES[24],
            AriaWidgetRolesEnum::Tooltip => ARIA_WIDGET_ROLES[25],
            AriaWidgetRolesEnum::Treeitem => ARIA_WIDGET_ROLES[26],
        }
    }
}
impl From<&str> for AriaWidgetRolesEnum {
    fn from(s: &str) -> Self {
        let index = ARIA_WIDGET_ROLES
            .binary_search(&s)
            .unwrap_or_else(|_| panic!("aria property not implemented {s:?}"));
        match index {
            0 => AriaWidgetRolesEnum::Alert,
            1 => AriaWidgetRolesEnum::Alertdialog,
            2 => AriaWidgetRolesEnum::Button,
            3 => AriaWidgetRolesEnum::Checkbox,
            4 => AriaWidgetRolesEnum::Dialog,
            5 => AriaWidgetRolesEnum::Gridcell,
            6 => AriaWidgetRolesEnum::Link,
            7 => AriaWidgetRolesEnum::Log,
            8 => AriaWidgetRolesEnum::Marquee,
            9 => AriaWidgetRolesEnum::Menuitem,
            10 => AriaWidgetRolesEnum::Menuitemcheckbox,
            11 => AriaWidgetRolesEnum::Menuitemradio,
            12 => AriaWidgetRolesEnum::Option,
            13 => AriaWidgetRolesEnum::Progressbar,
            14 => AriaWidgetRolesEnum::Radio,
            15 => AriaWidgetRolesEnum::Scrollbar,
            16 => AriaWidgetRolesEnum::Searchbox,
            17 => AriaWidgetRolesEnum::Slider,
            18 => AriaWidgetRolesEnum::Spinbutton,
            19 => AriaWidgetRolesEnum::Status,
            20 => AriaWidgetRolesEnum::Switch,
            21 => AriaWidgetRolesEnum::Tab,
            22 => AriaWidgetRolesEnum::Tabpanel,
            23 => AriaWidgetRolesEnum::Textbox,
            24 => AriaWidgetRolesEnum::Timer,
            25 => AriaWidgetRolesEnum::Tooltip,
            26 => AriaWidgetRolesEnum::Treeitem,
            _ => panic!("aria property not implemented"),
        }
    }
}
impl AriaWidgetRolesEnum {
    pub fn as_str(&self) -> &str {
        match self {
            AriaWidgetRolesEnum::Alert => ARIA_WIDGET_ROLES[0],
            AriaWidgetRolesEnum::Alertdialog => ARIA_WIDGET_ROLES[1],
            AriaWidgetRolesEnum::Button => ARIA_WIDGET_ROLES[2],
            AriaWidgetRolesEnum::Checkbox => ARIA_WIDGET_ROLES[3],
            AriaWidgetRolesEnum::Dialog => ARIA_WIDGET_ROLES[4],
            AriaWidgetRolesEnum::Gridcell => ARIA_WIDGET_ROLES[5],
            AriaWidgetRolesEnum::Link => ARIA_WIDGET_ROLES[6],
            AriaWidgetRolesEnum::Log => ARIA_WIDGET_ROLES[7],
            AriaWidgetRolesEnum::Marquee => ARIA_WIDGET_ROLES[8],
            AriaWidgetRolesEnum::Menuitem => ARIA_WIDGET_ROLES[9],
            AriaWidgetRolesEnum::Menuitemcheckbox => ARIA_WIDGET_ROLES[10],
            AriaWidgetRolesEnum::Menuitemradio => ARIA_WIDGET_ROLES[11],
            AriaWidgetRolesEnum::Option => ARIA_WIDGET_ROLES[12],
            AriaWidgetRolesEnum::Progressbar => ARIA_WIDGET_ROLES[13],
            AriaWidgetRolesEnum::Radio => ARIA_WIDGET_ROLES[14],
            AriaWidgetRolesEnum::Scrollbar => ARIA_WIDGET_ROLES[15],
            AriaWidgetRolesEnum::Searchbox => ARIA_WIDGET_ROLES[16],
            AriaWidgetRolesEnum::Slider => ARIA_WIDGET_ROLES[17],
            AriaWidgetRolesEnum::Spinbutton => ARIA_WIDGET_ROLES[18],
            AriaWidgetRolesEnum::Status => ARIA_WIDGET_ROLES[19],
            AriaWidgetRolesEnum::Switch => ARIA_WIDGET_ROLES[20],
            AriaWidgetRolesEnum::Tab => ARIA_WIDGET_ROLES[21],
            AriaWidgetRolesEnum::Tabpanel => ARIA_WIDGET_ROLES[22],
            AriaWidgetRolesEnum::Textbox => ARIA_WIDGET_ROLES[23],
            AriaWidgetRolesEnum::Timer => ARIA_WIDGET_ROLES[24],
            AriaWidgetRolesEnum::Tooltip => ARIA_WIDGET_ROLES[25],
            AriaWidgetRolesEnum::Treeitem => ARIA_WIDGET_ROLES[26],
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum AriaAbstractRolesEnum {
    Command,
    Composite,
    Input,
    Landmark,
    Range,
    Roletype,
    Section,
    Sectionhead,
    Select,
    Structure,
    Widget,
    Window,
}
impl From<AriaAbstractRolesEnum> for &str {
    fn from(property: AriaAbstractRolesEnum) -> Self {
        match property {
            AriaAbstractRolesEnum::Command => ARIA_ABSTRACT_ROLES[0],
            AriaAbstractRolesEnum::Composite => ARIA_ABSTRACT_ROLES[1],
            AriaAbstractRolesEnum::Input => ARIA_ABSTRACT_ROLES[2],
            AriaAbstractRolesEnum::Landmark => ARIA_ABSTRACT_ROLES[3],
            AriaAbstractRolesEnum::Range => ARIA_ABSTRACT_ROLES[4],
            AriaAbstractRolesEnum::Roletype => ARIA_ABSTRACT_ROLES[5],
            AriaAbstractRolesEnum::Section => ARIA_ABSTRACT_ROLES[6],
            AriaAbstractRolesEnum::Sectionhead => ARIA_ABSTRACT_ROLES[7],
            AriaAbstractRolesEnum::Select => ARIA_ABSTRACT_ROLES[8],
            AriaAbstractRolesEnum::Structure => ARIA_ABSTRACT_ROLES[9],
            AriaAbstractRolesEnum::Widget => ARIA_ABSTRACT_ROLES[10],
            AriaAbstractRolesEnum::Window => ARIA_ABSTRACT_ROLES[11],
        }
    }
}
impl From<&str> for AriaAbstractRolesEnum {
    fn from(s: &str) -> Self {
        let index = ARIA_ABSTRACT_ROLES
            .binary_search(&s)
            .unwrap_or_else(|_| panic!("aria property not implemented {s:?}"));
        match index {
            0 => AriaAbstractRolesEnum::Command,
            1 => AriaAbstractRolesEnum::Composite,
            2 => AriaAbstractRolesEnum::Input,
            3 => AriaAbstractRolesEnum::Landmark,
            4 => AriaAbstractRolesEnum::Range,
            5 => AriaAbstractRolesEnum::Roletype,
            6 => AriaAbstractRolesEnum::Section,
            7 => AriaAbstractRolesEnum::Sectionhead,
            8 => AriaAbstractRolesEnum::Select,
            9 => AriaAbstractRolesEnum::Structure,
            10 => AriaAbstractRolesEnum::Widget,
            11 => AriaAbstractRolesEnum::Window,
            _ => panic!("aria property not implemented"),
        }
    }
}
impl AriaAbstractRolesEnum {
    pub fn as_str(&self) -> &str {
        match self {
            AriaAbstractRolesEnum::Command => ARIA_ABSTRACT_ROLES[0],
            AriaAbstractRolesEnum::Composite => ARIA_ABSTRACT_ROLES[1],
            AriaAbstractRolesEnum::Input => ARIA_ABSTRACT_ROLES[2],
            AriaAbstractRolesEnum::Landmark => ARIA_ABSTRACT_ROLES[3],
            AriaAbstractRolesEnum::Range => ARIA_ABSTRACT_ROLES[4],
            AriaAbstractRolesEnum::Roletype => ARIA_ABSTRACT_ROLES[5],
            AriaAbstractRolesEnum::Section => ARIA_ABSTRACT_ROLES[6],
            AriaAbstractRolesEnum::Sectionhead => ARIA_ABSTRACT_ROLES[7],
            AriaAbstractRolesEnum::Select => ARIA_ABSTRACT_ROLES[8],
            AriaAbstractRolesEnum::Structure => ARIA_ABSTRACT_ROLES[9],
            AriaAbstractRolesEnum::Widget => ARIA_ABSTRACT_ROLES[10],
            AriaAbstractRolesEnum::Window => ARIA_ABSTRACT_ROLES[11],
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum AriaDocumentStructureRolesEnum {
    Article,
    Cell,
    Columnheader,
    Definition,
    Directory,
    Document,
    Feed,
    Figure,
    Group,
    Heading,
    Img,
    List,
    Listitem,
    Math,
    None,
    Note,
    Presentation,
    Region,
    Row,
    Rowgroup,
    Rowheader,
    Separator,
    Table,
    Term,
    Toolbar,
}
impl From<AriaDocumentStructureRolesEnum> for &str {
    fn from(property: AriaDocumentStructureRolesEnum) -> Self {
        match property {
            AriaDocumentStructureRolesEnum::Article => ARIA_DOCUMENT_STRUCTURE_ROLES[0],
            AriaDocumentStructureRolesEnum::Cell => ARIA_DOCUMENT_STRUCTURE_ROLES[1],
            AriaDocumentStructureRolesEnum::Columnheader => ARIA_DOCUMENT_STRUCTURE_ROLES[2],
            AriaDocumentStructureRolesEnum::Definition => ARIA_DOCUMENT_STRUCTURE_ROLES[3],
            AriaDocumentStructureRolesEnum::Directory => ARIA_DOCUMENT_STRUCTURE_ROLES[4],
            AriaDocumentStructureRolesEnum::Document => ARIA_DOCUMENT_STRUCTURE_ROLES[5],
            AriaDocumentStructureRolesEnum::Feed => ARIA_DOCUMENT_STRUCTURE_ROLES[6],
            AriaDocumentStructureRolesEnum::Figure => ARIA_DOCUMENT_STRUCTURE_ROLES[7],
            AriaDocumentStructureRolesEnum::Group => ARIA_DOCUMENT_STRUCTURE_ROLES[8],
            AriaDocumentStructureRolesEnum::Heading => ARIA_DOCUMENT_STRUCTURE_ROLES[9],
            AriaDocumentStructureRolesEnum::Img => ARIA_DOCUMENT_STRUCTURE_ROLES[10],
            AriaDocumentStructureRolesEnum::List => ARIA_DOCUMENT_STRUCTURE_ROLES[11],
            AriaDocumentStructureRolesEnum::Listitem => ARIA_DOCUMENT_STRUCTURE_ROLES[12],
            AriaDocumentStructureRolesEnum::Math => ARIA_DOCUMENT_STRUCTURE_ROLES[13],
            AriaDocumentStructureRolesEnum::None => ARIA_DOCUMENT_STRUCTURE_ROLES[14],
            AriaDocumentStructureRolesEnum::Note => ARIA_DOCUMENT_STRUCTURE_ROLES[15],
            AriaDocumentStructureRolesEnum::Presentation => ARIA_DOCUMENT_STRUCTURE_ROLES[16],
            AriaDocumentStructureRolesEnum::Region => ARIA_DOCUMENT_STRUCTURE_ROLES[17],
            AriaDocumentStructureRolesEnum::Row => ARIA_DOCUMENT_STRUCTURE_ROLES[18],
            AriaDocumentStructureRolesEnum::Rowgroup => ARIA_DOCUMENT_STRUCTURE_ROLES[19],
            AriaDocumentStructureRolesEnum::Rowheader => ARIA_DOCUMENT_STRUCTURE_ROLES[20],
            AriaDocumentStructureRolesEnum::Separator => ARIA_DOCUMENT_STRUCTURE_ROLES[21],
            AriaDocumentStructureRolesEnum::Table => ARIA_DOCUMENT_STRUCTURE_ROLES[22],
            AriaDocumentStructureRolesEnum::Term => ARIA_DOCUMENT_STRUCTURE_ROLES[23],
            AriaDocumentStructureRolesEnum::Toolbar => ARIA_DOCUMENT_STRUCTURE_ROLES[24],
        }
    }
}
impl From<&str> for AriaDocumentStructureRolesEnum {
    fn from(s: &str) -> Self {
        let index = ARIA_DOCUMENT_STRUCTURE_ROLES
            .binary_search(&s)
            .unwrap_or_else(|_| panic!("aria property not implemented {s:?}"));
        match index {
            0 => AriaDocumentStructureRolesEnum::Article,
            1 => AriaDocumentStructureRolesEnum::Cell,
            2 => AriaDocumentStructureRolesEnum::Columnheader,
            3 => AriaDocumentStructureRolesEnum::Definition,
            4 => AriaDocumentStructureRolesEnum::Directory,
            5 => AriaDocumentStructureRolesEnum::Document,
            6 => AriaDocumentStructureRolesEnum::Feed,
            7 => AriaDocumentStructureRolesEnum::Figure,
            8 => AriaDocumentStructureRolesEnum::Group,
            9 => AriaDocumentStructureRolesEnum::Heading,
            10 => AriaDocumentStructureRolesEnum::Img,
            11 => AriaDocumentStructureRolesEnum::List,
            12 => AriaDocumentStructureRolesEnum::Listitem,
            13 => AriaDocumentStructureRolesEnum::Math,
            14 => AriaDocumentStructureRolesEnum::None,
            15 => AriaDocumentStructureRolesEnum::Note,
            16 => AriaDocumentStructureRolesEnum::Presentation,
            17 => AriaDocumentStructureRolesEnum::Region,
            18 => AriaDocumentStructureRolesEnum::Row,
            19 => AriaDocumentStructureRolesEnum::Rowgroup,
            20 => AriaDocumentStructureRolesEnum::Rowheader,
            21 => AriaDocumentStructureRolesEnum::Separator,
            22 => AriaDocumentStructureRolesEnum::Table,
            23 => AriaDocumentStructureRolesEnum::Term,
            24 => AriaDocumentStructureRolesEnum::Toolbar,
            _ => panic!("aria property not implemented"),
        }
    }
}
impl AriaDocumentStructureRolesEnum {
    pub fn as_str(&self) -> &str {
        match self {
            AriaDocumentStructureRolesEnum::Article => ARIA_DOCUMENT_STRUCTURE_ROLES[0],
            AriaDocumentStructureRolesEnum::Cell => ARIA_DOCUMENT_STRUCTURE_ROLES[1],
            AriaDocumentStructureRolesEnum::Columnheader => ARIA_DOCUMENT_STRUCTURE_ROLES[2],
            AriaDocumentStructureRolesEnum::Definition => ARIA_DOCUMENT_STRUCTURE_ROLES[3],
            AriaDocumentStructureRolesEnum::Directory => ARIA_DOCUMENT_STRUCTURE_ROLES[4],
            AriaDocumentStructureRolesEnum::Document => ARIA_DOCUMENT_STRUCTURE_ROLES[5],
            AriaDocumentStructureRolesEnum::Feed => ARIA_DOCUMENT_STRUCTURE_ROLES[6],
            AriaDocumentStructureRolesEnum::Figure => ARIA_DOCUMENT_STRUCTURE_ROLES[7],
            AriaDocumentStructureRolesEnum::Group => ARIA_DOCUMENT_STRUCTURE_ROLES[8],
            AriaDocumentStructureRolesEnum::Heading => ARIA_DOCUMENT_STRUCTURE_ROLES[9],
            AriaDocumentStructureRolesEnum::Img => ARIA_DOCUMENT_STRUCTURE_ROLES[10],
            AriaDocumentStructureRolesEnum::List => ARIA_DOCUMENT_STRUCTURE_ROLES[11],
            AriaDocumentStructureRolesEnum::Listitem => ARIA_DOCUMENT_STRUCTURE_ROLES[12],
            AriaDocumentStructureRolesEnum::Math => ARIA_DOCUMENT_STRUCTURE_ROLES[13],
            AriaDocumentStructureRolesEnum::None => ARIA_DOCUMENT_STRUCTURE_ROLES[14],
            AriaDocumentStructureRolesEnum::Note => ARIA_DOCUMENT_STRUCTURE_ROLES[15],
            AriaDocumentStructureRolesEnum::Presentation => ARIA_DOCUMENT_STRUCTURE_ROLES[16],
            AriaDocumentStructureRolesEnum::Region => ARIA_DOCUMENT_STRUCTURE_ROLES[17],
            AriaDocumentStructureRolesEnum::Row => ARIA_DOCUMENT_STRUCTURE_ROLES[18],
            AriaDocumentStructureRolesEnum::Rowgroup => ARIA_DOCUMENT_STRUCTURE_ROLES[19],
            AriaDocumentStructureRolesEnum::Rowheader => ARIA_DOCUMENT_STRUCTURE_ROLES[20],
            AriaDocumentStructureRolesEnum::Separator => ARIA_DOCUMENT_STRUCTURE_ROLES[21],
            AriaDocumentStructureRolesEnum::Table => ARIA_DOCUMENT_STRUCTURE_ROLES[22],
            AriaDocumentStructureRolesEnum::Term => ARIA_DOCUMENT_STRUCTURE_ROLES[23],
            AriaDocumentStructureRolesEnum::Toolbar => ARIA_DOCUMENT_STRUCTURE_ROLES[24],
        }
    }
}

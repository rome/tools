export type ARIAPropertyDefinition = {
	type: ARIAPropertyType;
	values?: Array<string | boolean>;
	allowUndefined?: boolean;
};

export type MapOfARIAPropertyDefinitions = Map<
	ARIAProperty,
	ARIAPropertyDefinition
>;

export type ARIAAbstractRole =
	| "command"
	| "composite"
	| "input"
	| "landmark"
	| "range"
	| "roletype"
	| "section"
	| "sectionhead"
	| "select"
	| "structure"
	| "widget"
	| "window";

export type ARIAWidgetRole =
	| "alert"
	| "alertdialog"
	| "button"
	| "checkbox"
	| "dialog"
	| "gridcell"
	| "link"
	| "log"
	| "marquee"
	| "menuitem"
	| "menuitemcheckbox"
	| "menuitemradio"
	| "option"
	| "progressbar"
	| "radio"
	| "scrollbar"
	| "searchbox"
	| "slider"
	| "spinbutton"
	| "status"
	| "switch"
	| "tab"
	| "tabpanel"
	| "textbox"
	| "timer"
	| "tooltip"
	| "treeitem";

export type ARIADocumentStructureRole =
	| "article"
	| "cell"
	| "columnheader"
	| "definition"
	| "directory"
	| "document"
	| "feed"
	| "figure"
	| "group"
	| "heading"
	| "img"
	| "list"
	| "listitem"
	| "math"
	| "none"
	| "note"
	| "presentation"
	| "region"
	| "row"
	| "rowgroup"
	| "rowheader"
	| "separator"
	| "table"
	| "term"
	| "toolbar";

export type ARIAProperty =
	| "aria-activedescendant"
	| "aria-atomic"
	| "aria-autocomplete"
	| "aria-colcount"
	| "aria-colindex"
	| "aria-colspan"
	| "aria-controls"
	| "aria-current"
	| "aria-describedby"
	| "aria-details"
	| "aria-dropeffect"
	| "aria-errormessage"
	| "aria-flowto"
	| "aria-haspopup"
	| "aria-keyshortcuts"
	| "aria-label"
	| "aria-labelledby"
	| "aria-level"
	| "aria-live"
	| "aria-modal"
	| "aria-multiline"
	| "aria-multiselectable"
	| "aria-orientation"
	| "aria-owns"
	| "aria-placeholder"
	| "aria-posinset"
	| "aria-readonly"
	| "aria-relevant"
	| "aria-required"
	| "aria-roledescription"
	| "aria-rowcount"
	| "aria-rowindex"
	| "aria-rowspan"
	| "aria-setsize"
	| "aria-sort"
	| "aria-valuemax"
	| "aria-valuemin"
	| "aria-valuenow"
	| "aria-valuetext"
	| ARIAState;

export type ARIAState =
	| "aria-busy"
	| "aria-checked"
	| "aria-disabled"
	| "aria-expanded"
	| "aria-grabbed"
	| "aria-hidden"
	| "aria-invalid"
	| "aria-pressed"
	| "aria-selected";

export type ARIAPropertyType =
	| "string"
	| "id"
	| "idlist"
	| "integer"
	| "number"
	| "boolean"
	| "token"
	| "tokenlist"
	| "tristate";

type ARIAConceptAttribute = {
	name: string;
	value: string;
};

export type ARIAConcept = {
	// name of the concept. Is should be the tag name: tr, button, input, etc.
	name: string;
	// sometimes W3C specifies some attributes in relation of the role
	// For example, the role="checkbox" is like <input type="checkbox" />
	// attributes is needed to capture the _type="checkbox"_ part
	attributes?: Array<ARIAConceptAttribute>;
};

export type ARIABaseConcept = {
	module: "HTML" | "XForms";
	concept: ARIAConcept;
};

export type ARIARole =
	| ARIAAbstractRole
	| ARIAWidgetRole
	| ARIADocumentStructureRole;

/**
 * Table reference example: https://www.w3.org/TR/wai-aria-1.1/#checkbox
 */
export type ARIARoleDefinition = {
	props: Array<ARIAProperty>;
	requiredProps: Array<ARIAProperty>;
	superClassRole: Array<ARIARole>;
	/**
     * Having a concept means that a role can be directly mapped to a HTML element
     * For example:
     * - role row => <tr />
     * - role checkbox => <input type="checkbox" />
     * - role button => <button />
     */
	baseConcepts?: Array<ARIABaseConcept>;
};

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

export type ARIAPropertyDefinition = {
	type: ARIAPropertyType;
	values?: Array<string | boolean>;
	allowUndefined?: boolean;
};

export const ariaPropsMap: MapOfARIAPropertyDefinitions = new Map([
	[
		"aria-activedescendant",
		{
			type: "id",
		},
	],
	[
		"aria-atomic",
		{
			type: "boolean",
		},
	],
	[
		"aria-autocomplete",
		{
			type: "token",
			values: ["inline", "list", "both", "none"],
		},
	],
	[
		"aria-busy",
		{
			type: "boolean",
		},
	],
	[
		"aria-checked",
		{
			type: "tristate",
		},
	],
	[
		"aria-colcount",
		{
			type: "integer",
		},
	],
	[
		"aria-colindex",
		{
			type: "integer",
		},
	],
	[
		"aria-colspan",
		{
			type: "integer",
		},
	],
	[
		"aria-controls",
		{
			type: "idlist",
		},
	],
	[
		"aria-current",
		{
			type: "token",
			values: ["page", "step", "location", "date", "time", true, false],
		},
	],
	[
		"aria-describedby",
		{
			type: "idlist",
		},
	],
	[
		"aria-details",
		{
			type: "id",
		},
	],
	[
		"aria-disabled",
		{
			type: "boolean",
		},
	],
	[
		"aria-dropeffect",
		{
			type: "tokenlist",
			values: ["copy", "execute", "link", "move", "none", "popup"],
		},
	],
	[
		"aria-errormessage",
		{
			type: "id",
		},
	],
	[
		"aria-expanded",
		{
			type: "boolean",
			allowUndefined: true,
		},
	],
	[
		"aria-flowto",
		{
			type: "idlist",
		},
	],
	[
		"aria-grabbed",
		{
			type: "boolean",
			allowUndefined: true,
		},
	],
	[
		"aria-haspopup",
		{
			type: "token",
			values: [false, true, "menu", "listbox", "tree", "grid", "dialog"],
		},
	],
	[
		"aria-hidden",
		{
			type: "boolean",
			allowUndefined: true,
		},
	],
	[
		"aria-invalid",
		{
			type: "token",
			values: ["grammar", false, "spelling", true],
		},
	],
	[
		"aria-keyshortcuts",
		{
			type: "string",
		},
	],
	[
		"aria-label",
		{
			type: "string",
		},
	],
	[
		"aria-labelledby",
		{
			type: "idlist",
		},
	],
	[
		"aria-level",
		{
			type: "integer",
		},
	],
	[
		"aria-live",
		{
			type: "token",
			values: ["assertive", "off", "polite"],
		},
	],
	[
		"aria-modal",
		{
			type: "boolean",
		},
	],
	[
		"aria-multiline",
		{
			type: "boolean",
		},
	],
	[
		"aria-multiselectable",
		{
			type: "boolean",
		},
	],
	[
		"aria-orientation",
		{
			type: "token",
			values: ["vertical", "undefined", "horizontal"],
		},
	],
	[
		"aria-owns",
		{
			type: "idlist",
		},
	],
	[
		"aria-placeholder",
		{
			type: "string",
		},
	],
	[
		"aria-posinset",
		{
			type: "integer",
		},
	],
	[
		"aria-pressed",
		{
			type: "tristate",
		},
	],
	[
		"aria-readonly",
		{
			type: "boolean",
		},
	],
	[
		"aria-relevant",
		{
			type: "tokenlist",
			values: ["additions", "all", "removals", "text"],
		},
	],
	[
		"aria-required",
		{
			type: "boolean",
		},
	],
	[
		"aria-roledescription",
		{
			type: "string",
		},
	],
	[
		"aria-rowcount",
		{
			type: "integer",
		},
	],
	[
		"aria-rowindex",
		{
			type: "integer",
		},
	],
	[
		"aria-rowspan",
		{
			type: "integer",
		},
	],
	[
		"aria-selected",
		{
			type: "boolean",
			allowUndefined: true,
		},
	],
	[
		"aria-setsize",
		{
			type: "integer",
		},
	],
	[
		"aria-sort",
		{
			type: "token",
			values: ["ascending", "descending", "none", "other"],
		},
	],
	[
		"aria-valuemax",
		{
			type: "number",
		},
	],
	[
		"aria-valuemin",
		{
			type: "number",
		},
	],
	[
		"aria-valuenow",
		{
			type: "number",
		},
	],
	[
		"aria-valuetext",
		{
			type: "string",
		},
	],
]);

type ARIAConceptAttribute = {
	name: string;
	value: string;
};

type ARIAConcept = {
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

export type MapOfAriaRoles = Map<string, ARIARoleDefinition>;
export type MapOfElementsToConcepts = Map<string, Set<ARIARole>>;
export type MapOfElementsToRoles = Map<string, Set<ARIARole>>;

export const roles: MapOfAriaRoles = new Map([
	[
		"checkbox",
		{
			props: ["aria-checked", "aria-readonly"],
			requiredProps: ["aria-checked"],
			superClassRole: ["switch", "menuitemcheckbox", "widget"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "input",
						attributes: [
							{
								name: "type",
								value: "checkbox",
							},
						],
					},
				},
			],
		},
	],
	[
		"radio",
		{
			props: ["aria-checked", "aria-readonly"],
			requiredProps: ["aria-checked"],
			superClassRole: ["menuitemradio", "widget"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "input",
						attributes: [
							{
								name: "type",
								value: "radio",
							},
						],
					},
				},
			],
		},
	],
	[
		"switch",
		{
			props: ["aria-checked"],
			requiredProps: ["aria-checked"],
			superClassRole: ["checkbox", "widget"],
		},
	],
	[
		"option",
		{
			props: ["aria-selected"],
			requiredProps: ["aria-selected"],
			superClassRole: ["treeitem", "widget"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "option",
					},
				},
			],
		},
	],
	[
		"combobox",
		{
			props: ["aria-controls", "aria-expanded"],
			requiredProps: ["aria-controls", "aria-expanded"],
			superClassRole: ["select", "widget"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "select",
					},
				},
			],
		},
	],
	[
		"heading",
		{
			props: ["aria-level"],
			requiredProps: ["aria-level"],
			superClassRole: ["sectionhead"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "h1",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "h2",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "h3",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "h4",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "h5",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "h6",
					},
				},
			],
		},
	],
	[
		"spinbutton",
		{
			props: ["aria-valuemax", "aria-valuemin", "aria-valuenow"],
			requiredProps: ["aria-valuemax", "aria-valuemin", "aria-valuenow"],
			superClassRole: ["composite", "input", "range", "widget"],
		},
	],
	[
		"slider",
		{
			props: ["aria-valuemax", "aria-valuemin", "aria-valuenow"],
			requiredProps: ["aria-valuemax", "aria-valuemin", "aria-valuenow"],
			superClassRole: ["input", "range", "widget"],
		},
	],
	[
		"separator",
		{
			props: ["aria-valuemax", "aria-valuemin", "aria-valuenow"],
			requiredProps: ["aria-valuemax", "aria-valuemin", "aria-valuenow"],
			superClassRole: ["structure", "widget"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "hr",
					},
				},
			],
		},
	],

	[
		"scrollbar",
		{
			props: [
				"aria-valuemax",
				"aria-valuemin",
				"aria-valuenow",
				"aria-orientation",
				"aria-controls",
			],
			requiredProps: [
				"aria-valuemax",
				"aria-valuemin",
				"aria-valuenow",
				"aria-orientation",
				"aria-controls",
			],
			superClassRole: ["range", "widget"],
		},
	],

	[
		"button",
		{
			props: ["aria-expanded", "aria-pressed"],
			requiredProps: [],
			superClassRole: ["roletype", "widget", "command"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "button",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "input",
						attributes: [
							{
								name: "type",
								value: "button",
							},
						],
					},
				},
			],
		},
	],
	[
		"article",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["document"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "article",
					},
				},
			],
		},
	],
	[
		"dialog",
		{
			props: ["aria-label", "aria-labelledby"],
			requiredProps: [],
			superClassRole: ["window"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "dialog",
					},
				},
			],
		},
	],
]);

export const elementsToConcepts: MapOfElementsToConcepts = new Map();
export const elementsToRoles: MapOfElementsToRoles = new Map();

for (const [, attributes] of roles) {
	if (attributes.baseConcepts) {
		attributes.baseConcepts.forEach(({module, concept}) => {
			if (module === "HTML") {
				if (!elementsToConcepts.has(concept.name)) {
					elementsToConcepts.set(
						concept.name,
						new Set(attributes.superClassRole),
					);
				}
			}
		});
	}
}
for (const [, attributes] of roles) {
	if (attributes.baseConcepts) {
		attributes.baseConcepts.forEach(({module, concept}) => {
			if (module === "HTML") {
				if (!elementsToConcepts.has(concept.name)) {
					elementsToConcepts.set(
						concept.name,
						new Set(attributes.superClassRole),
					);
				}
			}
		});
	}
}

export function isRoleInteractive(role: ARIARoleDefinition) {
	return role.superClassRole.includes("widget");
}

export function isElementInteractive(elementName: string) {
	let role: ARIARoleDefinition | undefined;
	for (const [, roleInfo] of roles) {
		if (roleInfo.baseConcepts) {
			const elementMatched = roleInfo.baseConcepts.some(({concept}) =>
				concept.name === elementName
			);
			if (elementMatched) {
				role = roleInfo;
				break;
			}
		}
	}

	if (role) {
		return isRoleInteractive(role);
	}
	return false;
}

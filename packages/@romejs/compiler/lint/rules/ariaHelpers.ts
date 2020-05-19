export type MapOfARIAPropertyDefinitions = Map<
	ARIAProperty,
	ARIAPropertyDefinition
>;

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

export type ARIAPropertyDefinition = {
	type:
		| "string"
		| "id"
		| "idlist"
		| "integer"
		| "number"
		| "boolean"
		| "token"
		| "tokenlist"
		| "tristate";
	value?: Array<string | boolean>;
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


export type ARIARoleDefinition = {
	/* aria-* properties and states allowed on this role. */
	props: ARIAProperty[],
	/* aria-* properties and states required on this role. */
	requiredProps: ARIAProperty[],
};

export type MapOfAriaRoles = Map<
	string,
	ARIARoleDefinition
	>;

export const roles: MapOfAriaRoles = new Map([
	['checkbox', {
		props: ['aria-checked', 'aria-readonly'],
		requiredProps: ['aria-checked']
	}],
]);



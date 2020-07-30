import {ARIARoleDefinition} from "@internal/compiler/lint/utils/aria";

export type MapOfAriaRoles = Map<string, ARIARoleDefinition>;

const ariaRolesMap: MapOfAriaRoles = new Map([
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
	[
		"alert",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "XForms",
					concept: {
						name: "alert",
					},
				},
			],
		},
	],
	[
		"alertdialog",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["alert", "dialog"],
			baseConcepts: [
				{
					module: "XForms",
					concept: {
						name: "alert",
					},
				},
			],
		},
	],
	[
		"application",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["structure"],
		},
	],
	[
		"banner",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["landmark"],
		},
	],
	[
		"cell",
		{
			props: ["aria-colindex", "aria-colspan", "aria-rowindex", "aria-rowspan"],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "td",
					},
				},
			],
		},
	],
	[
		"columnheader",
		{
			props: ["aria-sort"],
			requiredProps: [],
			superClassRole: ["cell", "gridcell", "sectionhead"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "th",
						attributes: [
							{
								name: "scope",
								value: "col",
							},
						],
					},
				},
			],
		},
	],
	[
		"definition",
		{
			props: ["aria-labelledby"],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "dd",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "dfn",
					},
				},
			],
		},
	],
	[
		"feed",
		{
			props: ["aria-busy", "aria-setsize"],
			requiredProps: [],
			superClassRole: ["section"],
		},
	],
	[
		"figure",
		{
			props: ["aria-label", "aria-labelledby"],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "figure",
					},
				},
			],
		},
	],
	[
		"form",
		{
			props: ["aria-label", "aria-labelledby"],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "form",
					},
				},
			],
		},
	],
	[
		"grid",
		{
			props: ["aria-level", "aria-multiselectable", "aria-readonly"],
			requiredProps: [],
			superClassRole: ["composite", "table"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "table",
					},
				},
			],
		},
	],
	[
		"gridcell",
		{
			props: ["aria-readonly", "aria-required", "aria-selected"],
			requiredProps: [],
			superClassRole: ["cell", "widget"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "td",
					},
				},
			],
		},
	],
	[
		"group",
		{
			props: ["aria-activedescendant"],
			requiredProps: [],
			superClassRole: ["row", "select", "toolbar"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "fieldset",
					},
				},
			],
		},
	],
	[
		"img",
		{
			props: ["aria-activedescendant"],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "img",
					},
				},
			],
		},
	],
	[
		"link",
		{
			props: ["aria-expanded"],
			requiredProps: [],
			superClassRole: ["command"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "a",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "link",
					},
				},
			],
		},
	],
	[
		"list",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "ol",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "ul",
					},
				},
			],
		},
	],
	[
		"listbox",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["select"],
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
		"listitem",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "li",
					},
				},
			],
		},
	],
	[
		"log",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["section"],
		},
	],
	[
		"main",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["landmark"],
		},
	],
	[
		"menu",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["select"],
		},
	],
	[
		"menubar",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["toolbar"],
		},
	],
	[
		"menuitem",
		{
			props: ["aria-posinset", "aria-setsize"],
			requiredProps: [],
			superClassRole: ["command"],
		},
	],
	[
		"menuitemcheckbox",
		{
			props: [],
			requiredProps: ["aria-checked"],
			superClassRole: ["checkbox", "menuitem"],
		},
	],
	[
		"menuitemradio",
		{
			props: [],
			requiredProps: ["aria-checked"],
			superClassRole: ["menuitemcheckbox", "radio"],
		},
	],
	[
		"navigation",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["landmark"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "nav",
					},
				},
			],
		},
	],
	[
		"progressbar",
		{
			props: [],
			requiredProps: ["aria-valuenow", "aria-valuemin", "aria-valuemax"],
			superClassRole: ["range"],
		},
	],
	[
		"radiogroup",
		{
			props: ["aria-readonly", "aria-required"],
			requiredProps: [],
			superClassRole: ["range"],
		},
	],
	[
		"row",
		{
			props: ["aria-colindex", "aria-level", "aria-rowindex", "aria-selected"],
			requiredProps: [],
			superClassRole: ["group", "widget"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "tr",
					},
				},
			],
		},
	],
	[
		"rowgroup",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["structure"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "tbody",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "tfoot",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "thead",
					},
				},
			],
		},
	],
	[
		"rowheader",
		{
			props: ["aria-sort"],
			requiredProps: [],
			superClassRole: ["cell", "gridcell", "sectionhead"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "th",
						attributes: [
							{
								name: "scope",
								value: "row",
							},
						],
					},
				},
			],
		},
	],
	[
		"searchbox",
		{
			props: [
				"aria-activedescendant",
				"aria-autocomplete",
				"aria-multiline",
				"aria-placeholder",
				"aria-readonly",
				"aria-required",
			],
			requiredProps: [],
			superClassRole: ["textbox"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "input",
						attributes: [
							{
								name: "type",
								value: "search",
							},
						],
					},
				},
			],
		},
	],
	[
		"tab",
		{
			props: ["aria-posinset", "aria-selected", "aria-setsize"],
			requiredProps: [],
			superClassRole: ["sectionhead", "widget"],
		},
	],
	[
		"table",
		{
			props: ["aria-colcount", "aria-rowcount"],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "table",
					},
				},
			],
		},
	],
	[
		"tablist",
		{
			props: ["aria-level", "aria-multiselectable", "aria-orientation"],
			requiredProps: [],
			superClassRole: ["composite"],
		},
	],
	[
		"term",
		{
			props: [],
			requiredProps: [],
			superClassRole: ["section"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "dt",
					},
				},
			],
		},
	],
	[
		"textbox",
		{
			props: [
				"aria-activedescendant",
				"aria-autocomplete",
				"aria-multiline",
				"aria-placeholder",
				"aria-readonly",
				"aria-required",
			],
			requiredProps: [],
			superClassRole: ["input"],
			baseConcepts: [
				{
					module: "HTML",
					concept: {
						name: "textarea",
					},
				},
				{
					module: "HTML",
					concept: {
						name: "input",
						attributes: [
							{
								name: "type",
								value: "text",
							},
						],
					},
				},
			],
		},
	],
	[
		"toolbar",
		{
			props: ["aria-orientation"],
			requiredProps: [],
			superClassRole: ["group"],
		},
	],
	[
		"tree",
		{
			props: ["aria-multiselectable", "aria-required"],
			requiredProps: [],
			superClassRole: ["select"],
		},
	],
]);

export default ariaRolesMap;

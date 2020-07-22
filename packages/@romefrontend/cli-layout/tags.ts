import {
	MarkupColor,
	MarkupLineWrapMode,
	MarkupTagName,
	MarkupTokenType,
} from "./types";

type AttributeValidator = (
	value: string,
	key: string,
) => undefined | string | boolean | number;
type AttributeValidators = Map<string, AttributeValidator>;

const stringValidator: AttributeValidator = (value) => value;

const booleanValidator: AttributeValidator = (value, key) => {
	if (value === "false") {
		return false;
	}

	if (value === "true" || value === key) {
		return true;
	}

	return undefined;
};
const numberValidator: AttributeValidator = (value) => {
	const num = parseFloat(value);
	if (isNaN(num)) {
		return undefined;
	} else {
		return num;
	}
};

export const globalAttributes: AttributeValidators = new Map([
	["emphasis", booleanValidator],
	["dim", booleanValidator],
]);

// Tags and their corresponding supported attributes and validators
export const tags: Map<MarkupTagName, AttributeValidators> = new Map();

tags.set("emphasis", new Map());
tags.set(
	"number",
	new Map([
		["approx", booleanValidator],
		["pluralSuffix", stringValidator],
		["singularSuffix", stringValidator],
	]),
);
tags.set("indent", new Map());
tags.set(
	"view",
	new Map([
		["extraSoftWrapIndent", numberValidator],
		["lineWrap", lineWrapValidator],
		["align", alignValidator],
	]),
);
tags.set(
	"viewLinePrefix",
	new Map([["type", stringValidator], ["align", alignValidator]]),
);
tags.set(
	"viewPointer",
	new Map([
		["char", stringValidator],
		["line", numberValidator],
		["start", numberValidator],
		["end", numberValidator],
	]),
);
tags.set(
	"grammarNumber",
	new Map([
		["plural", stringValidator],
		["singular", stringValidator],
		["none", stringValidator],
	]),
);
tags.set("hyperlink", new Map([["target", stringValidator]]));
tags.set(
	"filelink",
	new Map([
		["target", stringValidator],
		["column", numberValidator],
		["line", numberValidator],
	]),
);
tags.set("inverse", new Map());
tags.set("dim", new Map());
tags.set("filesize", new Map());
tags.set("duration", new Map([["approx", booleanValidator]]));
tags.set("italic", new Map());
tags.set("underline", new Map());
tags.set("strike", new Map());
tags.set("token", new Map([["type", validateTokenType]]));
tags.set("error", new Map());
tags.set("success", new Map());
tags.set("warn", new Map());
tags.set("info", new Map());
tags.set("code", new Map());
tags.set("color", new Map([["fg", validateColor], ["bg", validateColor]]));
tags.set(
	"highlight",
	new Map([["i", numberValidator], ["legend", booleanValidator]]),
);
tags.set("table", new Map());
tags.set("tr", new Map());
tags.set("td", new Map([["align", alignValidator]]));
tags.set("hr", new Map());
tags.set(
	"pad",
	new Map([["width", numberValidator], ["align", alignValidator]]),
);
tags.set("li", new Map());
tags.set("ul", new Map());
tags.set(
	"ol",
	new Map([["reversed", booleanValidator], ["start", numberValidator]]),
);

// Tags that only support certain other tags as their children
export const tagsToOnlyChildren: Map<MarkupTagName, Array<MarkupTagName>> = new Map();
tagsToOnlyChildren.set("table", ["tr"]);
tagsToOnlyChildren.set("tr", ["td"]);
tagsToOnlyChildren.set("ol", ["li"]);
tagsToOnlyChildren.set("ul", ["li"]);

// Tags that should only be children of other tags
export const tagsToOnlyParent: Map<MarkupTagName, Array<MarkupTagName>> = new Map();
tagsToOnlyParent.set("tr", ["table"]);
tagsToOnlyParent.set("td", ["tr"]);
tagsToOnlyParent.set("li", ["ol", "ul"]);
tagsToOnlyParent.set("viewLinePrefix", ["view"]);
tagsToOnlyParent.set("viewPointer", ["view"]);

function alignValidator(align: undefined | string): undefined | string {
	if (align === "left" || align === "right") {
		return align;
	}

	return undefined;
}

// Validators
export function lineWrapValidator(
	mode: undefined | string,
): undefined | MarkupLineWrapMode {
	if (mode === "none" || mode === "word-break" || mode === "char-break") {
		return mode;
	}

	return undefined;
}

export function validateTokenType(
	type: undefined | string,
): undefined | MarkupTokenType {
	switch (type) {
		case "boolean":
		case "keyword":
		case "number":
		case "regex":
		case "string":
		case "comment":
		case "operator":
		case "punctuation":
		case "variable":
		case "attr-name":
		case "function":
		case "attr-value":
		case "attr-equals":
		case "tag":
			return type;

		default:
			return undefined;
	}
}

export function validateColor(
	color: undefined | string,
): undefined | MarkupColor {
	switch (color) {
		case "black":
		case "brightBlack":
		case "red":
		case "brightRed":
		case "green":
		case "brightGreen":
		case "yellow":
		case "brightYellow":
		case "blue":
		case "brightBlue":
		case "magenta":
		case "brightMagenta":
		case "cyan":
		case "brightCyan":
		case "white":
		case "brightWhite":
			return color;

		default:
			return undefined;
	}
}

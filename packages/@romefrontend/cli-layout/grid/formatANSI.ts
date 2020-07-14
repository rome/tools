import {
	MarkupColor,
	MarkupFormatOptions,
	MarkupTokenType,
	TagNode,
} from "../types";
import {formatAnsi} from "../ansi";
import {buildFileLink} from "../util";
import OneDarkPro from "../syntax-theme/OneDarkPro.json";
import {Dict} from "@romefrontend/typescript-helpers";
import {Consumer, consumeUnknown} from "@romefrontend/consume";
import {TerminalFeatures} from "@romefrontend/environment";
import {validateColor, validateTokenType} from "../tags";

export function ansiFormatText(
	{name: tagName, attributes}: TagNode,
	value: string,
	opts: MarkupFormatOptions,
	features: TerminalFeatures,
): string {
	switch (tagName) {
		case "hyperlink": {
			if (features.hyperlinks) {
				return formatAnsi.hyperlink(
					attributes.get("target").asString(""),
					value,
				);
			} else {
				return value;
			}
		}

		case "filelink": {
			if (features.hyperlinks) {
				const {filename} = buildFileLink(attributes, opts);
				return formatAnsi.hyperlink(value, `file://${filename}`);
			} else {
				return value;
			}
		}
	}

	if (!features.color) {
		return value;
	}

	switch (tagName) {
		case "inverse":
			return formatAnsi.inverse(value);

		case "emphasis":
			return formatAnsi.bold(value);

		case "dim":
			return formatAnsi.dim(value);

		case "italic":
			return formatAnsi.italic(value);

		case "underline":
			return formatAnsi.underline(value);

		case "strike":
			return formatAnsi.strikethrough(value);

		case "error":
			return formatAnsi.red(value);

		case "success":
			return formatAnsi.green(value);

		case "warn":
			return formatAnsi.yellow(value);

		case "info":
			return formatAnsi.blue(value);

		case "command":
			return formatAnsi.italic(value);

		case "highlight": {
			const index = Math.min(0, attributes.get("i").asNumber(0));
			const fn = ansiHighlightFactories[index % ansiHighlightFactories.length];
			return fn(value);
		}

		case "color":
			return formatAnsiBackground(
				validateColor(attributes.get("bg").asStringOrVoid()),
				formatAnsiForeground(
					validateColor(attributes.get("fg").asStringOrVoid()),
					value,
				),
			);

		case "token":
			return formatToken(
				validateTokenType(attributes.get("type").asStringOrVoid()),
				value,
				opts,
			);

		default:
			return value;
	}
}

type FontStyle = "normal" | "italic";

type TokenFormat = {
	rgb?: [number, number, number];
	fontStyle?: FontStyle;
};

// rome-ignore lint/js/noUnusedVariables
type TokenFormats = {[type in MarkupTokenType]?: TokenFormat};

const scopeToTokenTypes: Dict<Array<MarkupTokenType>> = {
	"constant": ["number", "boolean"],
	"constant.numeric": ["number"],
	"constant.language.boolean": ["boolean"],
	"variable.other.constant": ["boolean"],

	"string": ["string"],
	"string.regexp": ["regex"],

	"comment": ["comment"],
	"entity.name.function": ["function"],
	//"": "operator"],
	"punctuation": ["punctuation"],
	//"variable": ["variable"],
	"keyword": ["keyword"],

	"entity.name.tag.html": ["tag"],
	"punctuation.separator.key-value.html": ["attr-equals"],
	"string.quoted.double.html": ["attr-value"],
	"entity.other.attribute-name": ["attr-name"],
	"entity.other.attribute-name.js": ["attr-name"],
};

function normalizeFontStyle(style: undefined | string): FontStyle {
	switch (style) {
		case "italic":
			return style;

		default:
			return "normal";
	}
}

const tokenColorsCache: Map<Consumer, TokenFormats> = new Map();
let defaultTokenColors: undefined | TokenFormats;

function getTokenColors(consumer: undefined | Consumer): TokenFormats {
	if (consumer === undefined) {
		if (defaultTokenColors === undefined) {
			defaultTokenColors = getTokenColors(
				consumeUnknown(OneDarkPro, "parse/vscodeTheme"),
			);
		}

		return defaultTokenColors;
	}

	const cached = tokenColorsCache.get(consumer);
	if (cached !== undefined) {
		return cached;
	}

	const tokenTypeFormat: TokenFormats = {};
	tokenColorsCache.set(consumer, tokenTypeFormat);

	for (const prop of consumer.get("tokenColors").asArray()) {
		const settings = prop.get("settings");
		const scope = prop.get("scope");
		const scopes = Array.isArray(scope.asUnknown())
			? scope.asArray().map((elem) => elem.asString())
			: scope.asString().split(",").map((scope) => scope.trim());

		for (const scope of scopes) {
			const tokenTypes = scopeToTokenTypes[scope];
			if (tokenTypes === undefined) {
				continue;
			}

			for (const tokenType of tokenTypes) {
				const existing = tokenTypeFormat[tokenType];

				const newSettings: TokenFormat = {};

				if (settings.has("foreground")) {
					newSettings.rgb = hexToRgb(settings.get("foreground").asString());
				}

				if (settings.has("fontStyle")) {
					newSettings.fontStyle = normalizeFontStyle(
						settings.get("fontStyle").asString(),
					);
				}

				tokenTypeFormat[tokenType] = {
					...existing,
					...newSettings,
				};
			}
		}
	}

	return tokenTypeFormat;
}

function formatToken(
	type: undefined | MarkupTokenType,
	value: string,
	opts: MarkupFormatOptions,
): string {
	if (type === undefined) {
		return value;
	}

	const tokenTypeFormat = getTokenColors((opts?.userConfig)?.syntaxTheme);
	const format = tokenTypeFormat[type];

	if (format === undefined) {
		return value;
	}

	if (format.fontStyle === "italic") {
		value = formatAnsi.italic(value);
	}

	if (format.rgb === undefined) {
		return value;
	}

	return formatAnsi.rgb(value, format.rgb);
}

function hexToRgb(hex: undefined | string): [number, number, number] {
	if (hex === undefined) {
		throw new Error("No color string passed");
	}

	const match = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
	if (match == null) {
		throw new Error(`${hex} is not a valid hex color`);
	}

	return [
		parseInt(match[1], 16),
		parseInt(match[2], 16),
		parseInt(match[3], 16),
	];
}

// TODO fill this
const ansiHighlightFactories: Array<(str: string) => string> = [
	formatAnsi.magenta,
	formatAnsi.cyan,
];

function formatAnsiBackground(bg: undefined | MarkupColor, text: string): string {
	if (bg === undefined) {
		return text;
	}

	switch (bg) {
		case "black":
			return formatAnsi.bgBlack(text);

		case "brightBlack":
			return formatAnsi.bgBrightBlack(text);

		case "red":
			return formatAnsi.bgRed(text);

		case "brightRed":
			return formatAnsi.bgBrightRed(text);

		case "green":
			return formatAnsi.bgGreen(text);

		case "brightGreen":
			return formatAnsi.bgBrightGreen(text);

		case "yellow":
			return formatAnsi.bgYellow(text);

		case "brightYellow":
			return formatAnsi.bgBrightYellow(text);

		case "blue":
			return formatAnsi.bgBlue(text);

		case "brightBlue":
			return formatAnsi.bgBrightBlue(text);

		case "magenta":
			return formatAnsi.bgMagenta(text);

		case "brightMagenta":
			return formatAnsi.bgBrightMagenta(text);

		case "cyan":
			return formatAnsi.bgCyan(text);

		case "brightCyan":
			return formatAnsi.bgBrightCyan(text);

		case "white":
			return formatAnsi.bgWhite(text);

		case "brightWhite":
			return formatAnsi.bgBrightWhite(text);

		default:
			return text;
	}
}

function formatAnsiForeground(fg: undefined | MarkupColor, text: string): string {
	if (fg === undefined) {
		return text;
	}

	switch (fg) {
		case "black":
			return formatAnsi.black(text);

		case "brightBlack":
			return formatAnsi.brightBlack(text);

		case "red":
			return formatAnsi.red(text);

		case "brightRed":
			return formatAnsi.brightRed(text);

		case "green":
			return formatAnsi.green(text);

		case "brightGreen":
			return formatAnsi.brightGreen(text);

		case "yellow":
			return formatAnsi.yellow(text);

		case "brightYellow":
			return formatAnsi.brightYellow(text);

		case "blue":
			return formatAnsi.blue(text);

		case "brightBlue":
			return formatAnsi.brightBlue(text);

		case "magenta":
			return formatAnsi.magenta(text);

		case "brightMagenta":
			return formatAnsi.brightMagenta(text);

		case "cyan":
			return formatAnsi.cyan(text);

		case "brightCyan":
			return formatAnsi.brightCyan(text);

		case "white":
			return formatAnsi.white(text);

		case "brightWhite":
			return formatAnsi.brightWhite(text);

		default:
			return text;
	}
}

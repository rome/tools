import {formatAnsi, formatAnsiRGB} from "./ansi";
import OneDarkPro from "./syntax-theme/OneDarkPro.json";
import {Dict} from "@internal/typescript-helpers";
import {Consumer, consumeUnknown} from "@internal/consume";
import {
	MarkupColor,
	MarkupParsedTag,
	MarkupTokenType,
	buildFileLink,
	validateColor,
	validateTokenType,
} from "@internal/markup";
import Grid from "./Grid";

export function ansiFormatText(
	{name: tagName, attributes}: MarkupParsedTag,
	value: string,
	grid: Grid,
): string {
	const {features} = grid;

	switch (tagName) {
		case "hyperlink": {
			if (features.hyperlinks) {
				return formatAnsi.hyperlink(
					attributes.get("target").asString(value),
					value,
				);
			} else {
				return value;
			}
		}

		case "filelink": {
			if (features.hyperlinks) {
				const {filename} = buildFileLink(attributes, grid.options);
				return formatAnsi.hyperlink(value, `file://${filename}`);
			} else {
				return value;
			}
		}
	}

	if (features.colorDepth === 1) {
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

		case "code":
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
				grid,
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

type Theme = {
	kind: "default" | "user";
	tokens: {[type in MarkupTokenType]?: TokenFormat};
};

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

const tokenColorsCache: Map<Consumer, Theme> = new Map();
let defaultTokenColors: undefined | Theme;

function getTokenColors(consumer: undefined | Consumer): Theme {
	if (consumer === undefined) {
		if (defaultTokenColors === undefined) {
			defaultTokenColors = {
				...getTokenColors(consumeUnknown(OneDarkPro, "parse/vscodeTheme")),
				kind: "default",
			};
		}

		return defaultTokenColors;
	}

	const cached = tokenColorsCache.get(consumer);
	if (cached !== undefined) {
		return cached;
	}

	const theme: Theme = {
		kind: "user",
		tokens: {},
	};
	tokenColorsCache.set(consumer, theme);

	for (const prop of consumer.get("tokenColors").asIterable()) {
		const settings = prop.get("settings");
		const scope = prop.get("scope");
		const scopes = Array.isArray(scope.asUnknown())
			? scope.asMappedArray((elem) => elem.asString())
			: scope.asString().split(",").map((scope) => scope.trim());

		for (const scope of scopes) {
			const tokenTypes = scopeToTokenTypes[scope];
			if (tokenTypes === undefined) {
				continue;
			}

			for (const tokenType of tokenTypes) {
				const existing = theme.tokens[tokenType];

				const newSettings: TokenFormat = {};

				if (settings.has("foreground")) {
					newSettings.rgb = hexToRgb(settings.get("foreground").asString());
				}

				if (settings.has("fontStyle")) {
					newSettings.fontStyle = normalizeFontStyle(
						settings.get("fontStyle").asString(),
					);
				}

				theme.tokens[tokenType] = {
					...existing,
					...newSettings,
				};
			}
		}
	}

	return theme;
}

function formatToken(
	type: undefined | MarkupTokenType,
	value: string,
	grid: Grid,
): string {
	if (type === undefined) {
		return value;
	}

	const theme = getTokenColors((grid.options?.userConfig)?.syntaxTheme);

	// Only use our default syntax theme when we are confident the terminal has a dark background
	if (theme.kind === "default" && grid.features.background !== "dark") {
		return value;
	}

	const format = theme.tokens[type];
	if (format === undefined) {
		return value;
	}

	if (format.fontStyle === "italic") {
		value = formatAnsi.italic(value);
	}

	if (format.rgb === undefined) {
		return value;
	}

	return formatAnsiRGB({
		value,
		color: format.rgb,
		features: grid.features,
	});
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

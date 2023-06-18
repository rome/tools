import {
	IndentStyle,
	PlaygroundSettings,
	PrettierOutput,
	QuoteProperties,
	QuoteStyle,
	Semicolons,
	TrailingComma,
	defaultPlaygroundState,
} from "../types";
import { isJsonFilename, isTypeScriptFilename } from "../utils";
import prettier, { Options as PrettierOptions } from "prettier";
// @ts-expect-error
import parserBabel from "prettier/esm/parser-babel";

let settings = defaultPlaygroundState.settings;

self.addEventListener("message", (e) => {
	switch (e.data.type) {
		case "updateSettings": {
			settings = e.data.settings as PlaygroundSettings;
			break;
		}

		case "format": {
			const {
				lineWidth,
				indentStyle,
				indentWidth,
				quoteStyle,
				jsxQuoteStyle,
				quoteProperties,
				trailingComma,
				semicolons,
			} = settings;
			const code = e.data.code as string;
			const filename = e.data.filename as string;

			const prettierOutput = formatWithPrettier(code, {
				lineWidth,
				indentStyle,
				indentWidth,
				filepath: filename,
				quoteStyle,
				jsxQuoteStyle,
				quoteProperties,
				trailingComma,
				semicolons,
			});

			self.postMessage({
				type: "formatted",
				filename,
				prettierOutput,
			});

			break;
		}

		default:
			console.error(`Unknown message ${e.data.type}.`);
	}
});

function formatWithPrettier(
	code: string,
	options: {
		lineWidth: number;
		indentStyle: IndentStyle;
		indentWidth: number;
		filepath: string;
		quoteStyle: QuoteStyle;
		jsxQuoteStyle: QuoteStyle;
		quoteProperties: QuoteProperties;
		trailingComma: TrailingComma;
		semicolons: Semicolons;
	},
): PrettierOutput {
	try {
		const prettierOptions: PrettierOptions = {
			useTabs: options.indentStyle === IndentStyle.Tab,
			tabWidth: options.indentWidth,
			printWidth: options.lineWidth,
			filepath: options.filepath,
			plugins: [parserBabel],
			parser: getPrettierParser(options.filepath),
			singleQuote: options.quoteStyle === QuoteStyle.Single,
			jsxSingleQuote: options.jsxQuoteStyle === QuoteStyle.Single,
			quoteProps: options.quoteProperties,
			trailingComma: options.trailingComma,
			semi: options.semicolons === Semicolons.Always,
		};

		// @ts-expect-error
		const debug = prettier.__debug;
		const document = debug.printToDoc(code, prettierOptions);

		// formatDoc must be before printDocToString because printDocToString mutates the document and breaks the ir
		const ir = debug.formatDoc(document, {
			parser: "babel",
			plugins: [parserBabel],
		});

		const formattedCode = debug.printDocToString(
			document,
			prettierOptions,
		).formatted;

		return {
			type: "SUCCESS",
			code: formattedCode,
			ir,
		};
	} catch (err: unknown) {
		if (err instanceof SyntaxError) {
			return {
				type: "ERROR",
				stack: err.message,
			};
		} else {
			return {
				type: "ERROR",
				stack: (err as Error).stack ?? "",
			};
		}
	}
}

function getPrettierParser(filename: string): string {
	if (isTypeScriptFilename(filename)) {
		return "babel-ts";
	} else if (isJsonFilename(filename)) {
		return "json5";
	} else {
		return "babel";
	}
}

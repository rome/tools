import prettier, { Options as PrettierOptions } from "prettier";
import {
	IndentStyle,
	QuoteStyle,
	QuoteProperties,
	TrailingComma,
	Semicolons,
	PrettierOutput,
	defaultPlaygroundState,
	PlaygroundSettings,
} from "../types";
// @ts-ignore
import parserBabel from "prettier/esm/parser-babel";
import { isTypeScriptFilename } from "../utils";

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
			quoteProps: options.quoteProperties,
			trailingComma: options.trailingComma,
			semi: options.semicolons === Semicolons.Always,
		};

		// @ts-ignore
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
	} else {
		return "babel";
	}
}

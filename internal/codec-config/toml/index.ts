import {ParserOptions, TokenBase} from "@internal/parser-core";
import {
	ConfigParserOptions,
	ConfigParserResult,
	PartialConfigHandler,
	PartialConsumeConfigResult,
} from "@internal/codec-config/types";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {tomlParser} from "./tokenizer";
import {parseRoot} from "./parse";
import {stringifyTOMLFromConsumer} from "./stringify";

export const toml: PartialConfigHandler = {
	type: "toml",
	language: "toml",
	extensions: ["toml", "ini"],
	jsonSuperset: false,

	parseExtra(opts: ParserOptions): ConfigParserResult {
		const parser = tomlParser.create(opts);

		const root = parseRoot(parser);

		parser.finalize();

		return {
			type: "toml",
			value: root,
			// TODO position tracking
			context: {
				category: DIAGNOSTIC_CATEGORIES.parse,
				categoryValue: "toml",
				normalizeKey: (key) => key,
				getDiagnosticLocation: () => ({
					path: parser.path,
				}),
				getOriginalValue: () => undefined,
			},
			comments: parser.state.pathComments,
		};
	},

	tokenize(opts: ConfigParserOptions): TokenBase[] {
		return tomlParser.create(opts).getAllTokens();
	},

	stringifyFromConsumer(opts: PartialConsumeConfigResult): string {
		return stringifyTOMLFromConsumer(opts.consumer, opts.comments);
	},
};

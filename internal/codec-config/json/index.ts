import {jsonParser, parseJSONExtra} from "@internal/codec-config/json/parse";
import {TokenValues} from "@internal/parser-core";
import {Tokens} from "./types";
import {
	ConfigParserOptions,
	ConfigType,
	PartialConfigHandler,
	PartialConsumeConfigResult,
} from "@internal/codec-config/types";
import {stringifyJSON5RootConsumer} from "./stringify-json5";

function createJSONParserMethods(
	type: ConfigType,
): Omit<PartialConfigHandler, "extensions" | "language" | "stringifyFromConsumer"> {
	return {
		type,
		jsonSuperset: true,

		parseExtra(opts) {
			return parseJSONExtra(opts, type);
		},

		tokenize(opts: ConfigParserOptions): TokenValues<Tokens>[] {
			return jsonParser.create(opts, {type}, {diagnosticLanguage: type}).getAllTokens();
		},
	};
}

export const json: PartialConfigHandler = {
	extensions: ["json"],
	language: "json",
	...createJSONParserMethods("json"),

	stringifyFromConsumer(opts: PartialConsumeConfigResult): string {
		const val = opts.consumer.asUnknown();
		const serial = JSON.stringify(val, null, "\t");
		if (serial === undefined) {
			return "undefined";
		} else {
			return serial;
		}
	},
};

export const json5: PartialConfigHandler = {
	extensions: ["json5"],
	language: "json5",
	...createJSONParserMethods("json5"),

	stringifyFromConsumer(opts: PartialConsumeConfigResult): string {
		return stringifyJSON5RootConsumer(opts.consumer, opts.comments);
	},
};

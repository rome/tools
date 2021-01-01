import {
	createJSONParser,
	parseJSONExtra,
} from "@internal/codec-config/json/parse";
import {TokenValues} from "@internal/parser-core";
import {Tokens} from "./types";
import {stringifyRootConsumer} from "./stringify";
import {
	ConfigParserOptions,
	JSONConfigType,
	PartialConfigHandler,
	PartialConsumeConfigResult,
} from "@internal/codec-config/types";

function createJSONParserMethods(
	type: JSONConfigType,
): Omit<PartialConfigHandler, "extensions" | "language"> {
	return {
		type,
		jsonSuperset: true,

		stringifyFromConsumer(opts: PartialConsumeConfigResult): string {
			return stringifyRootConsumer(opts.consumer, opts.comments, type);
		},

		parseExtra(opts) {
			return parseJSONExtra(opts, type);
		},

		tokenize(opts: ConfigParserOptions): TokenValues<Tokens>[] {
			return createJSONParser(opts, {type}, {diagnosticLanguage: type}).getAllTokens();
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
	...createJSONParserMethods("json"),
};

export const rjson: PartialConfigHandler = {
	...createJSONParserMethods("rjson"),
	extensions: ["rjson"],
	language: "rjson",
};

export const yaml: PartialConfigHandler = {
	...createJSONParserMethods("yaml"),
	extensions: ["yaml", "yml"],
	language: "yaml",

	stringifyFromConsumer(opts: PartialConsumeConfigResult): string {
		throw new Error("todo");
	},
};

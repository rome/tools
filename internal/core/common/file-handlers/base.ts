import {
	ExtensionCustomLintResult,
	ExtensionHandlerMethodInfo,
	PartialExtensionHandler,
} from "@internal/core/common/file-handlers/types";
import {consumeConfig, json, stringifyConfig} from "@internal/codec-config";
import {parseJS} from "@internal/js-parser";

export const configHandler: PartialExtensionHandler = {
	language: "unknown",
	hasTabs: true,

	capabilities: {
		lint: false,
		format: true,
	},

	async customFormat(
		info: ExtensionHandlerMethodInfo,
	): Promise<ExtensionCustomLintResult> {
		const {file, integrity, mtimeNs, worker} = info;

		const sourceText = await worker.readFileText(file);

		let formatted: string = sourceText;

		if (sourceText.length > 50_000) {
			// Fast path for big files
			consumeConfig({
				path: file.uid,
				input: sourceText,
				integrity,
			});
		} else {
			formatted = stringifyConfig(
				consumeConfig({
					path: file.uid,
					input: sourceText,
					integrity,
				}),
			);
		}

		return {
			mtimeNs,
			sourceText,
			diagnostics: [],
			suppressions: [],
			formatted,
		};
	},

	async parse({integrity, path, file, worker}) {
		const src = await worker.readFileText(file);

		// Parse the JSON to make sure it's valid
		const obj = consumeConfig({
			path: file.uid,
			input: src,
		}).consumer.asUnknown();

		const serial = json.stringify(obj);
		const sourceText = `export default ${serial};`;

		// TODO we could produce an AST from the consumer and even retain source locations

		return {
			// Shouldn't error
			ast: parseJS({input: sourceText, integrity, sourceType: "module", path}),
			sourceText,
			astModifiedFromSource: true,
		};
	},
};

export const ASSET_EXPORT_TEMPORARY_VALUE = "VALUE_INJECTED_BY_BUNDLER";

export const assetHandler: PartialExtensionHandler = {
	sourceTypeJS: "module",
	language: "unknown",
	hasTabs: false,
	canHaveScale: true,
	isAsset: true,
	capabilities: {
		lint: false,
		format: false,
	},

	async parse({path}) {
		// This exists just so analyzeDependencies has something to look at
		// When bundling we'll have custom logic in the compiler to handle assets and inject the correct string
		const sourceText = `export default '${ASSET_EXPORT_TEMPORARY_VALUE}';`;

		return {
			// Shouldn't error
			ast: parseJS({input: sourceText, sourceType: "module", path}),
			astModifiedFromSource: true,
			sourceText,
		};
	},
};

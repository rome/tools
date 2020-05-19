/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	consumeJSONExtra,
	parseJSON,
	stringifyJSON,
	stringifyRJSONFromConsumer,
} from "@romejs/codec-json";
import {createAbsoluteFilePath, createUnknownFilePath} from "@romejs/path";
import {
	ExtensionHandler,
	ExtensionHandlerMethodInfo,
	ExtensionLintResult,
} from "./types";
import {textHandler} from "./text";

export const jsonHandler: ExtensionHandler = {
	ext: "json",

	// analyzeDependencies shim
	...textHandler,

	async format(info: ExtensionHandlerMethodInfo): Promise<ExtensionLintResult> {
		const {file, worker} = info;
		const {uid} = file;

		const real = createAbsoluteFilePath(file.real);
		const sourceText = await worker.readFile(real);
		const path = createUnknownFilePath(uid);

		let formatted: string = sourceText;

		if (sourceText.length > 50_000) {
			// Fast path for big JSON files
			parseJSON({
				path,
				input: sourceText,
			});
		} else {
			const {consumer, comments, hasExtensions} = consumeJSONExtra({
				input: sourceText,
				path,
			});

			if (hasExtensions) {
				formatted = stringifyRJSONFromConsumer({consumer, comments});
			} else {
				formatted = String(stringifyJSON(consumer.asUnknown()));
			}
		}

		return {
			sourceText,
			diagnostics: [],
			suppressions: [],
			formatted,
		};
	},

	async toJavaScript({file, worker}) {
		const src = await worker.readFile(file.real);

		// Parse the JSON to make sure it's valid
		const obj = parseJSON({
			path: createUnknownFilePath(file.uid),
			input: src,
		});

		const rawJson = JSON.stringify(obj);
		const json: string = rawJson === undefined ? "undefined" : rawJson;

		// TODO handle unicode newlines here
		return {
			sourceText: `export default ${json};`,
			generated: true,
		};
	},
};

export const rjsonHandler: ExtensionHandler = {
	...jsonHandler,
	ext: "rjson",
};

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
} from "@internal/codec-json";
import {createAbsoluteFilePath, createUnknownPath} from "@internal/path";
import {
	ExtensionHandler,
	ExtensionHandlerMethodInfo,
	ExtensionLintResult,
} from "./types";
import {parseJS} from "@internal/js-parser";

// Format these with spaces since npm uses them otherwise it's extremely annoying
const SPACE_WHITELIST = ["package.json", "package-lock.json"];

export const jsonHandler: ExtensionHandler = {
	ext: "json",
	language: "json",
	hasTabs: true,
	capabilities: {
		lint: false,
		format: true,
	},

	async customFormat(
		info: ExtensionHandlerMethodInfo,
	): Promise<ExtensionLintResult> {
		const {file, mtime, worker} = info;
		const {uid} = file;

		const real = createAbsoluteFilePath(file.real);
		const sourceText = await worker.readFile(real);
		const path = createUnknownPath(uid);

		let formatted: string = sourceText;

		if (sourceText.length > 50_000) {
			// Fast path for big JSON files
			parseJSON({
				path,
				input: sourceText,
				mtime,
			});
		} else {
			const {consumer, comments, hasExtensions} = consumeJSONExtra({
				input: sourceText,
				path,
				mtime,
			});

			if (hasExtensions) {
				formatted = stringifyRJSONFromConsumer({consumer, comments}) + "\n";
			} else {
				formatted =
					String(
						stringifyJSON(
							consumer.asUnknown(),
							SPACE_WHITELIST.includes(real.getBasename()),
						),
					) + "\n";
			}
		}

		return {
			mtime,
			sourceText,
			diagnostics: [],
			suppressions: [],
			formatted,
		};
	},

	async parse({mtime, path, file, worker}) {
		const src = await worker.readFile(file.real);

		// Parse the JSON to make sure it's valid
		const obj = parseJSON({
			path: createUnknownPath(file.uid),
			input: src,
		});

		const rawJson = JSON.stringify(obj);
		const json: string = rawJson === undefined ? "undefined" : rawJson;
		const sourceText = `export default ${json};`;

		return {
			// Shouldn't error
			ast: parseJS({input: sourceText, mtime, sourceType: "module", path}),
			sourceText,
			astModifiedFromSource: true,
		};
	},
};

export const rjsonHandler: ExtensionHandler = {
	...jsonHandler,
	ext: "rjson",
};

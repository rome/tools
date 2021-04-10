/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {markup} from "@internal/markup";
import {splitLines} from "@internal/string-utils";
import highlightShell from "./highlightShell";
import highlightJS from "./highlightJS";
import highlightHTML from "./highlightHTML";
import highlightJSON from "./highlightJSON";
import {AnsiHighlightOptions, HighlightCodeResult} from "./types";
import {CONFIG_HANDLERS} from "@internal/codec-config";
import highlightCSS from "@internal/markup-syntax-highlight/highlightCSS";

// Max file size to avoid doing expensive highlighting for massive files - 100KB
// NB: This should probably be lower
const FILE_SIZE_MAX = 100_000;

export {AnsiHighlightOptions};

export function highlightCode(opts: AnsiHighlightOptions): HighlightCodeResult {
	if (opts.input.length < FILE_SIZE_MAX && opts.highlight) {
		switch (opts.language) {
			case "js":
				return highlightJS(
					opts,
					// js-parser does not accept an "unknown" sourceType
					opts.sourceTypeJS === undefined || opts.sourceTypeJS === "unknown"
						? "script"
						: opts.sourceTypeJS,
				);

			case "html":
				return highlightHTML(opts);

			case "css":
				return highlightCSS(opts);

			case "shell":
				return highlightShell(opts);
		}
	}

	for (const handler of CONFIG_HANDLERS) {
		if (handler.jsonSuperset && opts.language === handler.language) {
			return highlightJSON(opts, handler);
		}
	}

	return splitLines(opts.input).map((line) => markup`${line}`);
}

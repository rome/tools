/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProjectConfig} from "@romefrontend/project";
import {UnknownFilePath} from "@romefrontend/path";
import {ExtensionHandler} from "./types";
import {
	cjsHandler,
	jsHandler,
	jsxHandler,
	mjsHandler,
	tsHandler,
	tsxHandler,
} from "./javascript";
import {textHandler} from "./text";
import {jsonHandler, rjsonHandler} from "./json";
import {htmHandler, htmlHandler} from "./html";
import {parseJS} from "@romefrontend/js-parser";
// import {markdownHandler} from "@romefrontend/core/common/file-handlers/markdown";

type ExtensionsMap = Map<string, ExtensionHandler>;

export type GetFileHandlerResult = {
	ext: string;
	handler?: ExtensionHandler;
};

export function getFileHandlerExtensions(
	projectConfig: ProjectConfig,
): Array<string> {
	return [...DEFAULT_HANDLERS.keys(), ...projectConfig.files.assetExtensions];
}

export function getFileHandler(
	path: UnknownFilePath,
	projectConfig: ProjectConfig,
): GetFileHandlerResult {
	const basename = path.getBasename();

	const match = basename.match(/\.([a-zA-Z]+)$/);
	if (match == null) {
		return {ext: "", handler: undefined};
	}

	const ext: string = match[1];
	let handler = DEFAULT_HANDLERS.get(ext);

	// Allow setting custom assert extensions in the project config
	if (handler === undefined && projectConfig.files.assetExtensions.includes(ext)) {
		handler = assetHandler;
	}

	return {ext, handler};
}

export function getFileHandlerAssert(
	path: UnknownFilePath,
	projectConfig: ProjectConfig,
): Required<GetFileHandlerResult> {
	const {handler, ext} = getFileHandler(path, projectConfig);

	if (handler === undefined) {
		throw new Error(`No file handler found for '${path.join()}'`);
	} else {
		return {handler, ext};
	}
}

export const ASSET_EXPORT_TEMPORARY_VALUE = "VALUE_INJECTED_BY_BUNDLER";

const assetHandler: ExtensionHandler = {
	// analyzeDependencies shim
	...textHandler,
	ext: "unknown",
	canHaveScale: true,
	isAsset: true,

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

// Extensions that have a `lint` handler
export const LINTABLE_EXTENSIONS: Array<string> = [];

// Extensions that have a `format` handler
export const FORMATTABLE_EXTENSIONS: Array<string> = [];

function setHandler(handler: ExtensionHandler) {
	const {ext} = handler;

	if (handler.canLint || handler.canFormat) {
		LINTABLE_EXTENSIONS.push(ext);
	}

	if (handler.canFormat) {
		FORMATTABLE_EXTENSIONS.push(ext);
	}

	DEFAULT_HANDLERS.set(ext, handler);
}

const DEFAULT_HANDLERS: ExtensionsMap = new Map();

const DEFAULT_ASSET_EXTENSIONS = [
	"css",
	// Images
	"png",
	"jpg",
	"jpeg",
	"gif",
	"svg",
	// Video
	"webm",
	"mp4",
	"m4v",
	"avi",
	"mkv",
	// Audio
	"mp3",
	// Fonts
	"woff",
	"woff2",
	"eot",
	"ttf",
	"otf",
];

for (const ext of DEFAULT_ASSET_EXTENSIONS) {
	setHandler({...assetHandler, ext});
}

setHandler(jsHandler);
setHandler(jsxHandler);
setHandler(cjsHandler);
setHandler(mjsHandler);
setHandler(tsHandler);
setHandler(tsxHandler);
setHandler(jsonHandler);
setHandler(rjsonHandler);
setHandler(htmlHandler);
setHandler(htmHandler);
// setHandler(markdownHandler);

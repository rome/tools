/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProjectConfig} from "@romejs/project";
import {UnknownFilePath} from "@romejs/path";
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
	async toJavaScript() {
		// This exists just so analyzeDependencies has something to look at
		// When bundling we'll have custom logic in the compiler to handle assets and inject the correct string
		return {
			generated: true,
			sourceText: `export default '${ASSET_EXPORT_TEMPORARY_VALUE}';`,
		};
	},
};

// Extensions that have a `lint` handler
export const LINTABLE_EXTENSIONS: Array<string> = [];

// Extensions that have a `format` handler
export const FORMATTABLE_EXTENSIONS: Array<string> = [];

function setHandler(handler: ExtensionHandler) {
	const {ext} = handler;

	if (handler.lint !== undefined) {
		LINTABLE_EXTENSIONS.push(ext);
	}

	if (handler.format !== undefined) {
		FORMATTABLE_EXTENSIONS.push(ext);
	}

	DEFAULT_HANDLERS.set(ext, handler);
}

const DEFAULT_HANDLERS: ExtensionsMap = new Map();

const DEFAULT_ASSET_EXTENSIONS = [
	// Images
	"png",
	"jpg",
	"jpeg",
	"gif",
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
	setHandler({ext, ...assetHandler});
}

setHandler(jsHandler);
setHandler(jsxHandler);
setHandler(cjsHandler);
setHandler(mjsHandler);
setHandler(tsHandler);
setHandler(tsxHandler);
setHandler(jsonHandler);
setHandler(rjsonHandler);

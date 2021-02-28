/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProjectConfig} from "@internal/project";
import {Path} from "@internal/path";
import {ExtensionHandler, PartialExtensionHandler} from "./types";
import {
	cjsHandler,
	jsHandler,
	jsxHandler,
	mjsHandler,
	tsHandler,
	tsxHandler,
} from "./javascript";
import {htmlHandler} from "./html";
import {
	DiagnosticLanguage,
	createSingleDiagnosticError,
	descriptions,
} from "@internal/diagnostics";
import {markdownHandler} from "@internal/core/common/file-handlers/markdown";
import {
	assetHandler,
	configHandler,
} from "@internal/core/common/file-handlers/base";
import {CONFIG_HANDLERS} from "@internal/codec-config";
import {cssHandler} from "@internal/core/common/file-handlers/css";

type ExtensionsMap = Map<string, ExtensionHandler>;

export type GetFileHandlerResult = {
	ext: string;
	handler?: ExtensionHandler;
};

export function inferDiagnosticLanguageFromPath(
	path: undefined | Path,
	existing?: DiagnosticLanguage,
): DiagnosticLanguage {
	if (existing !== undefined && existing !== "unknown") {
		return existing;
	}
	if (path !== undefined) {
		const {handler} = getFileHandlerFromPath(path, undefined);
		if (handler !== undefined) {
			return handler.language;
		}
	}

	return "unknown";
}

export function getFileHandlerExtensions(
	projectConfig: undefined | ProjectConfig,
): string[] {
	if (projectConfig === undefined) {
		return [...DEFAULT_HANDLERS.keys()];
	} else {
		return [...DEFAULT_HANDLERS.keys(), ...projectConfig.files.assetExtensions];
	}
}

export function getFileHandlerFromPath(
	path: Path,
	projectConfig: undefined | ProjectConfig,
): GetFileHandlerResult {
	const basename = path.getBasename();

	const match = basename.match(/\.([a-zA-Z]+)$/);
	if (match == null) {
		return {ext: "", handler: undefined};
	}

	const ext: string = match[1];
	let handler = DEFAULT_HANDLERS.get(ext);

	// Allow setting custom assert extensions in the project config
	if (
		handler === undefined &&
		projectConfig !== undefined &&
		projectConfig.files.assetExtensions.includes(ext)
	) {
		handler = {ext, ...assetHandler};
	}

	return {ext, handler};
}

export function getFileHandlerFromPathAssert(
	path: Path,
	projectConfig: undefined | ProjectConfig,
): Required<GetFileHandlerResult> {
	const {handler, ext} = getFileHandlerFromPath(path, projectConfig);

	if (handler === undefined) {
		throw createSingleDiagnosticError({
			description: descriptions.FILES.NO_FILE_HANDLER(path),
			location: {
				path,
			},
		});
	} else {
		return {handler, ext};
	}
}

// Extensions that have a `lint` handler
export const LINTABLE_EXTENSIONS: string[] = [];

// Extensions that have a `format` handler
export const FORMATTABLE_EXTENSIONS: string[] = [];

function setHandler(ext: string, handler: PartialExtensionHandler) {
	if (handler.capabilities.lint || handler.capabilities.format) {
		LINTABLE_EXTENSIONS.push(ext);
	}

	if (handler.capabilities.format) {
		FORMATTABLE_EXTENSIONS.push(ext);
	}

	DEFAULT_HANDLERS.set(ext, {...handler, ext});
}

const DEFAULT_HANDLERS: ExtensionsMap = new Map();

const DEFAULT_ASSET_EXTENSIONS = [
	// Extra css types to be ignored while we don't have a proper integration
	["scss", "text/css"],
	["sass", "text/css"],
	["less", "text/css"],

	["png", "image/png"],
	["jpg", "image/jpeg"],
	["jpeg", "image/jpeg"],
	["gif", "image/gif"],
	["svg", "image/svg+xml"],
	["webp", "image/webp"],
	["apng", "image/apng"],
	["avif", "image/avif"],
	["webm", "video/webm"],
	["mp4", "video/mp4"],
	["avi", "video/x-msvideo"],
	["weba", "audio/webm"],
	["mp3", "audio/mpeg"],
	["wav", "audio/wav"],
	["woff", "font/woff"],
	["woff2", "font/woff2"],
	["eot", "application/vnd.ms-fontobject"],
	["ttf", "font/ttf"],
	["otf", "font/otf"],
];

for (const [ext, mime] of DEFAULT_ASSET_EXTENSIONS) {
	setHandler(ext, {
		...assetHandler,
		mime,
	});
}

setHandler("js", jsHandler);
setHandler("jsx", jsxHandler);
setHandler("cjs", cjsHandler);
setHandler("mjs", mjsHandler);
setHandler("ts", tsHandler);
setHandler("tsx", tsxHandler);
setHandler("html", htmlHandler);
setHandler("htm", htmlHandler);
setHandler("md", markdownHandler);
setHandler("css", cssHandler);

// Config
for (const handler of CONFIG_HANDLERS) {
	for (const ext of handler.extensions) {
		if (ext === "yaml" || ext === "yml" || ext === "toml" || ext === "ini") {
			// Temporarily disable WIP extensions
			continue;
		}

		setHandler(
			ext,
			{
				...configHandler,
				language: handler.language,
			},
		);
	}
}

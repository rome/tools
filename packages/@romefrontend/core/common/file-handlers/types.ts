/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FileReference} from "@romefrontend/core";
import {WorkerLintOptions, WorkerParseOptions} from "../bridges/WorkerBridge";
import Worker from "../../worker/Worker";
import {
	DiagnosticLanguage,
	DiagnosticSuppressions,
	Diagnostics,
} from "@romefrontend/diagnostics";
import * as compiler from "@romefrontend/compiler";
import {AnyRoot, ConstJSSourceType} from "@romefrontend/ast";
import {UnknownFilePath} from "@romefrontend/path";

export type ExtensionLintInfo = ExtensionHandlerMethodInfo & {
	options: WorkerLintOptions;
};

export type ExtensionLintResult = {
	sourceText: string;
	diagnostics: Diagnostics;
	formatted: string;
	suppressions: DiagnosticSuppressions;
};

export type ExtensionHandlerMethodInfo = {
	parseOptions: WorkerParseOptions;
	file: FileReference;
	project: compiler.TransformProjectDefinition;
	worker: Worker;
};

export type ExtensionParseInfo = ExtensionHandlerMethodInfo & {
	sourceTypeJS: ConstJSSourceType;
	mtime: undefined | number;
	manifestPath: undefined | string;
	path: UnknownFilePath;
};

export type PartialExtensionHandler = {
	sourceTypeJS?: ConstJSSourceType;
	isAsset?: boolean;
	canHaveScale?: boolean;
	language: DiagnosticLanguage;

	canLint: boolean;
	canFormat: boolean;

	customFormat?: (
		info: ExtensionHandlerMethodInfo,
	) => Promise<ExtensionLintResult>;

	parse: (
		opts: ExtensionParseInfo,
	) => Promise<{
		sourceText: string;
		astModifiedFromSource: boolean;
		ast: AnyRoot;
	}>;
};

export type ExtensionHandler = PartialExtensionHandler & {
	ext: string;
};

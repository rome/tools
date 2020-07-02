/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FileReference} from "@romejs/core";
import {WorkerLintOptions, WorkerParseOptions} from "../bridges/WorkerBridge";
import Worker from "../../worker/Worker";
import {DiagnosticSuppressions, Diagnostics} from "@romejs/diagnostics";
import * as compiler from "@romejs/compiler";
import {AnyRoot, ConstJSSourceType} from "@romejs/ast";
import {AnalyzeDependencyResult} from "../types/analyzeDependencies";
import fs = require("fs");
import {UnknownFilePath} from "@romejs/path";

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
	stat: fs.Stats;
	manifestPath: undefined | string;
	path: UnknownFilePath;
};

export type PartialExtensionHandler = {
	sourceTypeJS?: ConstJSSourceType;
	isAsset?: boolean;
	canHaveScale?: boolean;
	lint?: (info: ExtensionLintInfo) => Promise<ExtensionLintResult>;
	format?: (info: ExtensionHandlerMethodInfo) => Promise<ExtensionLintResult>;

	parse: (
		opts: ExtensionParseInfo,
	) => Promise<{
		sourceText: string;
		generated: boolean;
		ast: AnyRoot;
	}>;

	analyzeDependencies?: (
		opts: ExtensionHandlerMethodInfo,
	) => Promise<AnalyzeDependencyResult>;
};

export type ExtensionHandler = PartialExtensionHandler & {
	ext: string;
};

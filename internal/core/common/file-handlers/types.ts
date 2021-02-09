/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {WorkerLintResult, WorkerParseOptions, WorkerProject, FileReference} from "@internal/core";
import Worker from "../../worker/Worker";
import {
	DiagnosticIntegrity,
	DiagnosticLanguage,
	DiagnosticSuppressions,
	Diagnostics,
} from "@internal/diagnostics";
import {AnyRoot, ConstJSSourceType} from "@internal/ast";
import {UnknownPath} from "@internal/path";
import { WorkerLintTimings } from "@internal/core/worker/types";

export interface ExtensionCustomLintResult {
	mtimeNs: bigint;
	sourceText: string;
	diagnostics: Diagnostics;
	formatted: string;
	suppressions: DiagnosticSuppressions;
}

export interface ExtensionLintResult extends ExtensionCustomLintResult {
	timingsNs: WorkerLintTimings;
}

export type ExtensionHandlerMethodInfo = {
	parseOptions: WorkerParseOptions;
	mtimeNs: bigint;
	integrity: undefined | DiagnosticIntegrity;
	file: FileReference;
	project: WorkerProject;
	worker: Worker;
};

export type ExtensionParseInfo = ExtensionHandlerMethodInfo & {
	sourceTypeJS: ConstJSSourceType;
	manifestPath: undefined | string;
	path: UnknownPath;
};

export type ExtensionHandlerParseResult = {
	sourceText: string;
	astModifiedFromSource: boolean;
	ast: AnyRoot;
};

export type PartialExtensionHandler = {
	sourceTypeJS?: ConstJSSourceType;
	isAsset?: boolean;
	canHaveScale?: boolean;
	language: DiagnosticLanguage;
	hasTabs: boolean;

	capabilities: {
		lint: boolean;
		format: boolean;
	};

	customFormat?: (
		info: ExtensionHandlerMethodInfo,
	) => Promise<ExtensionCustomLintResult>;

	parse: (opts: ExtensionParseInfo) => Promise<ExtensionHandlerParseResult>;
};

export type ExtensionHandler = PartialExtensionHandler & {
	ext: string;
};

/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FileReference, WorkerParseOptions, WorkerProject} from "@internal/core";
import Worker from "../../worker/Worker";
import {
	DiagnosticIntegrity,
	DiagnosticLanguage,
	DiagnosticSuppressions,
	Diagnostics,
} from "@internal/diagnostics";
import {AnyRoot, ConstJSSourceType} from "@internal/ast";
import {Path} from "@internal/path";
import {WorkerIntegrationTimings} from "@internal/core/worker/types";

export interface ExtensionCustomLintResult {
	mtimeNs: bigint;
	sourceText: string;
	diagnostics: Diagnostics;
	formatted: string;
	suppressions: DiagnosticSuppressions;
}

export interface ExtensionLintResult extends ExtensionCustomLintResult {
	timings: WorkerIntegrationTimings;
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
	path: Path;
};

export type ExtensionHandlerParseResult<ParseRoot extends AnyRoot = AnyRoot> = {
	sourceText: string;
	astModifiedFromSource: boolean;
	ast: ParseRoot;
};

export type PartialExtensionHandler<ParseRoot extends AnyRoot = AnyRoot> = {
	sourceTypeJS?: ConstJSSourceType;
	isAsset?: boolean;
	canHaveScale?: boolean;
	language: DiagnosticLanguage;
	mime: string;
	hasTabs: boolean;

	capabilities: {
		lint: boolean;
		format: boolean;
	};

	customFormat?: (
		info: ExtensionHandlerMethodInfo,
	) => Promise<ExtensionCustomLintResult>;

	parse: (opts: ExtensionParseInfo) => Promise<ExtensionHandlerParseResult<ParseRoot>>;
};

export type ExtensionHandler<ParseRoot extends AnyRoot = AnyRoot> = PartialExtensionHandler<ParseRoot> & {
	ext: string;
};

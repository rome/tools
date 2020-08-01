/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ModuleSignature} from "@internal/js-analysis";
import {Manifest} from "@internal/codec-js-manifest";
import {AnyRoot, ConstJSSourceType} from "@internal/ast";
import {
	BundleCompileOptions,
	CompileResult,
	LintCompilerOptions,
	TransformStageName,
} from "@internal/compiler";
import {Profile} from "@internal/v8";
import {ProfilingStartData} from "./ServerBridge";
import {
	DiagnosticSuppressions,
	Diagnostics,
	DiagnosticsError,
} from "@internal/diagnostics";
import {ProjectConfigJSON} from "@internal/project";
import {Bridge} from "@internal/events";
import {JSONFileReference} from "../types/files";
import {AnalyzeDependencyResult} from "../types/analyzeDependencies";
import {InlineSnapshotUpdates} from "@internal/core/test-worker/SnapshotManager";
import {FileNotFound} from "@internal/core/common/FileNotFound";
import {createAbsoluteFilePath} from "@internal/path";
import {Number0} from "@internal/ob1";
import {FormatterOptions} from "@internal/formatter";
import {RecoverySaveFile} from "@internal/core/server/fs/RecoveryStore";

export type WorkerProjects = Array<{
	id: number;
	directory: string;
	config: undefined | ProjectConfigJSON;
}>;

export type WorkerCompileResult = CompileResult & {
	cached: boolean;
};

export type WorkerPartialManifest = {
	path: string;
	type: Manifest["type"];
};

export type WorkerPartialManifests = Array<{
	id: number;
	manifest: undefined | WorkerPartialManifest;
}>;

// Omit analyze value as the worker will fetch it itself, skips sending over a large payload that it already has in memory
export type WorkerCompilerOptions = {
	bundle?: WorkerBundleCompileOptions;
};

export type WorkerBundleCompileOptions = Omit<BundleCompileOptions, "analyze">;

//
export type WorkerAnalyzeDependencyResult = AnalyzeDependencyResult & {
	cached: boolean;
};

export type WorkerLintOptions = {
	compilerOptions?: LintCompilerOptions;
	prefetchedModuleSignatures: PrefetchedModuleSignatures;
	applyRecommendedFixes: boolean;
	save: boolean;
};

export type WorkerParseOptions = {
	sourceTypeJS?: ConstJSSourceType;
	cache?: boolean;
	allowParserDiagnostics?: boolean;
	allowCorrupt?: boolean;
};

export type WorkerStatus = {
	astCacheSize: number;
	memoryUsage: {
		rss: number;
		heapTotal: number;
		heapUsed: number;
		external: number;
	};
	pid: number;
	uptime: number;
};

export type PrefetchedModuleSignatures = {
	[key: string]:
		| {
				type: "USE_CACHED";
				filename: string;
			}
		| {
				type: "RESOLVED";
				graph: ModuleSignature;
			}
		| {
				type: "OWNED";
				ref: JSONFileReference;
			}
		| {
				type: "POINTER";
				key: string;
			};
};

export type WorkerFormatResult = {
	original: string;
	formatted: string;
	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
};

export type WorkerLintResult = {
	save: undefined | RecoverySaveFile;
	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
};

export type WorkerBufferPosition = {
	line: Number0;
	character: Number0;
};

export type WorkerBufferPatch = {
	range: {
		start: WorkerBufferPosition;
		end: WorkerBufferPosition;
	};
	text: string;
};

export type WorkerUpdateInlineSnapshotResult = {
	diagnostics: Diagnostics;
	file: undefined | RecoverySaveFile;
};

export default class WorkerBridge extends Bridge {
	log = this.createEvent<string, void>({
		name: "log",
		direction: "server<-client",
	});

	updateProjects = this.createEvent<
		{
			projects: WorkerProjects;
		},
		void
	>({
		name: "updateProjects",
		direction: "server->client",
	});

	updateManifests = this.createEvent<
		{
			manifests: WorkerPartialManifests;
		},
		void
	>({
		name: "updateManifests",
		direction: "server->client",
	});

	profilingStart = this.createEvent<ProfilingStartData, void>({
		name: "profiling.start",
		direction: "server->client",
	});

	profilingStop = this.createEvent<void, Profile>({
		name: "profiling.stop",
		direction: "server->client",
	});

	status = this.createEvent<void, WorkerStatus>({
		name: "status",
		direction: "server->client",
	});

	evict = this.createEvent<
		{
			filename: string;
		},
		void
	>({
		name: "evict",
		direction: "server->client",
	});

	format = this.createEvent<
		{
			ref: JSONFileReference;
			options: FormatterOptions;
			parseOptions: WorkerParseOptions;
		},
		undefined | WorkerFormatResult
	>({
		name: "format",
		direction: "server->client",
	});

	moduleSignatureJS = this.createEvent<
		{
			ref: JSONFileReference;
			parseOptions: WorkerParseOptions;
		},
		ModuleSignature
	>({
		name: "moduleSignatureJS",
		direction: "server->client",
	});

	analyzeDependencies = this.createEvent<
		{
			ref: JSONFileReference;
			parseOptions: WorkerParseOptions;
		},
		AnalyzeDependencyResult
	>({
		name: "analyzeDependencies",
		direction: "server->client",
	});

	lint = this.createEvent<
		{
			ref: JSONFileReference;
			options: WorkerLintOptions;
			parseOptions: WorkerParseOptions;
		},
		WorkerLintResult
	>({name: "lint", direction: "server->client"});

	updateInlineSnapshots = this.createEvent<
		{
			ref: JSONFileReference;
			updates: InlineSnapshotUpdates;
			parseOptions: WorkerParseOptions;
		},
		WorkerUpdateInlineSnapshotResult
	>({name: "updateInlineSnapshots", direction: "server->client"});

	compile = this.createEvent<
		{
			ref: JSONFileReference;
			stage: TransformStageName;
			options: WorkerCompilerOptions;
			parseOptions: WorkerParseOptions;
		},
		CompileResult
	>({name: "compile", direction: "server->client"});

	parse = this.createEvent<
		{
			ref: JSONFileReference;
			options: WorkerParseOptions;
		},
		// @ts-ignore
		AnyRoot
	>({name: "parse", direction: "server->client"});

	getFileBuffers = this.createEvent<
		void,
		Array<{
			filename: string;
			content: string;
		}>
	>({
		name: "getFileBuffers",
		direction: "server->client",
	});

	updateBuffer = this.createEvent<
		{
			ref: JSONFileReference;
			content: string;
		},
		void
	>({
		name: "updateBuffer",
		direction: "server->client",
	});

	patchBuffer = this.createEvent<
		{
			ref: JSONFileReference;
			patches: Array<WorkerBufferPatch>;
		},
		string
	>({
		name: "patchBuffer",
		direction: "server->client",
	});

	clearBuffer = this.createEvent<
		{
			ref: JSONFileReference;
		},
		void
	>({
		name: "clearBuffer",
		direction: "server->client",
	});

	init() {
		this.addErrorTransport(
			"FileNotFound",
			{
				serialize(err: Error) {
					if (!(err instanceof FileNotFound)) {
						throw new Error("Expected FileNotFound");
					}

					return {
						path: err.path.join(),
					};
				},
				hydrate(err, data) {
					// rome-ignore lint/ts/noExplicitAny
					return new FileNotFound(
						createAbsoluteFilePath((data.path as any)),
						err.message,
					);
				},
			},
		);

		this.addErrorTransport(
			"DiagnosticsError",
			{
				serialize(err: Error) {
					if (!(err instanceof DiagnosticsError)) {
						throw new Error("Expected DiagnosticsError");
					}

					return {
						diagnostics: err.diagnostics,
					};
				},
				hydrate(err, data) {
					return new DiagnosticsError(
						String(err.message),
						// rome-ignore lint/ts/noExplicitAny
						(data.diagnostics as any),
					);
				},
			},
		);
	}
}

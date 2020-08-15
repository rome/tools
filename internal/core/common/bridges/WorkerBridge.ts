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
import {Bridge, BridgeErrorResponseDetails} from "@internal/events";
import {FileReference} from "../types/files";
import {AnalyzeDependencyResult} from "@internal/core";
import {InlineSnapshotUpdates} from "@internal/core/test-worker/SnapshotManager";
import {FileNotFound} from "@internal/fs/FileNotFound";
import {AbsoluteFilePath, createAbsoluteFilePath} from "@internal/path";
import {Number0} from "@internal/ob1";
import {FormatterOptions} from "@internal/formatter";
import {RecoverySaveFile} from "@internal/core/server/fs/RecoveryStore";
import {ProjectConfig} from "@internal/project";

export type WorkerProjects = Array<{
	id: number;
	directory: AbsoluteFilePath;
	config: undefined | ProjectConfig;
}>;

export type WorkerCompileResult = CompileResult & {
	cached: boolean;
};

export type WorkerPartialManifest = {
	path: AbsoluteFilePath;
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
	applySafeFixes: boolean;
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
				ref: FileReference;
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
	public log = this.createEvent<string, void>({
		name: "log",
		direction: "server<-client",
	});

	public fatalError = this.createEvent<BridgeErrorResponseDetails, void>({
		name: "fatalError",
		direction: "server<-client",
	});

	public updateProjects = this.createEvent<
		{
			projects: WorkerProjects;
		},
		void
	>({
		name: "updateProjects",
		direction: "server->client",
	});

	public updateManifests = this.createEvent<
		{
			manifests: WorkerPartialManifests;
		},
		void
	>({
		name: "updateManifests",
		direction: "server->client",
	});

	public profilingStart = this.createEvent<ProfilingStartData, void>({
		name: "profiling.start",
		direction: "server->client",
	});

	public profilingStop = this.createEvent<void, Profile>({
		name: "profiling.stop",
		direction: "server->client",
	});

	public status = this.createEvent<void, WorkerStatus>({
		name: "status",
		direction: "server->client",
	});

	public evict = this.createEvent<
		{
			filename: string;
		},
		void
	>({
		name: "evict",
		direction: "server->client",
	});

	public format = this.createEvent<
		{
			ref: FileReference;
			options: FormatterOptions;
			parseOptions: WorkerParseOptions;
		},
		undefined | WorkerFormatResult
	>({
		name: "format",
		direction: "server->client",
	});

	public moduleSignatureJS = this.createEvent<
		{
			ref: FileReference;
			parseOptions: WorkerParseOptions;
		},
		ModuleSignature
	>({
		name: "moduleSignatureJS",
		direction: "server->client",
	});

	public analyzeDependencies = this.createEvent<
		{
			ref: FileReference;
			parseOptions: WorkerParseOptions;
		},
		AnalyzeDependencyResult
	>({
		name: "analyzeDependencies",
		direction: "server->client",
	});

	public lint = this.createEvent<
		{
			ref: FileReference;
			options: WorkerLintOptions;
			parseOptions: WorkerParseOptions;
		},
		WorkerLintResult
	>({name: "lint", direction: "server->client"});

	public updateInlineSnapshots = this.createEvent<
		{
			ref: FileReference;
			updates: InlineSnapshotUpdates;
			parseOptions: WorkerParseOptions;
		},
		WorkerUpdateInlineSnapshotResult
	>({name: "updateInlineSnapshots", direction: "server->client"});

	public compile = this.createEvent<
		{
			ref: FileReference;
			stage: TransformStageName;
			options: WorkerCompilerOptions;
			parseOptions: WorkerParseOptions;
		},
		CompileResult
	>({name: "compile", direction: "server->client"});

	public parse = this.createEvent<
		{
			ref: FileReference;
			options: WorkerParseOptions;
		},
		// @ts-ignore
		AnyRoot
	>({name: "parse", direction: "server->client"});

	public getFileBuffers = this.createEvent<
		void,
		Array<{
			filename: string;
			content: string;
		}>
	>({
		name: "getFileBuffers",
		direction: "server->client",
	});

	public getBuffer = this.createEvent<
		{
			ref: FileReference;
		},
		string | undefined
	>({
		name: "getBuffer",
		direction: "server->client",
	});

	public updateBuffer = this.createEvent<
		{
			ref: FileReference;
			content: string;
		},
		void
	>({
		name: "updateBuffer",
		direction: "server->client",
	});

	public patchBuffer = this.createEvent<
		{
			ref: FileReference;
			patches: Array<WorkerBufferPatch>;
		},
		string
	>({
		name: "patchBuffer",
		direction: "server->client",
	});

	public clearBuffer = this.createEvent<
		{
			ref: FileReference;
		},
		void
	>({
		name: "clearBuffer",
		direction: "server->client",
	});

	public init() {
		this.addErrorTransport(
			"FileNotFound",
			{
				serialize(err: Error) {
					if (!(err instanceof FileNotFound)) {
						throw new Error("Expected FileNotFound");
					}

					return {
						suffixMessage: err.suffixMessage,
						path: err._path.join(),
					};
				},
				hydrate(err, data) {
					return new FileNotFound(
						createAbsoluteFilePath(data.path),
						data.suffixMessage,
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
					return new DiagnosticsError(err.message, data.diagnostics);
				},
			},
		);
	}
}

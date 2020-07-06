/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ModuleSignature} from "@romejs/js-analysis";
import {Manifest} from "@romejs/codec-js-manifest";
import {AnyRoot, ConstJSProgramSyntax, ConstJSSourceType} from "@romejs/ast";
import {
	BundleCompileOptions,
	CompileResult,
	LintCompilerOptions,
	TransformStageName,
} from "@romejs/compiler";
import {Profile} from "@romejs/v8";
import {ProfilingStartData} from "./ServerBridge";
import {
	DiagnosticSuppressions,
	Diagnostics,
	DiagnosticsError,
} from "@romejs/diagnostics";
import {ProjectConfigJSON} from "@romejs/project";
import {Bridge} from "@romejs/events";
import {JSONFileReference} from "../types/files";
import {AnalyzeDependencyResult} from "../types/analyzeDependencies";
import {InlineSnapshotUpdates} from "@romejs/core/test-worker/SnapshotManager";
import {FileNotFound} from "@romejs/core/common/FileNotFound";
import {createAbsoluteFilePath} from "@romejs/path";

export type WorkerProjects = Array<{
	id: number;
	folder: string;
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
	sourceType?: ConstJSSourceType;
	syntax?: Array<ConstJSProgramSyntax>;
	cache?: boolean;
	allowParserDiagnostics?: boolean;
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
				file: JSONFileReference;
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
};

export type WorkerLintResult = {
	save: undefined | string;
	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
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
			file: JSONFileReference;
			parseOptions: WorkerParseOptions;
		},
		undefined | WorkerFormatResult
	>({
		name: "format",
		direction: "server->client",
	});

	moduleSignatureJS = this.createEvent<
		{
			file: JSONFileReference;
			parseOptions: WorkerParseOptions;
		},
		ModuleSignature
	>({
		name: "moduleSignatureJS",
		direction: "server->client",
	});

	analyzeDependencies = this.createEvent<
		{
			file: JSONFileReference;
			parseOptions: WorkerParseOptions;
		},
		AnalyzeDependencyResult
	>({
		name: "analyzeDependencies",
		direction: "server->client",
	});

	lint = this.createEvent<
		{
			file: JSONFileReference;
			options: WorkerLintOptions;
			parseOptions: WorkerParseOptions;
		},
		WorkerLintResult
	>({name: "lint", direction: "server->client"});

	updateInlineSnapshots = this.createEvent<
		{
			file: JSONFileReference;
			updates: InlineSnapshotUpdates;
			parseOptions: WorkerParseOptions;
		},
		Diagnostics
	>({name: "updateInlineSnapshots", direction: "server->client"});

	compile = this.createEvent<
		{
			file: JSONFileReference;
			stage: TransformStageName;
			options: WorkerCompilerOptions;
			parseOptions: WorkerParseOptions;
		},
		CompileResult
	>({name: "compile", direction: "server->client"});

	parse = this.createEvent<
		{
			file: JSONFileReference;
			options: WorkerParseOptions;
		},
		AnyRoot
	>({name: "parse", direction: "server->client"});

	getFileBuffers = this.createEvent<
		void,
		Array<{
			file: string;
			content: string;
		}>
	>({
		name: "getFileBuffers",
		direction: "server->client",
	});

	updateBuffer = this.createEvent<
		{
			file: JSONFileReference;
			content: string;
		},
		void
	>({
		name: "updateBuffer",
		direction: "server->client",
	});

	clearBuffer = this.createEvent<
		{
			file: JSONFileReference;
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
					// rome-ignore lint/js/noExplicitAny
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
						// rome-ignore lint/js/noExplicitAny
						(data.diagnostics as any),
					);
				},
			},
		);
	}
}

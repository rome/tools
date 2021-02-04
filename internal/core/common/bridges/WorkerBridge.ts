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
	DiagnosticIntegrity,
	DiagnosticSuppressions,
	Diagnostics,
	DiagnosticsError,
} from "@internal/diagnostics";
import {BridgeErrorResponseDetails, createBridge} from "@internal/events";
import {FileReference} from "../types/files";
import {AnalyzeDependencyResult} from "@internal/core";
import {InlineSnapshotUpdates} from "@internal/core/test-worker/SnapshotManager";
import {AbsoluteFilePath, createAbsoluteFilePath} from "@internal/path";
import {Number0} from "@internal/ob1";
import {FormatterOptions} from "@internal/formatter";
import {RecoverySaveFile} from "@internal/core/server/fs/RecoveryStore";
import {ProjectConfig} from "@internal/project";
import {WorkerBuffer} from "@internal/core/worker/Worker";
import {createBridgeEventDeclaration} from "@internal/events/createBridge";
import {FileNotFound} from "@internal/fs";

export type WorkerProjects = {
	id: number;
	directory: AbsoluteFilePath;
	configHashes: string[];
	config: undefined | ProjectConfig;
}[];

export type WorkerPartialManifest = {
	path: AbsoluteFilePath;
	hash: string;
	type: Manifest["type"];
};

export type WorkerPartialManifests = {
	id: number;
	manifest: undefined | WorkerPartialManifest;
}[];

// Omit analyze value as the worker will fetch it itself, skips sending over a large payload that it already has in memory
export type WorkerCompilerOptions = {
	bundle?: WorkerBundleCompileOptions;
};

export type WorkerBundleCompileOptions = Omit<BundleCompileOptions, "analyze">;

export type CachedWrapper<T> = {
	value: T;
	integrity: undefined | DiagnosticIntegrity;
	cached: boolean;
};

export type WorkerAnalyzeDependencyResult = CachedWrapper<AnalyzeDependencyResult>;

export type WorkerCompileResult = CachedWrapper<CompileResult>;

export type WorkerLintOptions = {
	compilerOptions?: LintCompilerOptions;
	prefetchedModuleSignatures: PrefetchedModuleSignatures;
	applySafeFixes: boolean;
	suppressionExplanation?: string;
	save: boolean;
};

export type WorkerFormatOptions = Omit<FormatterOptions, "projectConfig"> & {
	forceFormat?: boolean;
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

export default createBridge({
	debugName: "worker",

	shared: {},

	server: {
		log: createBridgeEventDeclaration<string, void>(),
		fatalError: createBridgeEventDeclaration<BridgeErrorResponseDetails, void>(),
	},

	client: {
		updateProjects: createBridgeEventDeclaration<
			{
				projects: WorkerProjects;
			},
			void
		>(),

		updateManifests: createBridgeEventDeclaration<
			{
				manifests: WorkerPartialManifests;
			},
			void
		>(),

		profilingStart: createBridgeEventDeclaration<ProfilingStartData, void>(),

		profilingStop: createBridgeEventDeclaration<void, Profile>(),

		status: createBridgeEventDeclaration<void, WorkerStatus>(),

		evict: createBridgeEventDeclaration<
			{
				real: AbsoluteFilePath;
				uid: string;
			},
			void
		>(),

		format: createBridgeEventDeclaration<
			{
				ref: FileReference;
				options: WorkerFormatOptions;
				parseOptions: WorkerParseOptions;
			},
			undefined | WorkerFormatResult
		>(),

		moduleSignatureJS: createBridgeEventDeclaration<
			{
				ref: FileReference;
				parseOptions: WorkerParseOptions;
			},
			ModuleSignature
		>(),

		analyzeDependencies: createBridgeEventDeclaration<
			{
				ref: FileReference;
				parseOptions: WorkerParseOptions;
			},
			WorkerAnalyzeDependencyResult
		>(),

		lint: createBridgeEventDeclaration<
			{
				ref: FileReference;
				options: WorkerLintOptions;
				parseOptions: WorkerParseOptions;
			},
			WorkerLintResult
		>(),

		updateInlineSnapshots: createBridgeEventDeclaration<
			{
				ref: FileReference;
				updates: InlineSnapshotUpdates;
				parseOptions: WorkerParseOptions;
			},
			WorkerUpdateInlineSnapshotResult
		>(),

		compile: createBridgeEventDeclaration<
			{
				ref: FileReference;
				stage: TransformStageName;
				options: WorkerCompilerOptions;
				parseOptions: WorkerParseOptions;
			},
			WorkerCompileResult
		>(),

		parse: createBridgeEventDeclaration<
			{
				ref: FileReference;
				options: WorkerParseOptions;
			},
			// @ts-ignore: AST is a bunch of interfaces which we cannot match with an object index
			AnyRoot
		>(),

		getFileBuffers: createBridgeEventDeclaration<
			void,
			[AbsoluteFilePath, WorkerBuffer][]
		>(),

		getBuffer: createBridgeEventDeclaration<
			{
				ref: FileReference;
			},
			string | undefined
		>(),

		updateBuffer: createBridgeEventDeclaration<
			{
				ref: FileReference;
				buffer: WorkerBuffer;
			},
			void
		>(),

		patchBuffer: createBridgeEventDeclaration<
			{
				ref: FileReference;
				patches: WorkerBufferPatch[];
			},
			string
		>(),

		clearBuffer: createBridgeEventDeclaration<
			{
				ref: FileReference;
			},
			void
		>(),
	},

	init(bridge) {
		bridge.addErrorTransport(
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

		bridge.addErrorTransport(
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
	},
});

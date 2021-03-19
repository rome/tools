import {TokenBase} from "@internal/parser-core";
/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ModuleSignature} from "@internal/js-analysis";
import {AnyRoot} from "@internal/ast";
import {TransformStageName} from "@internal/compiler";
import {Profile} from "@internal/v8";
import {ProfilingStartData, ServerBridgeLog} from "./ServerBridge";
import {Diagnostic, DiagnosticsError} from "@internal/diagnostics";
import {BridgeErrorDetails, createBridge} from "@internal/events";
import {FileReference} from "../types/files";
import {
	InlineSnapshotUpdate,
	SnapshotEntry,
} from "@internal/core/worker/test/SnapshotManager";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	UIDPath,
	createAbsoluteFilePath,
} from "@internal/path";
import {createBridgeEventDeclaration} from "@internal/events/createBridge";
import {FileNotFound} from "@internal/fs";
import {
	TestRef,
	TestWorkerPrepareTestOptions,
	TestWorkerPrepareTestResult,
	TestWorkerRunTestOptions,
	WorkerAnalyzeDependencyResult,
	WorkerBuffer,
	WorkerBufferPatch,
	WorkerCompileResult,
	WorkerCompilerOptions,
	WorkerFormatOptions,
	WorkerFormatResult,
	WorkerLintOptions,
	WorkerLintResult,
	WorkerParseOptions,
	WorkerProjects,
	WorkerStatus,
	WorkerUpdateInlineSnapshotResult,
} from "@internal/core";
import {WorkerPartialManifest} from "@internal/core/worker/types";
import {TestConsoleAdvice} from "@internal/core/worker/test/TestWorkerFile";

export default createBridge({
	debugName: "Worker",

	shared: {},

	server: {
		log: createBridgeEventDeclaration<Omit<ServerBridgeLog, "origin">, void>(),

		fatalError: createBridgeEventDeclaration<BridgeErrorDetails, void>(),

		testInlineSnapshotUpdate: createBridgeEventDeclaration<
			{
				testPath: AbsoluteFilePath;
				update: InlineSnapshotUpdate;
			},
			void
		>(),

		testSnapshotEntry: createBridgeEventDeclaration<
			{
				testPath: AbsoluteFilePath;
				snapshotPath: AbsoluteFilePath;
				entry: SnapshotEntry;
			},
			void
		>(),

		testDiskSnapshotDiscovered: createBridgeEventDeclaration<
			{
				testPath: AbsoluteFilePath;
				snapshotPath: AbsoluteFilePath;
			},
			void
		>(),

		testStart: createBridgeEventDeclaration<
			{
				ref: TestRef;
				timeout: undefined | number;
			},
			void
		>(),

		testDiagnostic: createBridgeEventDeclaration<
			{
				testPath: undefined | AbsoluteFilePath;
				diagnostic: Diagnostic;
			},
			void
		>(),

		testFinish: createBridgeEventDeclaration<
			{
				success: boolean;
				ref: TestRef;
			},
			void
		>(),
	},

	client: {
		setLogs: createBridgeEventDeclaration<boolean, void>(),

		evictProject: createBridgeEventDeclaration<number, void>(),

		updateProjects: createBridgeEventDeclaration<WorkerProjects, void>(),

		updateManifests: createBridgeEventDeclaration<
			{
				manifests: Map<number, undefined | WorkerPartialManifest>;
			},
			void
		>(),

		profilingStart: createBridgeEventDeclaration<ProfilingStartData, void>(),

		profilingStop: createBridgeEventDeclaration<void, Profile>(),

		status: createBridgeEventDeclaration<void, WorkerStatus>(),

		evict: createBridgeEventDeclaration<
			{
				real: AbsoluteFilePath;
				uid: UIDPath;
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
				updates: InlineSnapshotUpdate[];
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

		tokenize: createBridgeEventDeclaration<
			{
				ref: FileReference;
				options: WorkerParseOptions;
			},
			TokenBase[]
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

		inspectorDetails: createBridgeEventDeclaration<
			void,
			{
				inspectorUrl: undefined | string;
			}
		>(),

		executeScript: createBridgeEventDeclaration<
			{
				contextDirectory: AbsoluteFilePath;
				cwd: AbsoluteFilePath;
				args: string[];
				path: AbsoluteFilePath;
				code: string;
			},
			{
				exitCode: undefined | number;
			}
		>(),

		testReceiveCompiledDependency: createBridgeEventDeclaration<
			AbsoluteFilePathMap<string>,
			void
		>(),

		testPrepare: createBridgeEventDeclaration<
			TestWorkerPrepareTestOptions,
			TestWorkerPrepareTestResult
		>(),

		testRun: createBridgeEventDeclaration<TestWorkerRunTestOptions, void>(),

		testGetConsoleAdvice: createBridgeEventDeclaration<
			AbsoluteFilePath,
			TestConsoleAdvice
		>(),

		testGetRawSnapshot: createBridgeEventDeclaration<
			{
				path: AbsoluteFilePath;
				snapshotPath: AbsoluteFilePath;
			},
			string
		>(),
	},

	init(bridge) {
		bridge.addCustomErrorTransport<{
			suffixMessage: undefined | string;
			path: string;
		}>(
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

		bridge.addCustomErrorTransport<{
			diagnostics: Diagnostic[];
		}>(
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

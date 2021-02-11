/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ModuleSignature} from "@internal/js-analysis";
import {AnyRoot} from "@internal/ast";
import {
	TransformStageName,
} from "@internal/compiler";
import {Profile} from "@internal/v8";
import {ProfilingStartData, ServerBridgeLog} from "./ServerBridge";
import {
	DiagnosticsError,
} from "@internal/diagnostics";
import {BridgeErrorResponseDetails, createBridge} from "@internal/events";
import {FileReference} from "../types/files";
import {InlineSnapshotUpdates} from "@internal/core/test-worker/SnapshotManager";
import {AbsoluteFilePath, createAbsoluteFilePath, UIDPath} from "@internal/path";
import {createBridgeEventDeclaration} from "@internal/events/createBridge";
import {FileNotFound} from "@internal/fs";
import {
	WorkerBuffer,
	WorkerProjects,
	WorkerPartialManifests,
	WorkerCompilerOptions,
	WorkerAnalyzeDependencyResult,
	WorkerCompileResult,
	WorkerLintOptions,
	WorkerFormatOptions,
	WorkerParseOptions,
	WorkerStatus,
	WorkerFormatResult,
	WorkerLintResult,
	WorkerBufferPatch,
	WorkerUpdateInlineSnapshotResult,
} from "@internal/core";

export default createBridge({
	debugName: "worker",

	shared: {},

	server: {
		log: createBridgeEventDeclaration<Omit<ServerBridgeLog, "origin">, void>(),
		fatalError: createBridgeEventDeclaration<BridgeErrorResponseDetails, void>(),
	},

	client: {
		setLogs: createBridgeEventDeclaration<boolean, void>(),

		evictProject: createBridgeEventDeclaration<number, void>(),

		updateProjects: createBridgeEventDeclaration<WorkerProjects, void>(),

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

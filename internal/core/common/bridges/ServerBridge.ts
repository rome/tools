/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Profile} from "@internal/v8";
import {Diagnostics} from "@internal/diagnostics";
import {
	ClientFlags,
	ClientLogsLevel,
	ClientRequestFlags,
} from "../types/client";
import {ReporterStream, ReporterStreamState} from "@internal/cli-reporter";
import {ServerMarker} from "../../server/Server";
import {TerminalFeatures} from "@internal/cli-environment";
import {Dict} from "@internal/typescript-helpers";
import {RecoverySaveFile} from "@internal/core/server/fs/RecoveryStore";
import {RSERObject, RSERValue} from "@internal/codec-binary-serial";
import createBridge, {createBridgeEventDeclaration} from "@internal/events/createBridge";
import {CommandName} from "../commands";

export type ServerQueryRequest = {
	requestFlags: ClientRequestFlags;
	commandFlags: RSERObject;
	args: string[];
	commandName: CommandName;
	silent: boolean;
	noData: boolean;
	noFileWrites: boolean;
	cancelToken?: string;
};

export type PartialServerQueryRequest = Partial<Omit<
	ServerQueryRequest,
	"requestFlags" | "commandName"
>> & {
	requestFlags?: Partial<ClientRequestFlags>;
	commandName: CommandName;
};

export type ServerQueryResponseBase = {
	markers: ServerMarker[];
};

export type ServerQueryResponseSuccess = ServerQueryResponseBase & {
	type: "SUCCESS";
	hasData: boolean;
	data: RSERValue;
	files: Dict<RecoverySaveFile>;
};

export type ServerQueryResponseDiagnostics = ServerQueryResponseBase & {
	type: "DIAGNOSTICS";
	hasDiagnostics: boolean;
	diagnostics: Diagnostics;
	files: Dict<RecoverySaveFile>;
};

export type ServerQueryResponseInvalid = ServerQueryResponseBase & {
	type: "INVALID_REQUEST";
	diagnostics: Diagnostics;
	showHelp: boolean;
};

export type ServerQueryResponseCancelled = ServerQueryResponseBase & {
	type: "CANCELLED";
	reason: string;
};

export type ServerQueryResponseExit = ServerQueryResponseBase & {
	type: "EXIT";
	code: number;
};

export type ServerQueryResponse =
	| ServerQueryResponseInvalid
	| ServerQueryResponseSuccess
	| ServerQueryResponseCancelled
	| ServerQueryResponseDiagnostics
	| ServerQueryResponseExit;

export type ProfilingStartData = {
	samplingInterval: number;
};

export type ServerBridgeInfo = {
	version: string;
	streamState: Omit<ReporterStreamState, "lineSnapshots"> & {
		lineSnapshots: undefined;
	};
	outputSupport: TerminalFeatures;
	outputFormat: ReporterStream["format"];
	flags: ClientFlags;
};

export type ServerBridgeLog = {
	isError: boolean;
	origin: "server" | "worker";
	chunk: string;
};

export default createBridge({
	debugName: "Server",

	shared: {},

	client: {
		getClientInfo: createBridgeEventDeclaration<void, ServerBridgeInfo>(),
		serverReady: createBridgeEventDeclaration<void, void>(),
		write: createBridgeEventDeclaration<[string, boolean], void>(),
		log: createBridgeEventDeclaration<ServerBridgeLog, void>(),
		lspFromServerBuffer: createBridgeEventDeclaration<string, void>(),
	},

	server: {
		setLogLevel: createBridgeEventDeclaration<
			{
				level: undefined | ClientLogsLevel;
				includeWorker: boolean;
			},
			void
		>(),
		endServer: createBridgeEventDeclaration<void, void>(),
		updateFeatures: createBridgeEventDeclaration<TerminalFeatures, void>(),
		query: createBridgeEventDeclaration<
			PartialServerQueryRequest,
			ServerQueryResponse
		>(),
		cancelQuery: createBridgeEventDeclaration<string, void>(),
		profilingGetWorkers: createBridgeEventDeclaration<void, number[]>(),
		profilingStart: createBridgeEventDeclaration<ProfilingStartData, void>(),
		profilingStop: createBridgeEventDeclaration<void, Profile>(),
		profilingStopWorker: createBridgeEventDeclaration<number, Profile>(),
		lspFromClientBuffer: createBridgeEventDeclaration<string, void>(),
	},
});

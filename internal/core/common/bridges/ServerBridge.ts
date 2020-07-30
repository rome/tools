/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Profile} from "@internal/v8";
import {Diagnostics} from "@internal/diagnostics";
import {ClientFlagsJSON, ClientRequestFlags} from "../types/client";
import {Bridge} from "@internal/events";
import {JSONObject, JSONPropertyValue} from "@internal/codec-json";
import {ReporterStream, ReporterStreamState} from "@internal/cli-reporter";
import {ServerMarker} from "../../server/Server";
import {TerminalFeatures} from "@internal/cli-environment";
import {Dict} from "@internal/typescript-helpers";
import {RecoverySaveFile} from "@internal/core/server/fs/RecoveryStore";

export type ServerQueryRequest = {
	requestFlags: ClientRequestFlags;
	commandFlags: JSONObject;
	args: Array<string>;
	commandName: string;
	silent: boolean;
	noData: boolean;
	noFileWrites: boolean;
	terminateWhenIdle: boolean;
	cancelToken?: string;
};

export type PartialServerQueryRequest = Partial<Omit<
	ServerQueryRequest,
	"requestFlags" | "commandName"
>> & {
	requestFlags?: Partial<ClientRequestFlags>;
	commandName: string;
};

type ServerQueryResponseBase = {
	markers: Array<ServerMarker>;
};

export type ServerQueryResponseSuccess = ServerQueryResponseBase & {
	type: "SUCCESS";
	hasData: boolean;
	data: JSONPropertyValue;
	files: Dict<RecoverySaveFile>;
};

export type ServerQueryResponseError = ServerQueryResponseBase & {
	type: "ERROR";
	fatal: boolean;
	handled: boolean;
	name: string;
	message: string;
	stack: undefined | string;
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
};

export type ServerQueryResponseExit = ServerQueryResponseBase & {
	type: "EXIT";
	code: number;
};

export type ServerQueryResponse =
	| ServerQueryResponseInvalid
	| ServerQueryResponseSuccess
	| ServerQueryResponseError
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
	flags: ClientFlagsJSON;
};

export default class ServerBridge extends Bridge {
	getClientInfo = this.createEvent<void, ServerBridgeInfo>({
		name: "getClientInfo",
		direction: "server->client",
	});

	write = this.createEvent<[string, boolean], void>({
		name: "write",
		direction: "server->client",
	});

	enableWorkerLogs = this.createEvent<void, void>({
		name: "enableWorkerLogs",
		direction: "server<-client",
	});

	log = this.createEvent<
		{
			origin: "server" | "worker";
			chunk: string;
		},
		void
	>({
		name: "log",
		direction: "server->client",
	});

	updateFeatures = this.createEvent<TerminalFeatures, void>({
		name: "updateFeatures",
		direction: "server<-client",
	});

	query = this.createEvent<PartialServerQueryRequest, ServerQueryResponse>({
		name: "query",
		direction: "server<-client",
	});

	cancelQuery = this.createEvent<string, void>({
		name: "cancel",
		direction: "server<-client",
	});

	profilingGetWorkers = this.createEvent<void, Array<number>>({
		name: "profiling.getWorkers",
		direction: "server<-client",
	});

	profilingStart = this.createEvent<ProfilingStartData, void>({
		name: "profiling.start",
		direction: "server<-client",
	});

	profilingStop = this.createEvent<void, Profile>({
		name: "profiling.stop",
		direction: "server<-client",
	});

	profilingStopWorker = this.createEvent<number, Profile>({
		name: "profile.stopWorker",
		direction: "server<-client",
	});

	lspFromClientBuffer = this.createEvent<string, void>({
		name: "lspFromClientBuffer",
		direction: "server<-client",
	});

	lspFromServerBuffer = this.createEvent<string, void>({
		name: "lspFromServerBuffer",
		direction: "server->client",
	});

	endServer = this.createEvent<void, void>({
		name: "endServer",
		direction: "server<-client",
	});
}

/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Profile} from "@internal/v8";
import {Diagnostics} from "@internal/diagnostics";
import {ClientFlags, ClientRequestFlags} from "../types/client";
import {Bridge} from "@internal/events";
import {JSONPropertyValue} from "@internal/codec-json";
import {ReporterStream, ReporterStreamState} from "@internal/cli-reporter";
import {ServerMarker} from "../../server/Server";
import {TerminalFeatures} from "@internal/cli-environment";
import {Dict} from "@internal/typescript-helpers";
import {RecoverySaveFile} from "@internal/core/server/fs/RecoveryStore";
import {RSERObject} from "@internal/codec-binary-serial";

export type ServerQueryRequest = {
	requestFlags: ClientRequestFlags;
	commandFlags: RSERObject;
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

export default class ServerBridge extends Bridge {
	public getClientInfo = this.createEvent<void, ServerBridgeInfo>({
		name: "getClientInfo",
		direction: "server->client",
	});

	public serverReady = this.createEvent<void, void>({
		name: "serverReady",
		direction: "server->client",
	});

	public write = this.createEvent<[string, boolean], void>({
		name: "write",
		direction: "server->client",
	});

	public enableWorkerLogs = this.createEvent<void, void>({
		name: "enableWorkerLogs",
		direction: "server<-client",
	});

	public log = this.createEvent<
		{
			origin: "server" | "worker";
			chunk: string;
		},
		void
	>({
		name: "log",
		direction: "server->client",
	});

	public updateFeatures = this.createEvent<TerminalFeatures, void>({
		name: "updateFeatures",
		direction: "server<-client",
	});

	public query = this.createEvent<
		PartialServerQueryRequest,
		ServerQueryResponse
	>({
		name: "query",
		direction: "server<-client",
	});

	public cancelQuery = this.createEvent<string, void>({
		name: "cancel",
		direction: "server<-client",
	});

	public profilingGetWorkers = this.createEvent<void, Array<number>>({
		name: "profiling.getWorkers",
		direction: "server<-client",
	});

	public profilingStart = this.createEvent<ProfilingStartData, void>({
		name: "profiling.start",
		direction: "server<-client",
	});

	public profilingStop = this.createEvent<void, Profile>({
		name: "profiling.stop",
		direction: "server<-client",
	});

	public profilingStopWorker = this.createEvent<number, Profile>({
		name: "profile.stopWorker",
		direction: "server<-client",
	});

	public lspFromClientBuffer = this.createEvent<string, void>({
		name: "lspFromClientBuffer",
		direction: "server<-client",
	});

	public lspFromServerBuffer = this.createEvent<string, void>({
		name: "lspFromServerBuffer",
		direction: "server->client",
	});

	public endServer = this.createEvent<void, void>({
		name: "endServer",
		direction: "server<-client",
	});
}

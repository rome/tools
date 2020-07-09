/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Profile} from "@romefrontend/v8";
import {Diagnostics} from "@romefrontend/diagnostics";
import {ClientFlagsJSON, ClientRequestFlags} from "../types/client";
import {Bridge} from "@romefrontend/events";
import {JSONObject, JSONPropertyValue} from "@romefrontend/codec-json";
import {
	RemoteReporterClientMessage,
	RemoteReporterReceiveMessage,
	ReporterStream,
} from "@romefrontend/cli-reporter";
import {ServerMarker} from "../../server/Server";

export type ServerQueryRequest = {
	requestFlags: ClientRequestFlags;
	commandFlags: JSONObject;
	args: Array<string>;
	commandName: string;
	silent: boolean;
	noData: boolean;
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
};

export type ServerQueryResponseInvalid = ServerQueryResponseBase & {
	type: "INVALID_REQUEST";
	diagnostics: Diagnostics;
	showHelp: boolean;
};

export type ServerQueryResponseCancelled = ServerQueryResponseBase & {
	type: "CANCELLED";
};

export type ServerQueryResponse =
	| ServerQueryResponseInvalid
	| ServerQueryResponseSuccess
	| ServerQueryResponseError
	| ServerQueryResponseCancelled
	| ServerQueryResponseDiagnostics;

export type ProfilingStartData = {
	samplingInterval: number;
};

export type ServerBridgeInfo = {
	version: string;
	columns: number;
	hasClearScreen: boolean;
	useRemoteReporter: boolean;
	unicode: boolean;
	format: ReporterStream["format"];
	flags: ClientFlagsJSON;
};

export default class ServerBridge extends Bridge {
	getClientInfo = this.createEvent<void, ServerBridgeInfo>({
		name: "getClientInfo",
		direction: "server->client",
	});

	stdout = this.createEvent<string, void>({
		name: "stdout",
		direction: "server->client",
	});

	stderr = this.createEvent<string, void>({
		name: "stderr",
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

	setColumns = this.createEvent<number, void>({
		name: "columns.set",
		direction: "server<-client",
	});

	reporterRemoteServerMessage = this.createEvent<
		RemoteReporterClientMessage,
		void
	>({
		name: "reporterRemoteToLocalMessage",
		direction: "server->client",
	});

	reporterRemoteClientMessage = this.createEvent<
		RemoteReporterReceiveMessage,
		void
	>({
		name: "reporterLocalToRemoteMessage",
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

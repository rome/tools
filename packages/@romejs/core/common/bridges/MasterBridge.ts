/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Profile} from "@romejs/v8";
import {Diagnostics} from "@romejs/diagnostics";
import {ClientFlagsJSON, ClientRequestFlags} from "../types/client";
import {Bridge} from "@romejs/events";
import {JSONObject, JSONPropertyValue} from "@romejs/codec-json";
import {
	RemoteReporterClientMessage,
	RemoteReporterReceiveMessage,
	ReporterStream,
} from "@romejs/cli-reporter";
import {MasterMarker} from "../../master/Master";

export type MasterQueryRequest = {
	requestFlags: ClientRequestFlags;
	commandFlags: JSONObject;
	args: Array<string>;
	commandName: string;
	silent: boolean;
	noData: boolean;
	terminateWhenIdle: boolean;
	cancelToken?: string;
};

export type PartialMasterQueryRequest = Partial<Omit<
	MasterQueryRequest,
	"requestFlags" | "commandName"
>> & {
	requestFlags?: Partial<ClientRequestFlags>;
	commandName: string;
};

export type MasterQueryResponseSuccess = {
	type: "SUCCESS";
	hasData: boolean;
	data: JSONPropertyValue;
	markers: Array<MasterMarker>;
};

export type MasterQueryResponseError = {
	type: "ERROR";
	fatal: boolean;
	handled: boolean;
	name: string;
	message: string;
	stack: undefined | string;
};

export type MasterQueryResponseDiagnostics = {
	type: "DIAGNOSTICS";
	hasDiagnostics: boolean;
	diagnostics: Diagnostics;
};

export type MasterQueryResponseInvalid = {
	type: "INVALID_REQUEST";
	diagnostics: Diagnostics;
	showHelp: boolean;
};

export type MasterQueryResponseCancelled = {
	type: "CANCELLED";
};

export type MasterQueryResponse =
	| MasterQueryResponseInvalid
	| MasterQueryResponseSuccess
	| MasterQueryResponseError
	| MasterQueryResponseCancelled
	| MasterQueryResponseDiagnostics;

export type ProfilingStartData = {
	samplingInterval: number;
};

export type MasterBridgeInfo = {
	version: string;
	columns: number;
	hasClearScreen: boolean;
	useRemoteReporter: boolean;
	unicode: boolean;
	format: ReporterStream["format"];
	flags: ClientFlagsJSON;
};

export default class MasterBridge extends Bridge {
	getClientInfo = this.createEvent<void, MasterBridgeInfo>({
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
			origin: "master" | "worker";
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

	query = this.createEvent<PartialMasterQueryRequest, MasterQueryResponse>({
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

	endMaster = this.createEvent<void, void>({
		name: "endMaster",
		direction: "server<-client",
	});
}

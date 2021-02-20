/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	PartialServerQueryRequest,
	ServerQueryResponse,
} from "../common/bridges/ServerBridge";
import {LocalCommand, localCommands} from "./commands";
import Client from "./Client";
import {consumeUnknown} from "@internal/consume";
import review from "./review";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import SilentClientError from "./SilentClientError";
import { ClientQueryResponse } from "../common/types/client";
import { isBridgeClosedDiagnosticError } from "@internal/events";

export type ClientRequestType = "local" | "server";

export default class ClientRequest {
	constructor(
		client: Client,
		type: ClientRequestType = "local",
		query: PartialServerQueryRequest,
	) {
		this.client = client;
		this.type = type;
		this.query = query;
	}

	public query: PartialServerQueryRequest;
	public type: ClientRequestType;
	public client: Client;

	public fork(query: PartialServerQueryRequest): ClientRequest {
		return new ClientRequest(this.client, this.type, query);
	}

	public async init(): Promise<ClientQueryResponse> {
		const {requestFlags} = this.query;
		if (requestFlags?.review) {
			return await this.initReview();
		} else {
			return await this.initCommand();
		}
	}

	private async initReview(): Promise<ClientQueryResponse> {
		return review(this);
	}

	public async initCommand(): Promise<ClientQueryResponse> {
		const localCommand = localCommands.get(this.query.commandName);

		if (this.type === "server" || localCommand === undefined) {
			return this.initFromServer();
		} else {
			return this.initFromLocal(localCommand);
		}
	}

	private async initFromLocal(
		// rome-ignore lint/ts/noExplicitAny: future cleanup
		localCommand: LocalCommand<any>,
	): Promise<ClientQueryResponse> {
		const {query} = this;

		let flags;
		if (localCommand.defineFlags !== undefined) {
			flags = localCommand.defineFlags(
				consumeUnknown(
					query.commandFlags,
					DIAGNOSTIC_CATEGORIES["flags/invalid"],
				),
			);
		}

		try {
			const res = await localCommand.callback(this, flags);
			if (res === true) {
				return {
					type: "SUCCESS",
					data: undefined,
					hasData: false,
					markers: [],
					files: {},
				};
			} else if (res === false) {
				return {
					type: "CLIENT_ERROR",
					message: `Command return`,
					markers: [],
				};
			} else {
				return res;
			}
		} catch (err) {
			if (err instanceof SilentClientError) {
				return {
					type: "CLIENT_ERROR",
					message: err.message,
					markers: [],
				};
			} else {
				throw err;
			}
		}
	}

	private async initFromServer(): Promise<ServerQueryResponse> {
		const {client} = this;

		try {
			const bridge = await client.findOrStartServer();
			return await bridge.events.query.call(this.query);
		} catch (err) {
			if (isBridgeClosedDiagnosticError(err)) {
				return {
					type: "CANCELLED",
					markers: [],
					reason: err.message,
				};
			} else {
				throw err;
			}
		}
	}
}

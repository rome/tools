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
import {BridgeError} from "@internal/events";

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

	public async init(): Promise<ServerQueryResponse> {
		const {requestFlags} = this.query;
		if (requestFlags !== undefined && requestFlags.review) {
			return await this.initReview();
		} else {
			return await this.initCommand();
		}
	}

	private async initReview(): Promise<ServerQueryResponse> {
		return review(this);
	}

	public async initCommand(): Promise<ServerQueryResponse> {
		const localCommand = localCommands.get(this.query.commandName);

		if (this.type === "server" || localCommand === undefined) {
			return this.initFromServer();
		} else {
			return this.initFromLocal(localCommand);
		}
	}

	private async initFromLocal(
		// rome-ignore lint/ts/noExplicitAny
		localCommand: LocalCommand<any>,
	): Promise<ServerQueryResponse> {
		const {query} = this;

		let flags;
		if (localCommand.defineFlags !== undefined) {
			flags = localCommand.defineFlags(
				consumeUnknown(query.commandFlags, "flags/invalid"),
			);
		}

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
				type: "EXIT",
				code: 1,
				markers: [],
			};
		} else {
			return res;
		}
	}

	private async initFromServer(): Promise<ServerQueryResponse> {
		const {client} = this;

		try {
			const bridge = await client.findOrStartServer();
			return await bridge.query.call(this.query);
		} catch (err) {
			if (err instanceof BridgeError) {
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

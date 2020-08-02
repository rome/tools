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
import {BridgeError} from "@internal/events";
import review from "./review";

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

	query: PartialServerQueryRequest;
	type: ClientRequestType;
	client: Client;

	fork(query: PartialServerQueryRequest): ClientRequest {
		return new ClientRequest(this.client, this.type, query);
	}

	async init(): Promise<ServerQueryResponse> {
		const {requestFlags} = this.query;
		if (requestFlags !== undefined && requestFlags.review) {
			return await this.initReview();
		} else {
			return await this.initCommand();
		}
	}

	async initReview(): Promise<ServerQueryResponse> {
		return review(this);
	}

	async initCommand(): Promise<ServerQueryResponse> {
		const localCommand = localCommands.get(this.query.commandName);

		if (this.type === "server" || localCommand === undefined) {
			return this.initFromServer();
		} else {
			return this.initFromLocal(localCommand);
		}
	}

	async initFromLocal(
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
				type: "ERROR",
				fatal: false,
				// Local command would have printed something
				handled: true,
				name: "Error",
				message: "Command was not successful",
				stack: undefined,
				markers: [],
			};
		} else {
			return res;
		}
	}

	async initFromServer(): Promise<ServerQueryResponse> {
		const {client} = this;

		try {
			const bridge = await client.findOrStartServer();
			return await bridge.query.call(this.query);
		} catch (err) {
			if (err instanceof BridgeError) {
				return {
					type: "ERROR",
					fatal: true,
					handled: false,
					name: "Error",
					message: "Server died while processing command. Results may be incomplete.",
					stack: undefined,
					markers: [],
				};
			} else {
				throw err;
			}
		}
	}
}

/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CLI_SOCKET_PATH,
	SOCKET_PATH,
	Server,
	ServerBridge,
} from "@internal/core";
import {createBridgeFromSocket} from "@internal/events";
import setProcessTitle from "./utils/setProcessTitle";
import net = require("net");

import {exists, removeFile} from "@internal/fs";

export default async function server() {
	setProcessTitle("server");

	const server = new Server({
		dedicated: true,
		globalErrorHandlers: true,
	});

	await server.init();

	const socketServer = net.createServer(function(socket) {
		const client = createBridgeFromSocket(
			ServerBridge,
			socket,
			{
				type: "client",
			},
		);
		server.attachToBridge(client);
	});

	if (await exists(SOCKET_PATH)) {
		await removeFile(SOCKET_PATH);
	}

	socketServer.listen(
		SOCKET_PATH.join(),
		() => {
			const socket = net.createConnection(
				CLI_SOCKET_PATH.join(),
				() => {
					socket.end();
				},
			);

			socket.on(
				"error",
				(err) => {
					// Socket error occured, cli could have died before it caught us
					err;
					console.log(err);
					process.exit();
				},
			);
		},
	);
}

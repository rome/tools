/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CLI_SOCKET_PATH,
	SERVER_SOCKET_PATH,
	Server,
	ServerBridge,
} from "@internal/core";
import setProcessTitle from "./utils/setProcessTitle";
import net = require("net");
import {loadUserConfig} from "@internal/core/common/userConfig";
import {isBridgeDisconnectedDiagnosticsError} from "@internal/events";

export default async function server() {
	setProcessTitle("server");

	const userConfig = await loadUserConfig();
	const server = new Server({
		userConfig,
		dedicated: true,
		daemon: true,
	});

	await server.init();

	const socketServer = net.createServer(function(socket) {
		const {bridge} = ServerBridge.Server.createFromSocket(socket);

		server.fatalErrorHandler.wrapPromise(
			server.createClient(bridge).catch((err) => {
				// Ignore bridge disconnect errors
				if (!isBridgeDisconnectedDiagnosticsError(err)) {
					throw err;
				}
			}),
		);
	});

	socketServer.on(
		"error",
		(err) => {
			server.fatalErrorHandler.handle(err);
		},
	);

	if (await SERVER_SOCKET_PATH.exists()) {
		await SERVER_SOCKET_PATH.removeFile();
	}

	await SERVER_SOCKET_PATH.getParent().createDirectory();

	socketServer.listen(
		SERVER_SOCKET_PATH.join(),
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
					server.fatalErrorHandler.handle(err);
				},
			);
		},
	);
}

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

import {createDirectory, exists, removeFile} from "@internal/fs";
import {loadUserConfig} from "@internal/core/common/userConfig";
import { BridgeError } from "@internal/events";

export default async function server() {
	setProcessTitle("server");

	const userConfig = await loadUserConfig();
	const server = new Server({
		userConfig,
		dedicated: true,
		globalErrorHandlers: true,
	});

	await server.init();

	const socketServer = net.createServer(function(socket) {
		const bridge = ServerBridge.Server.createFromSocket(socket);
		
		server.fatalErrorHandler.wrapPromise(server.attachToBridge(bridge).catch(err => {
			// Ignore bridge disconnect errors
			if (!(err instanceof BridgeError)) {
				throw err;
			}
		}));
	});

	socketServer.on("error", (err) => {
		server.fatalErrorHandler.handle(err);
	});

	if (await exists(SERVER_SOCKET_PATH)) {
		await removeFile(SERVER_SOCKET_PATH);
	}

	await createDirectory(SERVER_SOCKET_PATH.getParent());

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

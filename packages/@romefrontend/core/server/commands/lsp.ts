/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romefrontend/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import LSPServer from "../lsp/LSPServer";

export default createServerCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: "TODO",
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<void> {
		const {server, bridge, reporter} = req;

		// This way we can still log stuff since it'll be redirected to stderr and stdout will be reserved for messages
		reporter.redirectOutToErr(true);

		const lsp = new LSPServer(req);
		server.connectedLSPServers.add(lsp);

		bridge.endEvent.subscribe(() => {
			server.connectedLSPServers.delete(lsp);
		});

		const {transport} = lsp;

		bridge.lspFromClientBuffer.subscribe((chunk) => {
			transport.append(chunk);
		});

		transport.writeEvent.subscribe((msg) => {
			bridge.lspFromServerBuffer.send(msg);
		});

		await bridge.endEvent.wait();
	},
});

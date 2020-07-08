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
		const {server, bridge} = req;

		const lspServer = new LSPServer(req);
		server.connectedLSPServers.add(lspServer);

		bridge.endEvent.subscribe(() => {
			server.connectedLSPServers.delete(lspServer);
		});

		bridge.lspFromClientBuffer.subscribe((chunk) => {
			lspServer.append(chunk);
		});

		await bridge.endEvent.wait();
	},
});

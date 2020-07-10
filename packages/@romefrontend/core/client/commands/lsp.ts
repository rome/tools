/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories} from "../../common/commands";
import {createLocalCommand} from "../commands";
import ClientRequest from "../ClientRequest";

export default createLocalCommand({
	description: "connect to an lsp",
	category: commandCategories.PROJECT_MANAGEMENT,
	usage: "",
	examples: [],
	// vscode-languageclient adds these on
	ignoreFlags: ["stdio", "clientProcessId"],
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
		req.client.setFlags({
			clientName: "lsp",
			silent: true,
		});

		const stdin = req.client.reporter.getStdin();
		req.client.reporter.teardown();

		const bridge = await req.client.findOrStartServer();
		if (bridge === undefined) {
			return false;
		}

		bridge.lspFromServerBuffer.subscribe((chunk) => {
			req.client.derivedReporterStreams.stdoutWrite(chunk);
		});

		stdin.on(
			"data",
			(chunk) => {
				bridge.lspFromClientBuffer.call(chunk.toString());
			},
		);

		await req.client.query(
			{
				commandName: "lsp",
			},
			"server",
		);

		return true;
	},
});

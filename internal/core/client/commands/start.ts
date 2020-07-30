/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories} from "../../common/commands";
import {createLocalCommand} from "../commands";
import ClientRequest from "../ClientRequest";
import {markup} from "@internal/markup";

export default createLocalCommand({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`start daemon (if none running)`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
		const existingServer = await req.client.tryConnectToExistingDaemon();
		if (existingServer) {
			req.client.reporter.success(markup`Already running server.`);
			return true;
		}

		const bridge = await req.client.startDaemon();
		return bridge !== undefined;
	},
});

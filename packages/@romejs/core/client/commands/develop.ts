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
	category: commandCategories.PROCESS_MANAGEMENT,
	description: "TODO",
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
		const existingServer = await req.client.tryConnectToExistingDaemon();
		const hasExistingServer = existingServer !== undefined;

		if (!hasExistingServer) {
			await req.client.forceStartDaemon();
		}

		await req.client.query(
			{
				...req.query,
				terminateWhenIdle: true,
			},
			"server",
		);

		return true;
	},
});

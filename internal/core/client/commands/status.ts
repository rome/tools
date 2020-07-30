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
	description: markup`get the current daemon status`,
	category: commandCategories.PROCESS_MANAGEMENT,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
		const {reporter} = req.client;
		const bridge = await req.client.tryConnectToExistingDaemon();
		if (bridge) {
			const status = await req.client.query(
				{
					commandName: "status",
				},
				"server",
			);
			if (status.type === "SUCCESS") {
				reporter.inspect(status.data);
				return true;
			} else {
				return false;
			}
		} else {
			reporter.error(markup`Server not running.`);
			return false;
		}
	},
});

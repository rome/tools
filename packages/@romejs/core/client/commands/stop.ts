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
	description: "stop a running daemon if one exists",
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
		// We might want to use `terminateWhenIdle` here combined with a timeout instead of forcing it to die straight away
		const {reporter} = req.client;
		const bridge = await req.client.tryConnectToExistingDaemon();
		if (bridge) {
			const stop = await req.client.query(
				{
					commandName: "stop",
				},
				"master",
			);
			if (stop.type === "ERROR" && stop.fatal) {
				reporter.success("Stopped server.");
			} else {
				reporter.error("Failed to stop server.");
				return false;
			}
		} else {
			reporter.warn("No running server to stop.");
		}
		return true;
	},
});

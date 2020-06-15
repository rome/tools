/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romejs/core";
import {Consumer} from "@romejs/consume";
import {WebServer} from "../web/index";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";

type Flags = {
	port: number;
};

const DEFAULT_PORT = 8_081;

export default createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: "start a web server",
	usage: "",
	examples: [],
	defineFlags(c: Consumer): Flags {
		return {
			port: c.get("port").asNumber(DEFAULT_PORT),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		// Initialize cwd early since we'll need it for any requests
		await req.server.projectManager.findProject(req.client.flags.cwd);

		const web = new WebServer(req);
		web.listen(flags.port);

		req.endEvent.subscribe(() => {
			web.close();
		});

		await new Promise(() => {});
	},
});

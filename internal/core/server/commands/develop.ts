/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {createServerCommand} from "../commands";
import {commandCategories} from "../../common/commands";
import {markup} from "@internal/markup";
import DevelopServer, {DevelopServerListenOptions} from "../develop/DevelopServer";
import Bundler from "../bundler/Bundler";

export default createServerCommand<DevelopServerListenOptions>({
	category: commandCategories.INTERNAL,
	description: markup`d`,
	usage: "",
	examples: [],
	defineFlags(c) {
		return {
			public: c.get("public").default(false).asBoolean(),
			port: c.get("port").default(8_080).asNumber(),
		};
	},
	async callback(
		req: ServerRequest,
		flags: DevelopServerListenOptions,
	): Promise<void> {
		const {reporter} = req;
		const bundler = Bundler.createFromServerRequest(req);
		const resolution = await bundler.getResolvedEntry(".");

		const server = new DevelopServer({
			bundler,
			resolution,
			request: req,
			reporter,
		});

		const http = await server.listen(flags);
		req.endEvent.subscribe(async () => {
			http.close();
			await server.close();
		});

		await server.init();
		await req.endEvent.wait();
	},
});

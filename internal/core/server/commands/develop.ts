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
import DevelopServer from "../develop/DevelopServer";
import Bundler from "../bundler/Bundler";

type Flags = {
  port: number,
};

export default createServerCommand<Flags>({
	category: commandCategories.INTERNAL,
	description: markup`d`,
	usage: "",
	examples: [],
	defineFlags(c) {
		return {
      port: c.get("port").asNumber(8080),
    };
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
    const {reporter} = req;
    const bundler = Bundler.createFromServerRequest(req);
		const resolution = await bundler.getResolvedEntry(".");

    const server = new DevelopServer({
      bundler,
      resolution,
      request: req,
      reporter,
    });

    const http = await server.listen(flags.port);
    reporter.success(markup`Listening at <emphasis><hyperlink target="http://localhost:${String(flags.port)}" /></emphasis>`);
    req.endEvent.subscribe(async () => {
      http.close();
      await server.close();
    });
    
    await server.init();
    await req.endEvent.wait();
	},
});

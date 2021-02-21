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
    const server = new DevelopServer(req, req.reporter);
    await server.init();

    const http = server.listen(flags.port);
    req.endEvent.subscribe(() => {
      server.close();
      http.close();
    });

    await req.endEvent.wait();
	},
});

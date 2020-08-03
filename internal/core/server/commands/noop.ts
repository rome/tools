/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {markup} from "@internal/markup";

type Flags = {
	hang: boolean;
};

export default createServerCommand<Flags>({
	category: commandCategories.INTERNAL,
	description: markup`do nothing`,
	usage: "",
	examples: [],
	defineFlags(c) {
		return {
			hang: c.get(
				"hang",
				{description: markup`Hang rather than instantly quitting`},
			).asBoolean(false),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		if (flags.hang) {
			if (!req.server.options.dedicated) {
				req.reporter.warn(
					markup`Passed <emphasis>--hang</emphasis> flag but server not connected to a dedicated server so request will hang forever`,
				);
			}
			await req.endEvent.wait();
		}
	},
});

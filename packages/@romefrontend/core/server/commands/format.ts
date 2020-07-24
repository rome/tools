/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romefrontend/core";
import {createServerCommand} from "../commands";
import {commandCategories} from "../../common/commands";
import {Consumer} from "@romefrontend/consume";
import {markup} from "@romefrontend/cli-layout";

type Flags = {
	allowDiagnostics: boolean;
};

export default createServerCommand({
	category: commandCategories.INTERNAL,
	description: markup`formats a single file`,
	usage: "",
	examples: [],
	defineFlags(c: Consumer): Flags {
		return {
			allowDiagnostics: c.get("allowDiagnostics").asBoolean(false),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<undefined | string> {
		const {reporter} = req;
		const filename = await req.resolveEntryAssertPathArg(0);

		const res = await req.requestWorkerFormat(
			filename,
			{},
			{
				allowParserDiagnostics: flags.allowDiagnostics,
			},
		);

		if (res === undefined) {
			reporter.error(markup`No formatter for this file`);
			return undefined;
		} else {
			reporter.writeAll(res.formatted);
			return res.formatted;
		}
	},
});

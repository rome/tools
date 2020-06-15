/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romejs/core";
import {createServerCommand} from "../commands";
import {commandCategories} from "../../common/commands";
import {createUnknownFilePath} from "@romejs/path";
import {Consumer} from "@romejs/consume";

type Flags = {
	allowDiagnostics: boolean;
};

export default createServerCommand({
	category: commandCategories.INTERNAL,
	description: "TODO",
	usage: "",
	examples: [],
	defineFlags(c: Consumer): Flags {
		return {
			allowDiagnostics: c.get("allowDiagnostics").asBoolean(false),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<undefined | string> {
		const {reporter, server} = req;
		const {args} = req.query;
		req.expectArgumentLength(1);

		const filename = await server.resolver.resolveEntryAssertPath(
			{
				...req.getResolverOptionsFromFlags(),
				source: createUnknownFilePath(args[0]),
			},
			{location: req.getDiagnosticPointerFromFlags({type: "arg", key: 0})},
		);

		const res = await req.requestWorkerFormat(
			filename,
			{
				allowParserDiagnostics: flags.allowDiagnostics,
			},
		);

		if (res === undefined) {
			reporter.error("No formatter for this file");
			return undefined;
		} else {
			reporter.writeAll(res.formatted);
			return res.formatted;
		}
	},
});

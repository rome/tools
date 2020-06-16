/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romejs/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {createUnknownFilePath} from "@romejs/path";

export default createServerCommand({
	category: commandCategories.INTERNAL,
	description: "get the module type signature of a file",
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<void> {
		const {server, reporter} = req;
		const {args} = req.query;
		req.expectArgumentLength(1);

		const filename = await server.resolver.resolveEntryAssertPath(
			{
				...req.getResolverOptionsFromFlags(),
				source: createUnknownFilePath(args[0]),
			},
			{location: req.getDiagnosticPointerFromFlags({type: "arg", key: 0})},
		);
		reporter.inspect(await req.requestWorkerModuleSignature(filename, {}));
	},
});

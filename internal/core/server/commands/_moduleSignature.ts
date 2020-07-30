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

export default createServerCommand({
	category: commandCategories.INTERNAL,
	description: markup`get the module type signature of a file`,
	usage: "",
	examples: [],
	hidden: true,
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<void> {
		const {reporter} = req;
		const filename = await req.resolveEntryAssertPathArg(0);
		reporter.inspect(await req.requestWorkerModuleSignature(filename, {}));
	},
});

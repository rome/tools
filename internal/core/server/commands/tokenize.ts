/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {Consumer} from "@internal/consume";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {ConstJSSourceType} from "@internal/ast";
import {markup} from "@internal/markup";

type Flags = {
	sourceType: undefined | ConstJSSourceType;
};

export default createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`tokenize a single file and dump tokens`,
	usage: "",
	examples: [],
	hidden: true,
	defineFlags(c: Consumer): Flags {
		return {
			sourceType: c.get("sourceType").asStringSetOrVoid(["module", "script"]),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		const {reporter} = req;
		const filename = await req.resolveEntryAssertPathArg(0);

		let tokens = await req.requestWorkerTokenize(
			filename,
			{
				sourceTypeJS: flags.sourceType,
			},
		);

		reporter.inspect(tokens);
	},
});

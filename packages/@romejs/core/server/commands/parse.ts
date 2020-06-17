/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romejs/core";
import {Consumer} from "@romejs/consume";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {ConstSourceType, jsRoot} from "@romejs/ast";
import {removeLoc} from "@romejs/js-ast-utils";

type Flags = {
	allowDiagnostics: boolean;
	compact: boolean;
	sourceType: undefined | ConstSourceType;
};

export default createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: "parse a single file and dump its ast",
	usage: "",
	examples: [],
	defineFlags(c: Consumer): Flags {
		return {
			allowDiagnostics: c.get("allowDiagnostics").asBoolean(false),
			compact: c.get("compact").asBoolean(true),
			sourceType: c.get("sourceType").asStringSetOrVoid(["module", "script"]),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		const {reporter} = req;
		const filename = await req.resolveEntryAssertPathArg(0);

		let ast = await req.requestWorkerParse(
			filename,
			{
				sourceType: flags.sourceType,
				allowParserDiagnostics: flags.allowDiagnostics,
			},
		);

		if (flags.compact) {
			ast = jsRoot.assert(removeLoc(ast));
		}

		reporter.inspect(ast);
	},
});

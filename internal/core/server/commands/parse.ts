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
import {assertRoot, removeLoc} from "@internal/ast-utils";
import {markup} from "@internal/markup";
import {assertSingleNode} from "@internal/js-ast-utils";

type Flags = {
	allowDiagnostics: boolean;
	compact: boolean;
	sourceType: undefined | ConstJSSourceType;
};

export default createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`parse a single file and dump its ast`,
	usage: "",
	examples: [],
	hidden: true,
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
				sourceTypeJS: flags.sourceType,
				allowParserDiagnostics: flags.allowDiagnostics,
			},
		);

		if (flags.compact) {
			ast = assertRoot(assertSingleNode(removeLoc(ast)));
		}

		reporter.inspect(ast);
	},
});

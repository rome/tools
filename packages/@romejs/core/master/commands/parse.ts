/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from "@romejs/core";
import {Consumer} from "@romejs/consume";
import {commandCategories} from "../../common/commands";
import {createMasterCommand} from "../commands";
import {createUnknownFilePath} from "@romejs/path";
import {ConstSourceType, JSRoot} from "@romejs/ast";
import {removeLoc} from "@romejs/js-ast-utils";

type Flags = {
	allowDiagnostics: boolean;
	compact: boolean;
	sourceType: undefined | ConstSourceType;
};

export default createMasterCommand({
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
	async callback(req: MasterRequest, flags: Flags): Promise<void> {
		const {master, reporter} = req;
		const {args} = req.query;
		req.expectArgumentLength(1);

		const filename = await master.resolver.resolveEntryAssertPath(
			{
				...req.getResolverOptionsFromFlags(),
				source: createUnknownFilePath(args[0]),
			},
			{location: req.getDiagnosticPointerFromFlags({type: "arg", key: 0})},
		);

		let ast = await req.requestWorkerParse(
			filename,
			{
				sourceType: flags.sourceType,
				allowParserDiagnostics: flags.allowDiagnostics,
			},
		);

		if (flags.compact) {
			ast = JSRoot.assert(removeLoc(ast));
		}

		reporter.inspect(ast);
	},
});

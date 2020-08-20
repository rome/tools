/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {WorkerCompileResult} from "../../common/bridges/WorkerBridge";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {DiagnosticsError} from "@internal/diagnostics";
import {Consumer} from "@internal/consume";
import Bundler from "../bundler/Bundler";
import {markup} from "@internal/markup";

type Flags = {
	bundle: boolean;
};

export default createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`compile a single file`,
	usage: "",
	examples: [],
	hidden: true,
	defineFlags(c: Consumer): Flags {
		return {
			bundle: c.get("bundle").asBoolean(false),
		};
	},
	async callback(req: ServerRequest, commandFlags: Flags): Promise<void> {
		const {reporter} = req;
		const resolved = await req.resolveEntryAssertPathArg(0);

		let res: WorkerCompileResult;
		if (commandFlags.bundle) {
			const bundler = Bundler.createFromServerRequest(req);
			res = await bundler.compileSingle(resolved);
		} else {
			res = await req.requestWorkerCompile(resolved, "compile", {}, {});
		}

		const {compiledCode, diagnostics, suppressions}: WorkerCompileResult = res;

		if (diagnostics.length > 0) {
			throw new DiagnosticsError(
				"Compile diagnostics",
				diagnostics,
				suppressions,
			);
		}

		reporter.write(compiledCode);
	},
});

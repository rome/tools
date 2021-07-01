/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest, WorkerCompileResult} from "@internal/core";

import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {DiagnosticsError} from "@internal/diagnostics";
import {Consumer} from "@internal/consume";
import Bundler from "../bundler/Bundler";
import {markup} from "@internal/markup";

type Flags = {
	bundle: boolean;
	target: string;
};

export default createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`compile a single file`,
	usage: "",
	examples: [],
	hidden: true,
	defineFlags(c: Consumer): Flags {
		return {
			bundle: c.get("bundle").required(false).asBoolean(),
			target: c.get("target").required("default").asString(),
		};
	},
	async callback(req: ServerRequest, commandFlags: Flags): Promise<void> {
		const {reporter} = req;
		const resolved = await req.resolveEntryAssertPathArg(0);

		let res: WorkerCompileResult;
		if (commandFlags.bundle) {
			const bundler = Bundler.createFromServerRequest(
				req,
				{
					target: commandFlags.target,
				},
			);
			res = await bundler.compileSingle(resolved);
		} else {
			res = await req.requestWorkerCompile(
				resolved,
				"compile",
				{target: commandFlags.target},
				{},
			);
		}

		const {compiledCode, diagnostics} = res.value;

		if (diagnostics.length > 0) {
			throw new DiagnosticsError("Compile diagnostics", diagnostics);
		}

		reporter.write(compiledCode);
	},
});

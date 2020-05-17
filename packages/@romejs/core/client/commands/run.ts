/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories} from "../../common/commands";
import {createLocalCommand} from "../commands";
import ClientRequest from "../ClientRequest";
import {consumeUnknown} from "@romejs/consume";
import executeMain from "../../common/utils/executeMain";
import {createAbsoluteFilePath} from "@romejs/path";
import {createSingleDiagnosticError} from "@romejs/diagnostics";
import {SourceMapConsumer} from "@romejs/codec-source-map";

export default createLocalCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: "TODO",
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
		const bridge = await req.client.findOrStartMaster();
		if (bridge === undefined) {
			return false;
		}

		process.on(
			"unhandledRejection",
			(error) => {
				error;
				//console.log('unhandledRejection', error.stack);
			},
		);

		const res = await req.client.query(
			{
				commandName: "run",
				args: req.query.args,
			},
			"master",
		);

		if (res.type !== "SUCCESS") {
			return false;
		}

		const data = consumeUnknown(res.data, "parse/json");

		if (data.exists()) {
			const type = data.get("type").asString();

			switch (type) {
				case "executeCode": {
					process.execArgv = [...process.execArgv, process.argv[1], "run"];
					process.argv = [
						process.argv[0],
						String(data.filename),
						...process.argv.slice(4),
					];
					const {syntaxError} = await executeMain({
						path: createAbsoluteFilePath(data.get("filename").asString()),
						code: data.get("code").asString(),
						sourceMap: SourceMapConsumer.fromJSON(data.get("map").asAny()),
					});
					if (syntaxError !== undefined) {
						throw createSingleDiagnosticError(syntaxError);
					}
					await new Promise(() => {});
					break;
				}
			}
		}

		return true;
	},
});

/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories} from "../../common/commands";
import {createLocalCommand} from "../commands";
import ClientRequest from "../ClientRequest";
import {consumeUnknown} from "@romefrontend/consume";
import executeMain from "../../common/utils/executeMain";
import {createAbsoluteFilePath} from "@romefrontend/path";
import {createSingleDiagnosticError} from "@romefrontend/diagnostics";
import {SourceMapConsumer} from "@romefrontend/codec-source-map";
import {getEnvVar} from "@romefrontend/cli-environment";

export default createLocalCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: "TODO",
	usage: "",
	examples: [],
	hidden: getEnvVar("ROME_DEV").type !== "ENABLED",
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
		const bridge = await req.client.findOrStartServer();
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
			"server",
		);

		if (res.type !== "SUCCESS") {
			return false;
		}

		const data = consumeUnknown(res.data, "parse/json");

		if (data.exists()) {
			const type = data.get("type").asString();

			switch (type) {
				case "executeCode": {
					const {syntaxError} = await executeMain({
						args: req.query.args,
						path: createAbsoluteFilePath(data.get("filename").asString()),
						code: data.get("code").asString(),
						sourceMap: SourceMapConsumer.fromJSON(data.get("map").asAny()),
					});
					if (syntaxError !== undefined) {
						throw createSingleDiagnosticError(syntaxError);
					}
					break;
				}
			}
		}

		return true;
	},
});

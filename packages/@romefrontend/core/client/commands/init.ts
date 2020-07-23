/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories} from "../../common/commands";
import {createLocalCommand} from "../commands";
import ClientRequest from "../ClientRequest";
import {Dict} from "@romefrontend/typescript-helpers";
import {exists, writeFile} from "@romefrontend/fs";
import {stringifyRJSON} from "@romefrontend/codec-json";
import {markup} from "@romefrontend/cli-layout";

export default createLocalCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`create a project config`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
		const {reporter} = req.client;

		const projectPath = req.client.flags.cwd;
		const configPath = projectPath.append("rome.rjson");

		if (await exists(configPath)) {
			reporter.error(
				markup`<filelink target="${configPath.join()}" emphasis>rome.rjson</filelink> file already exists`,
			);
			reporter.info(
				markup`Use <code>rome config</code> to update an existing config`,
			);
			return false;
		}

		const config: Dict<unknown> = {};

		async function writeConfig() {
			await writeFile(configPath, stringifyRJSON(config));
			reporter.success(markup`Created config ${configPath}`);
		}
		// Run lint, capture diagnostics
		const response = await req.client.query(
			{
				commandName: "check",
				silent: true,
			},
			"server",
		);
		let globals: Array<string> = [];
		if (response.type === "DIAGNOSTICS") {
			if (response.hasDiagnostics) {
				response.diagnostics.forEach((d) => {
					if (d.description.category === "lint/js/undeclaredVariables") {
						if (d.meta && d.meta.identifierName) {
							globals.push(d.meta.identifierName);
						}
					}
				});
			}
		}
		if (globals.length > 0) {
			config["globals"] = globals;
		}

		await writeConfig();
		await req.client.query(
			{
				commandName: "init",
			},
			"server",
		);
		return true;
	},
});

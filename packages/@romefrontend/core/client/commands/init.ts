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
	hidden: true,
	defineFlags() {
		return {};
	},
	async callback(req: ClientRequest) {
		const {reporter} = req.client;

		const configPath = req.client.flags.cwd.append("rome.rjson");
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
		await writeConfig();

		async function writeConfig() {
			await writeFile(configPath, stringifyRJSON(config));
		}

		// Run lint, capture diagnostics

		reporter.success(markup`Created config ${configPath}`);

		return true;
	},
});

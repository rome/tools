/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romefrontend/core";
import {ProjectDefinition} from "@romefrontend/project";
import {SourceMap} from "@romefrontend/codec-source-map";
import Bundler from "../bundler/Bundler";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {AbsoluteFilePath, createRelativeFilePath} from "@romefrontend/path";

// This will be dispatched to the client where it has a special case for `executeCode`
type RunResult = {
	type: "executeCode";
	filename: string;
	code: string;
	map: SourceMap;
};

export default createServerCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: "TODO",
	usage: "",
	examples: [],
	hidden: true,
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<RunResult> {
		const {flags} = req.client;
		const {server} = req;
		req.expectArgumentLength(1);
		const arg = req.query.args[0];

		async function executeCode(path: AbsoluteFilePath): Promise<RunResult> {
			const bundler = Bundler.createFromServerRequest(req);
			const {entry} = await bundler.bundle(path);
			return {
				type: "executeCode",
				filename: path.join(),
				code: entry.js.content,
				map: entry.sourceMap.map.serialize(),
			};
		}

		// Get the current project
		const project: undefined | ProjectDefinition = await server.projectManager.findProject(
			flags.cwd,
		);

		// Check for bin files in any manifests that belong to any projects
		if (project !== undefined) {
			for (const {manifest, folder} of project.packages.values()) {
				const relative = manifest.bin.get(arg);
				if (relative === undefined) {
					continue;
				}

				const resolved = await server.resolver.resolveEntryAssertPath({
					...req.getResolverOptionsFromFlags(),
					origin: folder,
					platform: "node",
					source: createRelativeFilePath(relative),
				});

				return executeCode(resolved);
			}
		}

		// TODO check node_modules/.bin

		// TODO check package.json scripts

		// Assumed absolute paths
		const target = await req.resolveEntryAssertPathArg(0);
		return executeCode(target);
	},
});

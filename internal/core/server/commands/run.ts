/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {ProjectDefinition} from "@internal/project";
import Bundler from "../bundler/Bundler";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {AbsoluteFilePath, createRelativePath} from "@internal/path";
import {markup} from "@internal/markup";

export default createServerCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`TODO`,
	usage: "",
	examples: [],
	hidden: true,
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<void> {
		const {flags} = req.client;
		const {server} = req;
		req.expectArgumentLength(1, Infinity);
		const [arg, ...args] = req.query.args;

		async function executeCode(path: AbsoluteFilePath): Promise<void> {
			const bundler = Bundler.createFromServerRequest(req);
			const workerPromise = server.workerManager.spawnWorkerUnsafe({
				type: "script-runner",
				pipeIO: false,
				env: req.client.env,
			});
			workerPromise.then((worker) => {
				req.resources.add(worker.thread);
			});

			const [bundle, workerContainer] = await Promise.all([
				await bundler.bundle(path),
				workerPromise,
			]);

			const {entry} = bundle;

			const {worker} = workerContainer.thread;

			worker.on(
				"exit",
				(exitCode) => {
					req.teardown({
						type: "EXIT",
						code: Number(exitCode),
						markers: [],
					});
				},
			);

			worker.stderr.on(
				"data",
				(data) => {
					req.bridge.events.write.call([data, true]);
				},
			);

			worker.stdout.on(
				"data",
				(data) => {
					req.bridge.events.write.call([data, false]);
				},
			);

			const {exitCode} = await workerContainer.bridge.events.executeScript.call({
				contextDirectory: server.projectManager.getRootProjectForPath(path).directory,
				cwd: req.client.flags.cwd,
				args,
				path,
				code: await entry.js.content(),
			});
			if (exitCode !== undefined) {
				req.teardown({
					type: "EXIT",
					code: exitCode,
					markers: [],
				});
			}
		}

		// Get the current project
		const project: undefined | ProjectDefinition = await server.projectManager.findProject(
			flags.cwd,
		);

		// Check for bin files in any manifests that belong to any projects
		if (project !== undefined) {
			for (const {manifest, directory} of project.packages.values()) {
				const relative = manifest.bin.get(arg);
				if (relative === undefined) {
					continue;
				}

				const resolved = await server.resolver.resolveEntryAssertPath({
					...req.getResolverOptionsFromFlags(),
					origin: directory,
					platform: "node",
					source: createRelativePath(relative),
				});

				return executeCode(resolved);
			}
		}

		// TODO check node_modules/.bin

		// TODO check package.json scripts

		// Resolve path otherwise
		const target = await req.resolveEntryAssertPathArg(0, false);
		return executeCode(target);
	},
});

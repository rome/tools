/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romejs/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {assertHardMeta, modifyProjectConfig} from "@romejs/project";
import {createUnknownFilePath} from "@romejs/path";
import {escapeMarkup, markup} from "@romejs/string-markup";
import {descriptions} from "@romejs/diagnostics";

export default createServerCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: "Modify a project config",
	usage: "(enable|disable|set) key [value]",
	examples: [
		{
			command: "set name my_awesome_project",
			description: "Set the project name",
		},
	],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<void> {
		const {reporter} = req;
		req.expectArgumentLength(2, 3);

		const project = await req.assertClientCwdProject();

		let mutation: "set" | "push-array" = "set";
		let keyParts: string;
		let value: boolean | string | Array<string>;

		const [action, ...restArgs] = req.query.args;
		switch (action) {
			case "enable": {
				req.expectArgumentLength(2);
				keyParts = req.query.args[1];
				value = true;
				break;
			}

			case "disable": {
				req.expectArgumentLength(2);
				keyParts = req.query.args[1];
				value = false;
				break;
			}

			case "set-directory": {
				req.expectArgumentLength(3);
				[keyParts, value] = restArgs;

				// If the value is an absolute path, then make it relative to the project folder
				const path = createUnknownFilePath(value);
				if (path.isAbsolute()) {
					value = assertHardMeta(project.meta).projectFolder.relative(path).join();
				}

				break;
			}

			case "set": {
				req.expectArgumentLength(3);
				[keyParts, value] = restArgs;
				break;
			}

			case "push": {
				req.expectArgumentLength(3, Infinity);
				[keyParts, ...value] = restArgs;
				mutation = "push-array";
				break;
			}

			default:
				throw req.throwDiagnosticFlagError({
					description: descriptions.FLAGS.UNKNOWN_ACTION(action),
					target: {
						type: "arg",
						key: 0,
					},
				});
		}

		try {
			await modifyProjectConfig(
				project.meta,
				{
					pre: (meta) => {
						reporter.success(
							`${mutation === "set" ? "Setting" : "Adding"} <emphasis>${keyParts}</emphasis> to <emphasis>${escapeMarkup(
								JSON.stringify(value),
							)}</emphasis> in the project config ${meta.configPath.toMarkup({
								emphasis: true,
							})}`,
						);

						if (value === "true" || value === "false") {
							const suggestedCommand = value === "true" ? "enable" : "disable";
							reporter.warn(
								markup`Value is the string <emphasis>${value}</emphasis> but it looks like a boolean. You probably meant to use the command:`,
							);
							reporter.command(markup`config ${suggestedCommand} ${keyParts}`);
						}
					},
					modify: (consumer) => {
						// Set the specified value
						let keyConsumer = consumer;
						for (const key of keyParts.split(".")) {
							if (!keyConsumer.exists()) {
								keyConsumer.setValue({});
							}
							keyConsumer = keyConsumer.get(key);
						}

						switch (mutation) {
							case "set": {
								keyConsumer.setValue(value);
								break;
							}

							case "push-array": {
								keyConsumer.setValue([
									...keyConsumer.asArray(true).map((c) => c.asUnknown()),
									...Array.isArray(value) ? value : [],
								]);
								break;
							}
						}
					},
				},
			);
		} catch (err) {
			reporter.error(
				"Error occured while testing new project config. Your changes have not been saved.",
			);
			throw err;
		}
	},
});

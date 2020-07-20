/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romefrontend/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {assertHardMeta, normalizeProjectConfig} from "@romefrontend/project";
import {
	AbsoluteFilePath,
	HOME_PATH,
	createUnknownFilePath,
} from "@romefrontend/path";
import {escapeMarkup, markup} from "@romefrontend/cli-layout";
import {descriptions, interceptDiagnostics} from "@romefrontend/diagnostics";
import {Consumer} from "@romefrontend/consume";
import {
	ConsumeJSONResult,
	consumeJSONExtra,
	stringifyJSONExtra,
} from "@romefrontend/codec-json";
import {readFileText, writeFile} from "@romefrontend/fs";
import {
	loadUserConfig,
	normalizeUserConfig,
} from "@romefrontend/core/common/userConfig";

type Flags = {
	user: boolean;
};

export default createServerCommand<Flags>({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: "modify a project config",
	usage: "(enable|disable|set) key [value]",
	examples: [
		{
			command: "set name my_awesome_project",
			description: "Set the project name",
		},
	],
	defineFlags(c) {
		return {
			user: c.get("user").asBoolean(false),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		const {reporter} = req;
		req.expectArgumentLength(2, 3);

		// This is a bit janky because we want to allow calling `--user` outside a project
		const project = await req.server.projectManager.findProject(
			req.client.flags.cwd,
		);
		const cwd = project?.directory || HOME_PATH;

		const [action, keyParts, ...values] = req.query.args;
		let value: boolean | string | Array<string> = values[0];
		switch (action) {
			case "location": {
				req.expectArgumentLength(1);
				break;
			}

			case "enable": {
				req.expectArgumentLength(2);
				value = true;
				break;
			}

			case "disable": {
				req.expectArgumentLength(2);
				value = false;
				break;
			}

			case "set-directory": {
				req.expectArgumentLength(3);

				// If the value is an absolute path, then make it relative to the project directory
				const path = createUnknownFilePath(value);
				if (path.isAbsolute()) {
					value = cwd.relative(path).join();
				}

				break;
			}

			case "set": {
				req.expectArgumentLength(3);
				break;
			}

			case "push": {
				req.expectArgumentLength(3, Infinity);
				value = values;
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

		function modify(consumer: Consumer) {
			// Set the specified value
			let keyConsumer = consumer;
			for (const key of keyParts.split(".")) {
				if (!keyConsumer.exists()) {
					keyConsumer.setValue({});
				}
				keyConsumer = keyConsumer.get(key);
			}

			switch (action) {
				case "push": {
					keyConsumer.setValue([
						...Array.from(keyConsumer.asIterable(true), (c) => c.asUnknown()),
						...(Array.isArray(value) ? value : []),
					]);
					break;
				}

				default: {
					keyConsumer.setValue(value);
					break;
				}
			}
		}

		async function handleConfig(
			configPath: AbsoluteFilePath,
			subKey: string | undefined,
			validate: (res: ConsumeJSONResult, stringified: string) => void,
		) {
			if (action === "location") {
				reporter.logAllRaw(configPath.join());
				return;
			}

			reporter.success(
				`${action === "push" ? "Adding" : "Setting"} <emphasis>${keyParts}</emphasis> to <emphasis>${escapeMarkup(
					JSON.stringify(value),
				)}</emphasis> in the config <emphasis>${configPath.toMarkup()}</emphasis>`,
			);

			if (value === "true" || value === "false") {
				const suggestedCommand = value === "true" ? "enable" : "disable";
				reporter.warn(
					markup`Value is the string <emphasis>${value}</emphasis> but it looks like a boolean. You probably meant to use the command:`,
				);
				reporter.command(markup`config ${suggestedCommand} ${keyParts}`);
			}

			// Load the config file again
			const configFile = await readFileText(configPath);
			const res = consumeJSONExtra({
				path: configPath,
				input: configFile,
			});

			const {consumer} = res;
			if (subKey === undefined) {
				modify(consumer);
			} else {
				modify(consumer.get(subKey));
			}

			// Stringify the config
			const stringified = stringifyJSONExtra(res);

			// Test if this project config doesn't result in errors
			await interceptDiagnostics(
				async () => {
					// Reconsume with new stringified config
					const res = consumeJSONExtra({
						path: configPath,
						input: stringified,
					});

					validate(res, stringified);
				},
				(processor) => {
					processor.normalizer.setInlineSourceText(
						configPath.join(),
						stringified,
					);
				},
			);

			// Write it out
			await writeFile(configPath, stringified);
		}

		try {
			if (flags.user) {
				let {configPath: existingConfigPath} = loadUserConfig();

				let configPath: AbsoluteFilePath;
				if (existingConfigPath === undefined) {
					configPath = HOME_PATH.appendList(".config", "rome.rjson");
					await writeFile(configPath, "");
					reporter.info(
						`Created user config at <emphasis>${configPath.toMarkup()}</emphasis> as it did not exist`,
					);
				} else {
					configPath = existingConfigPath;
				}

				await handleConfig(
					configPath,
					undefined,
					(res) => {
						normalizeUserConfig(res.consumer, configPath);
					},
				);
			} else {
				const project = await req.assertClientCwdProject();
				const meta = assertHardMeta(project.meta);
				const {configPath, configSourceSubKey} = meta;

				await handleConfig(
					configPath,
					configSourceSubKey,
					(res, stringified) => {
						normalizeProjectConfig(
							res,
							meta.configPath,
							stringified,
							meta.projectDirectory,
						);
					},
				);
			}
		} catch (err) {
			reporter.warn(
				"Error occured while validating new config. Your changes have not been saved. Listed locations are not accurate.",
			);
			throw err;
		}
	},
});

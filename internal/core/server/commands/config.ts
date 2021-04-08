/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {normalizeProjectConfig} from "@internal/project";
import {AbsoluteFilePath, createPath} from "@internal/path";
import {markup} from "@internal/markup";
import {interceptDiagnostics} from "@internal/diagnostics";
import {Consumer} from "@internal/consume";
import {
	ConsumeConfigResult,
	consumeConfig,
	stringifyConfig,
} from "@internal/codec-config";
import {CachedFileReader} from "@internal/fs";
import {
	loadUserConfig,
	normalizeUserConfig,
} from "@internal/core/common/userConfig";
import {USER_CONFIG_DIRECTORY} from "@internal/core/common/constants";
import prettyFormat from "@internal/pretty-format";

type Flags = {
	user: boolean;
};

function defineFlags(c: Consumer): Flags {
	return {
		user: c.get("user").required(false).asBoolean(),
	};
}

async function runCommand(
	req: ServerRequest,
	flags: Flags,
	value: boolean | string | (string[]),
	action: string,
) {
	const {reporter} = req;
	const [keyParts] = req.query.args;

	function modify(consumer: Consumer) {
		// Set the specified value
		let keyConsumer = consumer;
		for (const key of keyParts.split(".")) {
			if (!keyConsumer.exists()) {
				keyConsumer.setValue({});
			}
			keyConsumer = keyConsumer.get(key);
		}

		if (action === "push") {
			keyConsumer.setValue([
				...Array.from(keyConsumer.required([]).asIterable(), (c) => c.asUnknown()),
				...(Array.isArray(value) ? value : []),
			]);
		} else {
			keyConsumer.setValue(value);
		}
	}

	async function handleConfig(
		configPath: AbsoluteFilePath,
		subKey: string | undefined,
		validate: (res: ConsumeConfigResult, stringified: string) => Promise<void>,
	) {
		if (action === "location") {
			reporter.log(configPath);
			return;
		}

		reporter.success(
			markup`${action === "push" ? "Adding" : "Setting"} <emphasis>${prettyFormat(
				value,
			)}</emphasis> to <emphasis>${keyParts}</emphasis> in the config <emphasis>${configPath}</emphasis>`,
		);

		if (value === "true" || value === "false") {
			const suggestedCommand = value === "true" ? "enable" : "disable";
			reporter.warn(
				markup`Value is the string <emphasis>${value}</emphasis> but it looks like a boolean. You probably meant to use the command:`,
			);
			reporter.command(`config ${suggestedCommand} ${keyParts}`);
		}

		// Load the config file again
		const configFile = await configPath.readFileText();
		const res = consumeConfig({
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
		const stringified = stringifyConfig(res);

		// Test if this project config doesn't result in errors
		await interceptDiagnostics(
			async () => {
				// Reconsume with new stringified config
				const res = consumeConfig({
					path: configPath,
					input: stringified,
				});

				await validate(res, stringified);

				return {};
			},
			(processor) => {
				processor.normalizer.setInlineSourceText(configPath, stringified);
			},
		);

		// Write it out
		await configPath.writeFile(stringified);
	}

	try {
		if (flags.user) {
			let {configPath: existingConfigPath} = await loadUserConfig();

			let configPath: AbsoluteFilePath;
			if (existingConfigPath === undefined) {
				configPath = USER_CONFIG_DIRECTORY.append("rome.json");
				await configPath.writeFile("");
				reporter.info(
					markup`Created user config at <emphasis>${configPath}</emphasis> as it did not exist`,
				);
			} else {
				configPath = existingConfigPath;
			}

			await handleConfig(
				configPath,
				undefined,
				async (res) => {
					await normalizeUserConfig(res.consumer, configPath);
				},
			);
		} else {
			const project = await req.assertClientCwdProject();
			const {meta} = project;
			const {configPath, configSourceSubKey} = meta;
			const rootProject = project.root ?? project;

			await handleConfig(
				configPath,
				configSourceSubKey,
				async (res) => {
					await normalizeProjectConfig(
						res,
						{
							reader: new CachedFileReader(),
							configPath: meta.configPath,
							projectDirectory: project.directory,
							rootProjectDirectory: rootProject.directory,
						},
					);
				},
			);
		}
	} catch (err) {
		reporter.warn(
			markup`Error occured while validating new config. Your changes have not been saved. Listed locations are not accurate.`,
		);
		throw err;
	}
}

export const location = createServerCommand<Flags>({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`show the config location`,
	usage: "",
	examples: [],
	defineFlags,
	async callback(req, flags) {
		req.expectArgumentLength(0);
		await runCommand(req, flags, "", "location");
	},
});

export const enable = createServerCommand<Flags>({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`modify a project config- set the ${"<key>"} to true`,
	usage: "<key>",
	examples: [],
	defineFlags,
	async callback(req, flags) {
		req.expectArgumentLength(1);
		await runCommand(req, flags, true, "enable");
	},
});

export const disable = createServerCommand<Flags>({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`modify a project config- set the ${"<key>"} to false`,
	usage: "<key>",
	examples: [],
	defineFlags,
	async callback(req, flags) {
		req.expectArgumentLength(1);
		await runCommand(req, flags, false, "disable");
	},
});

export const setDirectory = createServerCommand<Flags>({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`modify a project config - set the ${"<key>"} to ${"<value>"}`,
	usage: "<key> <value>",
	examples: [],
	defineFlags,
	async callback(req, flags) {
		req.expectArgumentLength(2);

		let value = req.query.args[1];
		const path = createPath(value);

		// If the value is an absolute path, then make it relative to the project directory
		if (path.isAbsolute()) {
			let cwd;
			if (flags.user) {
				// Relative to home user config
				cwd = USER_CONFIG_DIRECTORY;
			} else {
				// Relative to project config folder
				const project = await req.assertClientCwdProject();
				cwd = project.meta.configPath.getParent();
			}

			value = cwd.relative(path).join();
		}

		await runCommand(req, flags, value, "setDirectory");
	},
});

export const set = createServerCommand<Flags>({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`modify a project config - set the ${"<key>"} to ${"<value>"}`,
	usage: "<key> <value>",
	examples: [],
	defineFlags,
	async callback(req, flags) {
		req.expectArgumentLength(2);
		await runCommand(req, flags, req.query.args[1], "set");
	},
});

export const push = createServerCommand<Flags>({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`modify a project config - push ${"<values>"} to ${"<key>"}`,
	usage: "<key> <...values>",
	examples: [],
	defineFlags,
	async callback(req, flags) {
		req.expectArgumentLength(2, Infinity);
		await runCommand(req, flags, req.query.args.slice(1), "push");
	},
});

/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest, VERSION} from "@internal/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {
	PROJECT_CONFIG_PACKAGE_JSON_FIELD,
	normalizeProjectConfig,
} from "@internal/project";
import {AbsoluteFilePath, createPath} from "@internal/path";
import {markup} from "@internal/markup";
import {
	createSingleDiagnosticsError,
	descriptions,
	interceptDiagnostics,
} from "@internal/diagnostics";
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
import {Migrator} from "@internal/core/server/migrate/Migrator";
import {checkVSCWorkingDirectory} from "@internal/core/server/utils/checkVCSWorkingDirectory";

type Flags = {
	user: boolean;
};

function defineFlags(c: Consumer): Flags {
	return {
		user: c.get("user").required(false).asBoolean(),
	};
}

type Action =
	| "push"
	| "set"
	| "location"
	| "enable"
	| "disable"
	| "remove"
	| "pop"
	| "setDirectory";

async function runCommand(
	req: ServerRequest,
	flags: Flags,
	value: boolean | string | (string[]),
	action: Action,
) {
	const {reporter} = req;
	const [keyPath] = req.query.args;

	function modify(consumer: Consumer) {
		// Set the specified value
		let keyConsumer = consumer;
		let prevKeyConsumer = consumer;
		const keyParts = keyPath.split(".");

		for (const key of keyParts) {
			if (!keyConsumer.exists()) {
				keyConsumer.setValue({});
			}
			prevKeyConsumer = keyConsumer;
			keyConsumer = keyConsumer.get(key);
		}

		if (action === "push") {
			keyConsumer.setValue([
				...Array.from(
					keyConsumer.required([]).asIterable(),
					(c) => c.asUnknown(),
				),
				...(Array.isArray(value) ? value : []),
			]);
		} else if (action === "remove") {
			const lastKey = keyParts[keyParts.length - 1];
			prevKeyConsumer.delete(lastKey);
		} else if (action === "pop") {
			if (keyConsumer.exists()) {
				const existingValues = Array.from(
					keyConsumer.asIterable(),
					(c) => c.asUnknown(),
				);
				const valuesToRemove = Array.isArray(value) ? value : [value];

				keyConsumer.setValue(
					existingValues.filter((item) =>
						!valuesToRemove.includes(item as string | boolean)
					),
				);
			}
		} else {
			const currentValue = keyConsumer.asUnknown();
			if (typeof currentValue === "number") {
				keyConsumer.setValue(Number(value));
			} else {
				keyConsumer.setValue(value);
			}
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
		const finalKeyPath =
			action === "disable" || action === "enable"
				? `${keyPath}.enabled`
				: keyPath;

		switch (action) {
			case "remove": {
				reporter.success(
					markup`Removing <emphasis>${keyPath}</emphasis> in the config <emphasis>${configPath}</emphasis>`,
				);
				break;
			}
			case "pop": {
				reporter.success(
					markup`Removing <emphasis>${prettyFormat(value)}</emphasis> from <emphasis>${keyPath}</emphasis> in the config <emphasis>${configPath}</emphasis>`,
				);
				break;
			}
			default:
				reporter.success(
					markup`${action === "push" ? "Adding" : "Setting"} <emphasis>${prettyFormat(
						value,
					)}</emphasis> to <emphasis>${keyPath}</emphasis> in the config <emphasis>${configPath}</emphasis>`,
				);
		}

		if (value === "true" || value === "false") {
			const suggestedCommand = value === "true" ? "enable" : "disable";
			reporter.warn(
				markup`Value is the string <emphasis>${value}</emphasis> but it looks like a boolean. You probably meant to use the command:`,
			);
			reporter.command(`config ${suggestedCommand} ${finalKeyPath}`);
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
			markup`Error occurred while validating new config. Your changes have not been saved. Listed locations are not accurate.`,
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

export const migrate = createServerCommand({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`Migrates Rome's old configuration file to use its latest version`,
	usage: "migrate",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req) {
		req.expectArgumentLength(0, 0);
		const {client} = req;
		await checkVSCWorkingDirectory(
			req,
			[
				descriptions.MIGRATE_COMMAND.EXPECT_REPO.advice,
				descriptions.MIGRATE_COMMAND.UNCOMMITTED_CHANGES.advice,
			],
		);
		const result = await req.retrieveProjectAndConfigPaths();
		if (!result) {
			throw createSingleDiagnosticsError({
				location: req.getDiagnosticLocationForClientCwd(),
				description: descriptions.MIGRATE_COMMAND.MISSING_CONFIGURATION,
			});
		}

		const {configPath} = result;
		const migrator = new Migrator({
			reporter: client.reporter,
			version: VERSION,
		});

		const configFile = await configPath.readFileText();

		let finalConsumer: Consumer;
		let {consumer, type, comments} = consumeConfig({
			path: configPath,
			input: configFile,
		});

		const isInPackageJson = configPath.getBasename() === "package.json";
		if (isInPackageJson) {
			finalConsumer = consumer.get(PROJECT_CONFIG_PACKAGE_JSON_FIELD);
		} else {
			finalConsumer = consumer;
		}

		await migrator.run(finalConsumer);

		// Stringify the config
		const stringified = stringifyConfig({
			consumer,
			comments,
			type,
		});

		await configPath.writeFile(stringified);
	},
});

export const remove = createServerCommand<Flags>({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`modify a project config - remove "<key>"`,
	usage: "<key>",
	examples: [],
	hidden: true,
	defineFlags,
	async callback(req, flags) {
		req.expectArgumentLength(1);
		await runCommand(req, flags, req.query.args[1], "remove");
	},
});

export const pop = createServerCommand<Flags>({
	category: commandCategories.PROJECT_MANAGEMENT,
	description: markup`modify a project config - remove "<values>" from "<key>"`,
	usage: "<key> <...values>",
	examples: [],
	hidden: true,
	defineFlags,
	async callback(req, flags) {
		req.expectArgumentLength(2, Infinity);
		await runCommand(req, flags, req.query.args.slice(1), "pop");
	},
});

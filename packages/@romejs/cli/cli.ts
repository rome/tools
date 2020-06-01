/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Client,
	ClientFlags,
	ClientRequestFlags,
	DEFAULT_CLIENT_FLAGS,
	DEFAULT_CLIENT_REQUEST_FLAGS,
	PLATFORMS,
	VERSION,
	localCommands,
	masterCommands,
} from "@romejs/core";
import setProcessTitle from "./utils/setProcessTitle";
import {parseCLIFlagsFromProcess} from "@romejs/cli-flags";
import {UnknownFilePath, createAbsoluteFilePath} from "@romejs/path";
import {Consumer} from "@romejs/consume";
import {
	ClientProfileOptions,
	getFilenameTimestamp,
} from "@romejs/core/client/Client";
import {commandCategories} from "@romejs/core/common/commands";
import {writeFile} from "@romejs/fs";
import fs = require("fs");
import {markup} from "@romejs/string-markup";
import {JSONObject, stringifyJSON} from "@romejs/codec-json";

type CLIFlags = {
	logs: boolean;
	logWorkers: undefined | boolean;
	logPath: undefined | UnknownFilePath;
	markersPath: undefined | UnknownFilePath;
	rage: boolean;
	ragePath: undefined | UnknownFilePath;
	profile: boolean;
	profilePath: undefined | UnknownFilePath;
	profileTimeout: undefined | number;
	profileSampling: number;
	profileWorkers: boolean;
	temporaryDaemon: boolean;
};

export default async function cli() {
	setProcessTitle("cli");
	const p = parseCLIFlagsFromProcess({
		programName: process.env.ROME_DEV === "1" ? "dev-rome" : "rome",
		usage: "[command] [flags]",
		version: VERSION,
		defineFlags(
			c: Consumer,
		): {
			cliFlags: CLIFlags;
			clientFlags: ClientFlags;
			requestFlags: ClientRequestFlags;
		} {
			// We need this to resolve other flags relative to
			// We do the word `void ||` nonsense to avoid setting a default flag value
			const cwd =
				c.get(
					"cwd",
					{
						description: "Specify a different working directory",
					},
				).asAbsoluteFilePathOrVoid() || createAbsoluteFilePath(process.cwd());

			return {
				clientFlags: {
					clientName: "cli",
					cwd,
					verbose: c.get(
						"verbose",
						{
							description: "Output verbose logs",
						},
					).asBoolean(DEFAULT_CLIENT_FLAGS.verbose),
					silent: c.get(
						"silent",
						{
							description: "Don't write anything to the console",
						},
					).asBoolean(DEFAULT_CLIENT_FLAGS.silent),
					...overrideClientFlags,
				},
				cliFlags: {
					markersPath: c.get(
						"markersPath",
						{
							description: "Path where to write markers. When ommitted defaults to Marker-TIMESTAMP.json",
						},
					).asAbsoluteFilePathOrVoid(undefined, cwd),
					profile: c.get(
						"profile",
						{
							description: "Collect and write profile to disk. Includes profiles for all processes.",
						},
					).asBoolean(false),
					profilePath: c.get(
						"profilePath",
						{
							description: "Path where to write profile. When omitted defaults to Profile-TIMESTAMP.json",
						},
					).asAbsoluteFilePathOrVoid(undefined, cwd),
					profileTimeout: c.get(
						"profileTimeout",
						{
							inputName: "millisec",
							description: "Stop the profile after the milliseconds specified. When omitted the profile is of the whole command",
						},
					).asNumberOrVoid(),
					profileWorkers: c.get(
						"profileWorkers",
						{
							description: "Exclude workers from profile",
						},
					).asBoolean(true),
					profileSampling: c.get(
						"profileSampling",
						{
							description: "Profiler sampling interval in microseconds",
							inputName: "microsec",
						},
					).asNumber(100),
					temporaryDaemon: c.get(
						"temporaryDaemon",
						{
							description: "Start a daemon, if one isn't already running, for the lifetime of this command",
						},
					).asBoolean(false),
					rage: c.get(
						"rage",
						{
							description: "Create a rage tarball of debug information",
						},
					).asBoolean(false),
					ragePath: c.get(
						"ragePath",
						{
							description: "Path where to write rage tarball. When omitted defaults to Rage-TIMESTAMP.tgz",
						},
					).asAbsoluteFilePathOrVoid(undefined, cwd),
					logs: c.get(
						"logs",
						{
							description: "Output master logs",
						},
					).asBoolean(false),
					logWorkers: c.get(
						"logWorkers",
						{
							description: "Output worker logs",
						},
					).asBooleanOrVoid(),
					logPath: c.get(
						"logPath",
						{
							description: "Path where to output logs. When omitted logs are not written anywhere",
						},
					).asAbsoluteFilePathOrVoid(undefined, cwd),
					...overrideCLIFlags,
				},
				requestFlags: {
					benchmark: c.get(
						"benchmark",
						{
							description: "Run a command multiple times, calculating average",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.benchmark),
					benchmarkIterations: c.get(
						"benchmarkIterations",
						{
							description: "The amount of benchmark iterations to perform",
						},
					).asNumber(DEFAULT_CLIENT_REQUEST_FLAGS.benchmarkIterations),
					collectMarkers: c.get(
						"collectMarkers",
						{
							description: "Collect and write performance markers to disk",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.collectMarkers),
					timing: c.get(
						"timing",
						{
							description: "Dump timing information after running the command",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.timing),
					review: c.get(
						"review",
						{
							description: "Display and perform actions on diagnostics. Only some commands support this.",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.review),
					watch: c.get(
						"watch",
						{
							description: "Keep running command and update on file changes. Only some commands support this.",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.watch),
					fieri: c.get(
						"fieri",
						{
							description: "Head to flavortown",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.fieri),
					grep: c.get(
						"grep",
						{
							description: "Only display diagnostics with messages containing this string",
						},
					).asString(DEFAULT_CLIENT_REQUEST_FLAGS.grep),
					inverseGrep: c.get(
						"inverseGrep",
						{
							description: "Flip grep match. Only display diagnostics with messages that do NOT contain the grep string",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.inverseGrep),
					maxDiagnostics: c.get(
						"maxDiagnostics",
						{
							description: "Cap the amount of diagnostics displayed",
						},
					).asNumber(DEFAULT_CLIENT_REQUEST_FLAGS.maxDiagnostics),
					verboseDiagnostics: c.get(
						"verboseDiagnostics",
						{
							description: "Display hidden and truncated diagnostic information",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.verboseDiagnostics),
					showAllDiagnostics: c.get(
						"showAllDiagnostics",
						{
							description: "Display all diagnostics ignoring caps",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.showAllDiagnostics),
					resolverPlatform: c.get(
						"resolverPlatform",
						{
							description: "Specify the platform for module resolution",
							inputName: "platform",
						},
					).asStringSetOrVoid(PLATFORMS),
					resolverScale: c.get(
						"resolverScale",
						{
							description: "Specify the image scale for module resolution",
						},
					).asNumberOrVoid(),
					resolverMocks: c.get(
						"resolverMocks",
						{
							description: "Enable mocks for module resolution",
						},
					).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.resolverMocks),
					...overrideRequestFlags,
				},
			};
		},
	});

	let command = "";
	let overrideClientFlags: undefined | Partial<ClientFlags>;
	let overrideRequestFlags: undefined | Partial<ClientRequestFlags>;
	let overrideCLIFlags: Partial<CLIFlags> = {};
	let commandFlags: JSONObject = {};
	let args: Array<string> = [];

	// Create command handlers. We use a set here since we may have some conflicting master and local command names. We always want the local command to take precedence.
	const commandNames = new Set([
		...localCommands.keys(),
		...masterCommands.keys(),
	]);
	for (const cmd of commandNames) {
		const local = localCommands.get(cmd);
		if (local !== undefined) {
			p.command({
				name: cmd,
				category: local.category,
				description: local.description,
				defineFlags: local.defineFlags,
				ignoreFlags: local.ignoreFlags,
				examples: local.examples,
				usage: local.usage,
				callback(_commandFlags) {
					commandFlags = _commandFlags;
					args = p.getArgs();
					command = cmd;
				},
			});
			continue;
		}

		const master = masterCommands.get(cmd);
		if (master !== undefined) {
			p.command({
				name: cmd,
				category: master.category,
				description: master.description,
				defineFlags: master.defineFlags,
				ignoreFlags: master.ignoreFlags,
				usage: master.usage,
				examples: master.examples,
				callback(_commandFlags) {
					commandFlags = _commandFlags;
					overrideClientFlags = master.overrideClientFlags;
					overrideRequestFlags = master.overrideRequestFlags;

					args = p.getArgs();
					command = cmd;
				},
			});
		}
	}

	// Mock `rage` command that just uses the master noop command and adds the --rage flag
	p.command({
		name: "rage",
		category: commandCategories.INTERNAL,
		description: "TODO",
		callback() {
			overrideCLIFlags = {
				rage: true,
			};

			command = "_noop";
		},
	});

	// Mock `logs` command that just uses the master noop command and adds the --logs flag
	p.command({
		name: "logs",
		category: commandCategories.INTERNAL,
		description: "TODO",
		callback() {
			overrideCLIFlags = {
				logs: true,
			};

			command = "_noop";
		},
	});

	// Initialize flags
	let {clientFlags, cliFlags, requestFlags} = await p.init();

	// Force collection of markers if markersPath or we are raging
	if (cliFlags.markersPath || cliFlags.rage) {
		requestFlags.collectMarkers = true;
	}

	// Force logs when logPath or logWorkers is set
	if (cliFlags.logPath !== undefined || cliFlags.logWorkers === true) {
		cliFlags.logs = true;
	}

	p.commandRequired();

	const client = new Client({
		globalErrorHandlers: true,
		flags: clientFlags,
		stdin: process.stdin,
		stdout: process.stdout,
		stderr: process.stderr,
	});

	client.bridgeAttachedEvent.subscribe(async () => {
		const profileOptions: ClientProfileOptions = {
			samplingInterval: cliFlags.profileSampling,
			timeoutInterval: cliFlags.profileTimeout,
			includeWorkers: cliFlags.profileWorkers,
		};

		if (cliFlags.rage) {
			const {ragePath} = cliFlags;
			const filename = clientFlags.cwd.resolve(
				ragePath === undefined ? `Rage-${getFilenameTimestamp()}.tgz` : ragePath,
			).join();
			await client.rage(filename, profileOptions);
			return;
		}

		if (cliFlags.profile) {
			await client.profile(
				profileOptions,
				async (events) => {
					const {cwd} = clientFlags;
					const {profilePath} = cliFlags;

					const resolvedProfilePath = cwd.resolve(
						profilePath === undefined
							? `Profile-${getFilenameTimestamp()}.json`
							: profilePath,
					);

					const str = stringifyJSON(events);
					await writeFile(resolvedProfilePath, str);

					client.reporter.success(
						markup`Wrote CPU profile to ${resolvedProfilePath.toMarkup({
							emphasis: true,
						})}`,
					);
				},
			);
		}

		if (cliFlags.logs) {
			let fileout: undefined | fs.WriteStream;
			if (cliFlags.logPath !== undefined) {
				fileout = fs.createWriteStream(
					clientFlags.cwd.resolve(cliFlags.logPath).join(),
				);

				client.endEvent.subscribe(() => {
					if (fileout !== undefined) {
						fileout.end();
					}
				});
			}

			await client.subscribeLogs(
				cliFlags.logWorkers === true,
				(chunk) => {
					if (fileout === undefined) {
						client.reporter.writeAll(chunk);
					} else {
						fileout.write(chunk);
					}
				},
			);
		}
	});

	if (cliFlags.temporaryDaemon) {
		await client.forceStartDaemon();
	}

	const res = await client.query({
		commandName: command,
		commandFlags,
		args,
		requestFlags,
		// Daemon would have been started before, so terminate when we complete
		terminateWhenIdle: cliFlags.temporaryDaemon,
		// We don't use the data result, so no point transporting it over the bridge
		noData: true,
	});
	await client.end();

	if (res.type === "SUCCESS") {
		// Write markers if we were collecting them
		if (requestFlags.collectMarkers) {
			const markersPath = clientFlags.cwd.resolve(
				cliFlags.markersPath === undefined
					? `Markers-${getFilenameTimestamp()}.json`
					: cliFlags.markersPath,
			);

			await writeFile(markersPath, stringifyJSON(res.markers));

			client.reporter.success(
				markup`Wrote markers to ${markersPath.toMarkup({emphasis: true})}`,
			);
		}
	}

	switch (res.type) {
		case "ERROR": {
			if (!res.handled) {
				console.error("Unhandled CLI query error");
				console.error(res.stack);
			}
			process.exit(1);
			break;
		}

		case "INVALID_REQUEST": {
			if (res.showHelp) {
				await p.showHelp();
			}
			process.exit(1);
			break;
		}

		case "DIAGNOSTICS": {
			process.exit(res.hasDiagnostics ? 1 : 0);
			break;
		}

		case "CANCELLED": {
			process.exit(0);
			break;
		}

		case "SUCCESS": {
			process.exit(0);
			break;
		}
	}
}

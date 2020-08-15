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
	ClientTerminalFeatures,
	DEFAULT_CLIENT_FLAGS,
	DEFAULT_CLIENT_REQUEST_FLAGS,
	PLATFORMS,
	REQUIRED_NODE_VERSION_RANGE,
	USER_CONFIG_DIRECTORY,
	VERSION,
	localCommands,
	serverCommands,
} from "@internal/core";
import setProcessTitle from "./utils/setProcessTitle";
import {parseCLIFlagsFromProcess} from "@internal/cli-flags";
import {AbsoluteFilePath, CWD_PATH} from "@internal/path";
import {Consumer} from "@internal/consume";
import {
	ClientProfileOptions,
	getFilenameTimestamp,
} from "@internal/core/client/Client";
import {commandCategories} from "@internal/core/common/commands";
import {WriteStream, createWriteStream, writeFile} from "@internal/fs";
import {markupToPlainText} from "@internal/cli-layout";
import {
	convertToMarkupFromRandomString,
	joinMarkupLines,
	markup,
} from "@internal/markup";
import {stringifyJSON} from "@internal/codec-json";
import {getEnvVar} from "@internal/cli-environment";
import {satisfiesSemver} from "@internal/codec-semver";
import {Reporter} from "@internal/cli-reporter";
import {loadUserConfig} from "@internal/core/common/userConfig";
import {RSERObject} from "@internal/codec-binary-serial";

type CLIFlags = {
	logs: boolean;
	logWorkers: undefined | boolean;
	logPath: undefined | AbsoluteFilePath;
	markersPath: undefined | AbsoluteFilePath;
	rage: boolean;
	ragePath: undefined | AbsoluteFilePath;
	profile: boolean;
	profilePath: undefined | AbsoluteFilePath;
	profileTimeout: undefined | number;
	profileSampling: number;
	profileWorkers: boolean;
	temporaryDaemon: boolean;
};

export default async function cli() {
	setProcessTitle("cli");

	// Verify correct Node version
	if (
		!satisfiesSemver(
			process.version,
			REQUIRED_NODE_VERSION_RANGE,
			{loose: true},
		)
	) {
		const reporter = Reporter.fromProcess();
		reporter.error(
			markup`Node <emphasis>${REQUIRED_NODE_VERSION_RANGE}</emphasis> is required, but you are on <emphasis>${process.version}</emphasis>`,
		);
		process.exit(1);
	}

	const p = parseCLIFlagsFromProcess({
		programName: getEnvVar("ROME_DEV").type === "ENABLED" ? "dev-rome" : "rome",
		usage: "[command] [flags]",
		version: VERSION,
		commandRequired: true,
		shellCompletionDirectory: USER_CONFIG_DIRECTORY,
		commandSuggestions: {
			lint: {
				commandName: "check",
				description: markup`The <emphasis>check</emphasis> command covers linting, formatting, and more`,
			},
		},
		defineFlags(
			c: Consumer,
		): {
			terminalFeatures: ClientTerminalFeatures;
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
						description: markup`Specify a different working directory`,
					},
				).asAbsoluteFilePathOrVoid() || CWD_PATH;

			const terminalFeatures: ClientTerminalFeatures = {
				format: c.get(
					"outputFormat",
					{
						description: markup`Change the output format. By default it is automatically inferred from terminal settings.`,
					},
				).asStringSetOrVoid(["ansi", "html", "none"]),
				isTTY: c.get(
					"outputTty",
					{
						description: markup`Treat output as TTY regardless of terminal information. This will enable things like ANSI cursor, progress bars etc.`,
					},
				).asBooleanOrVoid(),
				columns: c.get(
					"outputColumns",
					{
						description: markup`Change the display width. By default it is automatically inferred and updated from the terminal.`,
					},
				).asOneIndexedNumberOrVoid(),
				colorDepth: c.get(
					"outputColorDepth",
					{
						description: markup`Change the display width. By default it is automatically inferred and updated from the terminal.`,
					},
				).asNumberSetOrVoid([1, 4, 8, 24]),
				redirectError: c.get(
					"outputRedirectError",
					{
						description: markup`Redirect stderr to stdout.`,
					},
				).asBooleanOrVoid(),
			};

			const clientFlags: ClientFlags = {
				realCwd: CWD_PATH,
				clientName: "cli",
				cwd,
				silent: c.get(
					"silent",
					{
						description: markup`Don't write anything to the console`,
					},
				).asBoolean(DEFAULT_CLIENT_FLAGS.silent),
			};

			const cliFlags: CLIFlags = {
				markersPath: c.get(
					"markersPath",
					{
						description: markup`Path where to write markers. When ommitted defaults to Marker-TIMESTAMP.json`,
					},
				).asAbsoluteFilePathOrVoid(cwd),
				profile: c.get(
					"profile",
					{
						description: markup`Collect and write profile to disk. Includes profiles for all processes.`,
					},
				).asBoolean(false),
				profilePath: c.get(
					"profilePath",
					{
						description: markup`Path where to write profile. When omitted defaults to Profile-TIMESTAMP.json`,
					},
				).asAbsoluteFilePathOrVoid(cwd),
				profileTimeout: c.get(
					"profileTimeout",
					{
						inputName: "millisec",
						description: markup`Stop the profile after the milliseconds specified. When omitted the profile is of the whole command`,
					},
				).asNumberOrVoid(),
				profileWorkers: c.get(
					"profileWorkers",
					{
						description: markup`Exclude workers from profile`,
					},
				).asBoolean(true),
				profileSampling: c.get(
					"profileSampling",
					{
						description: markup`Profiler sampling interval in microseconds`,
						inputName: "microsec",
					},
				).asNumber(100),
				temporaryDaemon: c.get(
					"temporaryDaemon",
					{
						description: markup`Start a daemon, if one isn't already running, for the lifetime of this command`,
					},
				).asBoolean(false),
				rage: c.get(
					"rage",
					{
						description: markup`Create a rage tarball of debug information`,
					},
				).asBoolean(false),
				ragePath: c.get(
					"ragePath",
					{
						description: markup`Path where to write rage tarball. When omitted defaults to Rage-TIMESTAMP.tgz`,
					},
				).asAbsoluteFilePathOrVoid(cwd),
				logs: c.get(
					"logs",
					{
						description: markup`Output server logs`,
					},
				).asBoolean(false),
				logWorkers: c.get(
					"logWorkers",
					{
						description: markup`Output worker logs`,
					},
				).asBooleanOrVoid(),
				logPath: c.get(
					"logPath",
					{
						description: markup`Path where to output logs. When omitted logs are not written anywhere`,
					},
				).asAbsoluteFilePathOrVoid(cwd),
			};

			const requestFlags: ClientRequestFlags = {
				unsafeWrites: c.get(
					"unsafeWrites",
					{
						description: markup`When writing files, don't verify mtime or existence. Potentially dangerous and could lead to unintended data loss`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.unsafeWrites),
				auxiliaryDiagnosticFormat: c.get(
					"auxiliaryDiagnosticFormat",
					{
						description: markup`When printing diagnostics, output another format alongside`,
					},
				).asStringSetOrVoid(["github-actions"]),
				benchmark: c.get(
					"benchmark",
					{
						description: markup`Run a command multiple times, calculating average`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.benchmark),
				benchmarkIterations: c.get(
					"benchmarkIterations",
					{
						description: markup`The amount of benchmark iterations to perform`,
					},
				).asNumber(DEFAULT_CLIENT_REQUEST_FLAGS.benchmarkIterations),
				collectMarkers: c.get(
					"collectMarkers",
					{
						description: markup`Collect and write performance markers to disk`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.collectMarkers),
				timing: c.get(
					"timing",
					{
						description: markup`Dump timing information after running the command`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.timing),
				review: c.get(
					"review",
					{
						description: markup`Display and perform actions on diagnostics. Only some commands support this.`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.review),
				watch: c.get(
					"watch",
					{
						description: markup`Keep running command and update on file changes. Only some commands support this.`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.watch),
				fieri: c.get(
					"fieri",
					{
						description: markup`Head to flavortown`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.fieri),
				grep: c.get(
					"grep",
					{
						description: markup`Only display diagnostics with messages containing this string`,
					},
				).asString(DEFAULT_CLIENT_REQUEST_FLAGS.grep),
				inverseGrep: c.get(
					"inverseGrep",
					{
						description: markup`Flip grep match. Only display diagnostics with messages that do NOT contain the grep string`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.inverseGrep),
				maxDiagnostics: c.get(
					"maxDiagnostics",
					{
						description: markup`Cap the amount of diagnostics displayed`,
					},
				).asNumber(DEFAULT_CLIENT_REQUEST_FLAGS.maxDiagnostics),
				// DiagnosticsPrinterFlags["verboseDiagnostics"] can be a boolean or some string constants
				// But we only want to allow booleans in the CLI
				verboseDiagnostics: c.get(
					"verboseDiagnostics",
					{
						description: markup`Display hidden and truncated diagnostic information`,
					},
				).asBoolean(false),
				showAllDiagnostics: c.get(
					"showAllDiagnostics",
					{
						description: markup`Display all diagnostics ignoring caps`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.showAllDiagnostics),
				resolverPlatform: c.get(
					"resolverPlatform",
					{
						description: markup`Specify the platform for module resolution`,
						inputName: "platform",
					},
				).asStringSetOrVoid(PLATFORMS),
				resolverScale: c.get(
					"resolverScale",
					{
						description: markup`Specify the image scale for module resolution`,
					},
				).asNumberOrVoid(),
				resolverMocks: c.get(
					"resolverMocks",
					{
						description: markup`Enable mocks for module resolution`,
					},
				).asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.resolverMocks),
			};

			return {terminalFeatures, clientFlags, requestFlags, cliFlags};
		},
	});

	let command = "";
	let overrideCLIFlags: Partial<CLIFlags> = {};
	let commandFlags: RSERObject = {};
	let args: Array<string> = [];

	const isRelease = getEnvVar("ROME_DEV").type !== "ENABLED";

	// Create command handlers. We use a set here since we may have some conflicting server and local command names. We always want the local command to take precedence.
	const commandNames = new Set([
		...localCommands.keys(),
		...serverCommands.keys(),
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
				hidden: isRelease && local.hidden,
				callback(_commandFlags) {
					commandFlags = _commandFlags;
					args = p.getArgs();
					command = cmd;
				},
			});
			continue;
		}

		const server = serverCommands.get(cmd);
		if (server !== undefined) {
			p.command({
				name: cmd,
				category: server.category,
				description: server.description,
				defineFlags: server.defineFlags,
				ignoreFlags: server.ignoreFlags,
				usage: server.usage,
				examples: server.examples,
				hidden: isRelease && server.hidden,
				callback(_commandFlags) {
					commandFlags = _commandFlags;
					args = p.getArgs();
					command = cmd;
				},
			});
		}
	}

	// Mock `rage` command that just uses the server noop command and adds the --rage flag
	p.command({
		name: "rage",
		category: commandCategories.INTERNAL,
		description: markup`create a rage archive for debugging`,
		defineFlags(c) {
			return {
				summary: c.get("summary").asBoolean(false),
			};
		},
		callback(_commandFlags) {
			commandFlags = _commandFlags;

			overrideCLIFlags = {
				rage: true,
			};

			command = "noop";
		},
	});

	// Mock `logs` command that just uses the server noop command and adds the --logs flag
	p.command({
		name: "logs",
		category: commandCategories.INTERNAL,
		description: markup`view the logs stream`,
		callback() {
			overrideCLIFlags = {
				logs: true,
			};

			commandFlags = {
				hang: true,
			};

			command = "noop";
		},
	});

	// Initialize flags
	let {terminalFeatures, clientFlags, cliFlags, requestFlags} = await p.init();

	// We force some cli flags to be set for certain commands
	cliFlags = {
		...cliFlags,
		...overrideCLIFlags,
	};

	// Default according to env vars
	if (requestFlags.auxiliaryDiagnosticFormat === undefined) {
		if (getEnvVar("GITHUB_ACTIONS").type === "ENABLED") {
			requestFlags.auxiliaryDiagnosticFormat = "github-actions";
		}
	}

	// Force collection of markers if markersPath or we are raging
	// Save the real value here since if rage is set we don't want to save them
	const shouldCollectMarkers =
		requestFlags.collectMarkers || cliFlags.markersPath !== undefined;
	if (cliFlags.markersPath || cliFlags.rage) {
		requestFlags.collectMarkers = true;
	}

	// Force logs when logPath or logWorkers is set
	if (cliFlags.logPath !== undefined || cliFlags.logWorkers === true) {
		cliFlags.logs = true;
	}

	const userConfig = await loadUserConfig();
	const client = new Client({
		userConfig,
		terminalFeatures,
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
			if (commandFlags.summary === true) {
				client.reporter.log(await client.generateRageSummary());
			} else {
				let {ragePath} = cliFlags;

				// Resolve or add default filename
				ragePath = clientFlags.cwd.resolve(
					ragePath === undefined
						? `Rage-${getFilenameTimestamp()}.tgz`
						: ragePath,
				);
				await client.rage(ragePath, profileOptions);
			}
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
						markup`Wrote CPU profile to <emphasis>${resolvedProfilePath}</emphasis>`,
					);
				},
			);
		}

		if (cliFlags.logs) {
			let fileout: undefined | WriteStream;
			if (cliFlags.logPath !== undefined) {
				fileout = createWriteStream(clientFlags.cwd.resolve(cliFlags.logPath));

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
						client.reporter.log(
							convertToMarkupFromRandomString(chunk),
							{noNewline: true},
						);
					} else {
						fileout.write(
							joinMarkupLines(
								markupToPlainText(convertToMarkupFromRandomString(chunk)),
							),
						);
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
		// We want it in rage mode though for debugging
		noData: !cliFlags.rage,
	});
	await client.end();

	// Write markers if we were collecting them
	if (shouldCollectMarkers && res.markers.length > 0) {
		const markersPath = clientFlags.cwd.resolve(
			cliFlags.markersPath === undefined
				? `Markers-${getFilenameTimestamp()}.json`
				: cliFlags.markersPath,
		);

		await writeFile(markersPath, stringifyJSON(res.markers));

		client.reporter.success(
			markup`Wrote markers to <emphasis>${markersPath}</emphasis>`,
		);
	}

	switch (res.type) {
		case "EXIT": {
			process.exit(res.code);
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
			client.reporter.error(markup`Command cancelled: ${res.reason}`);
			process.exit(0);
			break;
		}

		case "SUCCESS": {
			process.exit(0);
			break;
		}
	}
}

/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Client,
	ClientFlags,
	ClientLogsLevel,
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
import {CommandName, commandCategories} from "@internal/core/common/commands";
import {FSWriteStream} from "@internal/fs";
import {markupToPlainText} from "@internal/cli-layout";
import {
	convertToMarkupFromRandomString,
	joinMarkupLines,
	markup,
} from "@internal/markup";
import {json} from "@internal/codec-config";
import {IS_ROME_DEV_ENV, isEnvVarSet} from "@internal/cli-environment";
import {
	parseSemverRange,
	parseSemverVersion,
	satisfiesSemver,
} from "@internal/codec-semver";
import {Reporter} from "@internal/cli-reporter";
import {loadUserConfig} from "@internal/core/common/userConfig";
import {RSERObject} from "@internal/binary-transport";
import {safeProcessExit} from "@internal/resources";

type CLIFlags = {
	logs: undefined | ClientLogsLevel;
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
	showAllDiagnostics: boolean;
};

export default async function cli() {
	setProcessTitle("cli");

	// Verify correct Node version
	if (
		!satisfiesSemver(
			parseSemverVersion({input: process.version, loose: true}),
			parseSemverRange({input: REQUIRED_NODE_VERSION_RANGE}),
		)
	) {
		const reporter = Reporter.fromProcess();
		reporter.error(
			markup`Node <emphasis>${REQUIRED_NODE_VERSION_RANGE}</emphasis> is required, but you are on <emphasis>${process.version}</emphasis>`,
		);
		await safeProcessExit(1);
	}

	const p = parseCLIFlagsFromProcess({
		programName: IS_ROME_DEV_ENV ? "dev-rome" : "rome",
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
		onRunHiddenCommand(reporter) {
			if (IS_ROME_DEV_ENV || isEnvVarSet("ROME_DEV_VENDOR_BUNDLING")) {
				return;
			}

			reporter.warn(
				markup`This command has been hidden. Consider its usage to be experimental and do not expect support or backwards compatibility.`,
			);
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
				).required(DEFAULT_CLIENT_FLAGS.silent).asBoolean(),
			};

			const cliFlags: CLIFlags = {
				showAllDiagnostics: c.get(
					"showAllDiagnostics",
					{
						description: markup`Display all diagnostics ignoring caps`,
					},
				).required(false).asBoolean(),
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
				).required(false).asBoolean(),
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
				).required(true).asBoolean(),
				profileSampling: c.get(
					"profileSampling",
					{
						description: markup`Profiler sampling interval in microseconds`,
						inputName: "microsec",
					},
				).required(100).asNumber(),
				temporaryDaemon: c.get(
					"temporaryDaemon",
					{
						description: markup`Start a daemon, if one isn't already running, for the lifetime of this command`,
					},
				).required(false).asBoolean(),
				rage: c.get(
					"rage",
					{
						description: markup`Create a rage tarball of debug information`,
					},
				).required(false).asBoolean(),
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
				).asStringSetOrVoid(["all", "error"]),
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
				programmatic: false,
				unsafeWrites: c.get(
					"unsafeWrites",
					{
						description: markup`When writing files, don't verify mtime or existence. Potentially dangerous and could lead to unintended data loss`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.unsafeWrites).asBoolean(),
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
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.benchmark).asBoolean(),
				benchmarkIterations: c.get(
					"benchmarkIterations",
					{
						description: markup`The amount of benchmark iterations to perform`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.benchmarkIterations).asNumber(),
				collectMarkers: c.get(
					"collectMarkers",
					{
						description: markup`Collect and write performance markers to disk`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.collectMarkers).asBoolean(),
				timing: c.get(
					"timing",
					{
						description: markup`Dump timing information after running the command`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.timing).asBoolean(),
				review: c.get(
					"review",
					{
						description: markup`Display and perform actions on diagnostics. Only some commands support this.`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.review).asBoolean(),
				watch: c.get(
					"watch",
					{
						description: markup`Keep running command and update on file changes. Only some commands support this.`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.watch).asBoolean(),
				fieri: c.get(
					"fieri",
					{
						description: markup`Head to flavortown`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.fieri).asBoolean(),
				grep: c.get(
					"grep",
					{
						description: markup`Only display diagnostics with messages containing this string`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.grep).asString(),
				inverseGrep: c.get(
					"inverseGrep",
					{
						description: markup`Flip grep match. Only display diagnostics with messages that do NOT contain the grep string`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.inverseGrep).asBoolean(),
				maxDiagnostics: c.get(
					"maxDiagnostics",
					{
						description: markup`Cap the amount of diagnostics displayed`,
					},
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.maxDiagnostics).asNumber(),
				verboseDiagnostics: c.get(
					"verboseDiagnostics",
					{
						description: markup`Display additional hidden diagnostic information`,
					},
				).required(false).asBoolean(),
				truncateDiagnostics: c.get(
					"truncateDiagnostics",
					{
						description: markup`Display truncated diagnostic information`,
					},
				).required(true).asBoolean(),
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
				).required(DEFAULT_CLIENT_REQUEST_FLAGS.resolverMocks).asBoolean(),
			};

			return {terminalFeatures, clientFlags, requestFlags, cliFlags};
		},
	});

	let command: CommandName = "noop";
	let overrideCLIFlags: Partial<CLIFlags> = {};
	let commandFlags: RSERObject = {};
	let args: string[] = [];

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
				hidden: local.hidden,
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
				hidden: server.hidden,
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
				summary: c.get("summary").required(false).asBoolean(),
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
				logs: "all",
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

	// --show-all-diagnostics is just a shorthand for setting this to Infinity as there's no way to specify that via CLI args
	if (cliFlags.showAllDiagnostics) {
		requestFlags.maxDiagnostics = Infinity;
	}

	// Default according to env vars
	if (requestFlags.auxiliaryDiagnosticFormat === undefined) {
		if (isEnvVarSet("GITHUB_ACTIONS")) {
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
	if (
		cliFlags.logs === undefined &&
		(cliFlags.logPath !== undefined || cliFlags.logWorkers === true)
	) {
		cliFlags.logs = "error";
	}

	const userConfig = await loadUserConfig();
	const client = new Client({
		userConfig,
		terminalFeatures,
		dedicated: true,
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

					const str = json.stringify(events);
					await resolvedProfilePath.writeFile(str);

					client.reporter.success(
						markup`Wrote CPU profile to <emphasis>${resolvedProfilePath}</emphasis>`,
					);
				},
			);
		}

		if (cliFlags.logs) {
			let fileout: undefined | FSWriteStream;
			if (cliFlags.logPath !== undefined) {
				fileout = clientFlags.cwd.resolve(cliFlags.logPath).createWriteStream();

				client.endEvent.subscribe(() => {
					if (fileout !== undefined) {
						fileout.end();
					}
				});
			}

			await client.subscribeLogs(
				cliFlags.logs,
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
		// We don't use the data result, so no point transporting it over the bridge
		// We want it in rage mode though for debugging
		noData: !cliFlags.rage,
	});

	if (cliFlags.temporaryDaemon) {
		await client.shutdownServer();
	} else {
		await client.end();
	}

	// Write markers if we were collecting them
	if (shouldCollectMarkers && res.markers.length > 0) {
		const markersPath = clientFlags.cwd.resolve(
			cliFlags.markersPath === undefined
				? `Markers-${getFilenameTimestamp()}.json`
				: cliFlags.markersPath,
		);

		await markersPath.writeFile(json.stringify(res.markers));

		client.reporter.success(
			markup`Wrote markers to <emphasis>${markersPath}</emphasis>`,
		);
	}

	let exitCode;
	switch (res.type) {
		case "EXIT": {
			exitCode = res.code;
			break;
		}

		case "CLIENT_ERROR": {
			exitCode = 1;
			break;
		}

		case "INVALID_REQUEST": {
			if (res.showHelp) {
				await p.showHelp();
			}
			exitCode = 1;
			break;
		}

		case "DIAGNOSTICS": {
			exitCode = res.hasDiagnostics ? 1 : 0;
			break;
		}

		case "CANCELLED": {
			client.reporter.error(markup`Command cancelled: ${res.reason}`);
			exitCode = 0;
			break;
		}

		case "SUCCESS": {
			exitCode = 0;
			break;
		}
	}
	await safeProcessExit(exitCode);
}

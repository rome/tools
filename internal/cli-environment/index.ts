import stream = require("stream");
import tty = require("tty");
import {Event} from "@internal/events";
import {mergeObjects} from "@internal/typescript-helpers";
import {OneIndexed} from "@internal/numbers";
import {
	Resource,
	createResourceContainer,
	createResourceFromCallback,
} from "@internal/resources";
import {Consumer, consumeUnknown} from "@internal/consume";

export type Stdout = stream.Writable | tty.WriteStream;

export type InferredTerminalFeatures = {
	features: TerminalFeatures;
	updateEvent: Event<TerminalFeatures, void>;
	setupUpdateEvent: () => Resource;
};

export type TerminalFeatures = {
	columns?: OneIndexed;
	progress: boolean;
	isTTY: boolean;
	background: "dark" | "light" | "unknown";
	cursor: boolean;
	hyperlinks: boolean;
	colorDepth: 1 | 4 | 8 | 24;
	unicode: boolean;
};

export const DEFAULT_TERMINAL_FEATURES: TerminalFeatures = {
	background: "unknown",
	isTTY: false,
	columns: new OneIndexed(100),
	progress: false,
	cursor: false,
	unicode: true,
	hyperlinks: false,
	colorDepth: 4,
};

let envAllConsumer: undefined | Consumer;

export function consumeEnvVar(key: string): Consumer {
	if (envAllConsumer === undefined) {
		envAllConsumer = consumeUnknown({...process.env}, ["parse"], "env");
	}

	return envAllConsumer.get(key);
}

export function getEnvVarBoolean(key: string): undefined | boolean {
	const env = consumeEnvVar(key);
	if (env.exists()) {
		const bool = consumeEnvVar(key).deriveBooleanString();
		if (bool.exists()) {
			return bool.asBoolean();
		} else {
			return undefined;
		}
	} else {
		return undefined;
	}
}

export function isEnvVarSet(key: string): boolean {
	return consumeEnvVar(key).exists();
}

export function inferTerminalFeatures(
	stdout?: Stdout,
	force: Partial<TerminalFeatures> = {},
): InferredTerminalFeatures {
	let columns: OneIndexed = new OneIndexed(100);
	let colorDepth: TerminalFeatures["colorDepth"] = 1;
	let isTTY = force.isTTY === true;
	let unicode = false;
	let background: TerminalFeatures["background"] = "unknown";

	// Increase column size slightly for CI
	if (IS_CI_ENV) {
		columns = new OneIndexed(120);
		colorDepth = 4;
	}

	// Only apply this environment sniffing when we've been given a process stdout stream
	// Otherwise it'll be some custom stream and if they really want to infer from the environment
	// Then they will do it on process.stdout and pass the features as the force param
	if (stdout instanceof tty.WriteStream) {
		isTTY = true;
		unicode = process.platform !== "win32";
		colorDepth = stdout.getColorDepth() as TerminalFeatures["colorDepth"];
		columns = new OneIndexed(stdout.columns);

		// Sniff for the background
		// https://github.com/vim/vim/blob/e3f915d12c8fe0466918a29ab4eaef153f71a2cd/src/term.c#L2943-L2952
		const COLORFGBG = consumeEnvVar("COLORFGBG");
		if (COLORFGBG.exists()) {
			const color = parseInt(String(COLORFGBG.asString()).split(";").pop()!);
			if (!isNaN(color)) {
				if ((color >= 0 && color <= 6) || color === 8) {
					background = "dark";
				} else {
					background = "light";
				}
			}
		}
	}

	const fancyAnsi = getEnvVarBoolean("ROME_CLI_FANCY") ?? (isTTY && !IS_CI_ENV);

	let features: TerminalFeatures = mergeObjects(
		{
			isTTY,
			progress: getEnvVarBoolean("ROME_CLI_PROGRESS") ?? fancyAnsi,
			background: consumeEnvVar("ROME_CLI_BACKGROUND").default(background).asStringSet([
				"unknown",
				"dark",
				"light",
			]),
			columns: consumeEnvVar("ROME_CLI_COLUMNS").deriveNumberString().default(
				columns,
			).asOneIndexedNumber(),
			cursor: getEnvVarBoolean("ROME_CLI_CURSOR") ?? fancyAnsi,
			hyperlinks: getEnvVarBoolean("ROME_CLI_HYPERLINKS") ?? fancyAnsi,
			colorDepth: consumeEnvVar("ROME_CLI_COLOR_DEPTH").deriveNumberString().default(
				colorDepth,
			).asNumberSet([1, 4, 8, 24]),
			unicode: getEnvVarBoolean("ROME_CLI_UNICODE") ?? unicode,
		},
		force,
	);

	const updateEvent: Event<TerminalFeatures, void> = new Event(
		"TerminalFeatures.update",
	);

	let setupUpdateEvent: InferredTerminalFeatures["setupUpdateEvent"] = () => {
		return createResourceContainer("TerminalFeatures.update");
	};

	// Watch for resizing, unless force.columns has been set and we'll consider it to be fixed
	if (stdout instanceof tty.WriteStream && force.columns === undefined) {
		function onStdoutResize() {
			if (stdout instanceof tty.WriteStream) {
				features = {
					...features,
					columns: new OneIndexed(stdout.columns),
				};
				updateEvent.send(features);
			}
		}

		setupUpdateEvent = () => {
			stdout.on("resize", onStdoutResize);

			return createResourceFromCallback(
				"TerminalFeatures.update",
				() => {
					stdout.off("resize", onStdoutResize);
				},
			);
		};
	}

	return {
		updateEvent,
		features,
		setupUpdateEvent,
	};
}

const CI_ENV_NAMES = [
	"CI",
	"TRAVIS",
	"CIRCLECI",
	"APPVEYOR",
	"GITLAB_CI",
	"GITHUB_ACTIONS",
];

export let IS_CI_ENV = false;
for (const key of CI_ENV_NAMES) {
	if (getEnvVarBoolean(key)) {
		IS_CI_ENV = true;
		break;
	}
}

export const IS_ROME_DEV_ENV = getEnvVarBoolean("ROME_DEV") === true;

import stream = require("stream");
import {Event} from "@romefrontend/events";
import {mergeObjects} from "@romefrontend/typescript-helpers";

export type Stdout = stream.Writable & {
	isTTY?: boolean;
	columns?: number;
};

export type InferredTerminalFeatures = {
	features: TerminalFeatures;
	updateEvent: Event<TerminalFeatures, void>;
	setupUpdateEvent: () => void;
	closeUpdateEvent: () => void;
};

export type TerminalFeatures = {
	columns: number;
	cursor: boolean;
	progressBars: boolean;
	hyperlinks: boolean;
	color: boolean;
	unicode: boolean;
};

export const TERMINAL_FEATURES_DEFAULT: TerminalFeatures = {
	columns: 100,
	cursor: true,
	progressBars: true,
	unicode: true,
	hyperlinks: true,
	color: true,
};

type EnvVarStatus =
	| {
			type: "DISABLED";
			value: false;
		}
	| {
			type: "ENABLED";
			value: true | string;
		}
	| {
			type: "UNDEFINED";
			value: undefined;
		};

export function getEnvVar(key: string): EnvVarStatus {
	const value = process.env[key];
	if (value === undefined) {
		return {type: "UNDEFINED", value: undefined};
	}
	if (value === "0" || value === "false") {
		return {type: "DISABLED", value: false};
	}
	if (value === "1" || value === "true") {
		return {type: "ENABLED", value: true};
	}
	return {type: "ENABLED", value};
}

export function inferTerminalFeatures(
	stdout?: Stdout,
	force: Partial<TerminalFeatures> = {},
): InferredTerminalFeatures {
	const isTTY = stdout?.isTTY === true;

	let columns = TERMINAL_FEATURES_DEFAULT.columns;
	let unicode = false;
	let isCI = false;

	// Only apply this environment sniffing when we've been given a process stdout stream
	// Otherwise it'll be some custom stream and if they really want to infer from the environment
	// Then they will do it on process.stdout and pass the features as the force param
	if (
		stdout !== undefined &&
		(stdout === process.stdout || stdout === process.stderr)
	) {
		unicode = process.platform !== "win32";
		isCI = isCIEnv();
	}

	if (stdout === undefined || stdout.columns === undefined) {
		// Increase column size for CI
		if (isCI) {
			columns = 200;
		}
	} else if (stdout.columns !== undefined) {
		columns = stdout.columns;
	}

	const fancyAnsi = isTTY && !isCI;

	let features: TerminalFeatures = mergeObjects(
		{
			columns,
			cursor: fancyAnsi,
			hyperlinks: fancyAnsi,
			progressBars: fancyAnsi,
			color: isTTY || isCI,
			unicode,
		},
		force,
	);

	const updateEvent: Event<TerminalFeatures, void> = new Event({
		name: "update",
	});

	let closeUpdateEvent: InferredTerminalFeatures["closeUpdateEvent"] = () => {};
	let setupUpdateEvent: InferredTerminalFeatures["setupUpdateEvent"] = () => {};

	// Watch for resizing, unless force.columns has been set and we'll consider it to be fixed
	if (stdout !== undefined && force.columns === undefined) {
		function onStdoutResize() {
			if (stdout?.columns !== undefined) {
				features = {
					...features,
					columns: stdout.columns,
				};
				updateEvent.send(features);
			}
		}

		setupUpdateEvent = () => {
			stdout.on("resize", onStdoutResize);
		};

		closeUpdateEvent = () => {
			stdout.off("resize", onStdoutResize);
		};
	}

	return {
		updateEvent,
		features,
		setupUpdateEvent,
		closeUpdateEvent,
	};
}

const CI_ENV_NAMES = [
	"TRAVIS",
	"CIRCLECI",
	"APPVEYOR",
	"GITLAB_CI",
	"GITHUB_ACTIONS",
];

export function isCIEnv(): boolean {
	for (const key of CI_ENV_NAMES) {
		if (getEnvVar(key).type === "ENABLED") {
			return true;
		}
	}
	return false;
}

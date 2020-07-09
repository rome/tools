import stream = require("stream");
import {Event} from "@romefrontend/events";
import {mergeObjects} from "@romefrontend/typescript-helpers";

export type Stdout = stream.Writable & {
	unicode?: boolean;
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
	progressBars: boolean;
	hyperlinks: boolean;
	color: boolean;
	unicode: boolean;
};

export const TERMINAL_FEATURES_DEFAULT: TerminalFeatures = {
	columns: 100,
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
	// Windows terminals are awful
	const unicode =
		stdout?.unicode === undefined
			? process.platform !== "win32"
			: stdout.unicode;

	const isTTY = stdout?.isTTY === true;
	const isCI = isCIEnv();

	let columns = TERMINAL_FEATURES_DEFAULT.columns;

	if (stdout === undefined || stdout.columns === undefined) {
		// Increase column size for CI
		if (isCI) {
			columns = 200;
		}
	} else if (stdout.columns !== undefined) {
		columns = stdout.columns;
	}

	let features: TerminalFeatures = mergeObjects(
		{
			columns,
			hyperlinks: isTTY && !isCI,
			progressBars: isTTY && !isCI,
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

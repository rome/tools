import stream = require("stream");
import tty = require("tty");
import {Event} from "@internal/events";
import {VoidCallback, mergeObjects} from "@internal/typescript-helpers";
import {Number1, ob1Coerce1} from "@internal/ob1";

export type Stdout = stream.Writable | tty.WriteStream;

export type InferredTerminalFeatures = {
	features: TerminalFeatures;
	updateEvent: Event<TerminalFeatures, void>;
	setupUpdateEvent: VoidCallback;
	closeUpdateEvent: VoidCallback;
};

export type TerminalFeatures = {
	columns?: Number1;
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
	columns: ob1Coerce1(100),
	progress: false,
	cursor: false,
	unicode: true,
	hyperlinks: false,
	colorDepth: 4,
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
	let columns: Number1 = ob1Coerce1(100);
	let colorDepth: TerminalFeatures["colorDepth"] = 1;
	let isTTY = force.isTTY === true;
	let unicode = false;
	let isCI = isCIEnv();
	let background: TerminalFeatures["background"] = "unknown";

	// Increase column size for CI
	if (isCI) {
		columns = ob1Coerce1(200);
		colorDepth = 4;
	}

	// Only apply this environment sniffing when we've been given a process stdout stream
	// Otherwise it'll be some custom stream and if they really want to infer from the environment
	// Then they will do it on process.stdout and pass the features as the force param
	if (stdout instanceof tty.WriteStream) {
		isTTY = true;
		unicode = process.platform !== "win32";
		colorDepth = (stdout.getColorDepth() as TerminalFeatures["colorDepth"]);
		columns = ob1Coerce1(stdout.columns);

		// Sniff for the background
		// https://github.com/vim/vim/blob/e3f915d12c8fe0466918a29ab4eaef153f71a2cd/src/term.c#L2943-L2952
		const COLORFGBG = getEnvVar("COLORFGBG");
		if (COLORFGBG.type === "ENABLED") {
			const color = parseInt(String(COLORFGBG.value).split(";").pop()!);
			if (!isNaN(color)) {
				if ((color >= 0 && color <= 6) || color === 8) {
					background = "dark";
				} else {
					background = "light";
				}
			}
		}
	}

	const fancyAnsi = isTTY && !isCI;

	const progress = fancyAnsi && getEnvVar("ROME_PROGRESS").type !== "DISABLED";

	let features: TerminalFeatures = mergeObjects(
		{
			progress,
			isTTY,
			background,
			columns,
			cursor: fancyAnsi,
			hyperlinks: fancyAnsi,
			colorDepth,
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
	if (stdout instanceof tty.WriteStream && force.columns === undefined) {
		function onStdoutResize() {
			if (stdout instanceof tty.WriteStream) {
				features = {
					...features,
					columns: ob1Coerce1(stdout.columns),
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
	"CI",
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

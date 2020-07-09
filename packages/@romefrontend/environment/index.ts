import stream = require("stream");

export type Stdout = stream.Writable & {
	unicode?: boolean;
	isTTY?: boolean;
	columns?: number;
};

export type TerminalFeatures = {
	progressBars: boolean;
	hyperlinks: boolean;
	color: boolean;
	unicode: boolean;
};

export const TERMINAL_FEATURES_ALL: TerminalFeatures = {
	progressBars: true,
	unicode: true,
	hyperlinks: true,
	color: true,
};

export function isEnvVarEnabled(key: string): boolean {
	const val = process.env[key];
	return val !== undefined && val !== "0" && val !== "false";
}

export function inferTerminalFeatures(stdout?: Stdout): TerminalFeatures {
	// Windows terminals are awful
	const unicode =
		stdout?.unicode === undefined
			? process.platform !== "win32"
			: stdout.unicode;

	const isTTY = stdout?.isTTY === true;
	const isCI = isCIEnv();

	const features: TerminalFeatures = {
		hyperlinks: isTTY && !isCI,
		progressBars: isTTY && !isCI,
		color: isTTY || isCI,
		unicode,
	};

	return features;
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
		if (isEnvVarEnabled(key)) {
			return true;
		}
	}
	return false;
}

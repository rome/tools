/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TerminalFeatures} from "@internal/cli-environment";
import {MarkupRGB} from "@internal/markup";

const ESC = "\x1b";

function createColorEscape(num: string | number): string {
	return `${ESC}[${String(num)}m`;
}

function rgbTo8BitAnsi([r, g, b]: MarkupRGB): number {
	if (r === g && g === b) {
		if (r < 8) {
			return 16;
		}

		if (r > 248) {
			return 231;
		}

		return Math.round((r - 8) / 247 * 24) + 232;
	}

	return (
		16 +
		36 * Math.round(r / 255 * 5) +
		6 * Math.round(g / 255 * 5) +
		Math.round(b / 255 * 5)
	);
}

function saturation(rgb: MarkupRGB): number {
	const r = rgb[0] / 255;
	const g = rgb[1] / 255;
	const b = rgb[2] / 255;
	const v = Math.max(r, g, b);
	const diff = v - Math.min(r, g, b);

	let s;
	if (diff === 0) {
		s = 0;
	} else {
		s = diff / v;
	}

	return s * 100;
}

function rgbTo4BitAnsi(color: MarkupRGB): number {
	const [r, g, b] = color;
	let value = saturation(color);

	value = Math.round(value / 50);

	if (value === 0) {
		return 30;
	}

	let ansi =
		30 +
		(Math.round(b / 255) << 2 | Math.round(g / 255) << 1 | Math.round(r / 255));

	if (value === 2) {
		ansi += 60;
	}

	return ansi;
}

type RGBOptions = {
	value: string;
	color: MarkupRGB;
	features: TerminalFeatures;
	background?: boolean;
};

// 4 and 8 bit ANSi color codes can be switched from foreground to background by adding 10
function fgToMaybeBgCode(code: number, opts: RGBOptions): string {
	return String(opts.background ? code + 10 : code);
}

function formatAnsiRGBReset(opts: RGBOptions): string {
	return createColorEscape(fgToMaybeBgCode(39, opts));
}

export function formatAnsiRGB(opts: RGBOptions): string {
	const {color, value} = opts;
	switch (opts.features.colorDepth) {
		case 1:
			return value;

		case 4: {
			const colorCode = fgToMaybeBgCode(rgbTo4BitAnsi(color), opts);
			return createColorEscape(colorCode) + value + formatAnsiRGBReset(opts);
		}

		case 8: {
			const tableCode = rgbTo8BitAnsi(color);
			const colorCode = `${fgToMaybeBgCode(38, opts)};5;${String(tableCode)}`;
			return createColorEscape(colorCode) + value + formatAnsiRGBReset(opts);
		}

		case 24: {
			const tuple = color.join(";");
			const colorCode = `${fgToMaybeBgCode(38, opts)};2;${tuple}`;
			return createColorEscape(colorCode) + value + formatAnsiRGBReset(opts);
		}
	}
}

export const formatAnsi = {
	reset(str: string): string {
		return createColorEscape(0) + str + createColorEscape(0);
	},
	fileHyperlink(name: string, filename: string): string {
		let href = "file://";
		const {HOSTNAME} = process.env;
		if (HOSTNAME != null) {
			href += `${HOSTNAME}/`;
		}
		href += filename;
		return formatAnsi.hyperlink(name, href);
	},
	hyperlink(name: string, href: string): string {
		return `${ESC}]8;;${href}\u0007${name}${ESC}]8;;\u0007`;
	},
	bold(str: string): string {
		return createColorEscape(1) + str + createColorEscape(22);
	},
	dim(str: string): string {
		return createColorEscape(2) + str + createColorEscape(22);
	},
	italic(str: string): string {
		return createColorEscape(3) + str + createColorEscape(23);
	},
	underline(str: string): string {
		return createColorEscape(4) + str + createColorEscape(24);
	},
	inverse(str: string): string {
		return createColorEscape(7) + str + createColorEscape(27);
	},
	hidden(str: string): string {
		return createColorEscape(8) + str + createColorEscape(28);
	},
	strikethrough(str: string): string {
		return createColorEscape(9) + str + createColorEscape(29);
	},
	black(str: string): string {
		return createColorEscape(30) + str + createColorEscape(39);
	},
	brightBlack(str: string): string {
		return createColorEscape(90) + str + createColorEscape(39);
	},
	red(str: string): string {
		return createColorEscape(31) + str + createColorEscape(39);
	},
	brightRed(str: string): string {
		return createColorEscape(91) + str + createColorEscape(39);
	},
	green(str: string): string {
		return createColorEscape(32) + str + createColorEscape(39);
	},
	brightGreen(str: string): string {
		return createColorEscape(92) + str + createColorEscape(39);
	},
	yellow(str: string): string {
		return createColorEscape(33) + str + createColorEscape(39);
	},
	brightYellow(str: string): string {
		return createColorEscape(93) + str + createColorEscape(39);
	},
	blue(str: string): string {
		return createColorEscape(34) + str + createColorEscape(39);
	},
	brightBlue(str: string): string {
		return createColorEscape(94) + str + createColorEscape(39);
	},
	magenta(str: string): string {
		return createColorEscape(35) + str + createColorEscape(39);
	},
	brightMagenta(str: string): string {
		return createColorEscape(95) + str + createColorEscape(39);
	},
	cyan(str: string): string {
		return createColorEscape(36) + str + createColorEscape(39);
	},
	brightCyan(str: string): string {
		return createColorEscape(96) + str + createColorEscape(39);
	},
	white(str: string): string {
		return createColorEscape(37) + str + createColorEscape(39);
	},
	brightWhite(str: string): string {
		return createColorEscape(97) + str + createColorEscape(39);
	},
	bgBlack(str: string): string {
		return createColorEscape(40) + str + createColorEscape(49);
	},
	bgBrightBlack(str: string): string {
		return createColorEscape(100) + str + createColorEscape(49);
	},
	bgRed(str: string): string {
		return createColorEscape(41) + str + createColorEscape(49);
	},
	bgBrightRed(str: string): string {
		return createColorEscape(101) + str + createColorEscape(49);
	},
	bgGreen(str: string): string {
		return createColorEscape(42) + str + createColorEscape(49);
	},
	bgBrightGreen(str: string): string {
		return createColorEscape(102) + str + createColorEscape(49);
	},
	bgYellow(str: string): string {
		return createColorEscape(43) + str + createColorEscape(49);
	},
	bgBrightYellow(str: string): string {
		return createColorEscape(103) + str + createColorEscape(49);
	},
	bgBlue(str: string): string {
		return createColorEscape(44) + str + createColorEscape(49);
	},
	bgBrightBlue(str: string): string {
		return createColorEscape(104) + str + createColorEscape(49);
	},
	bgMagenta(str: string): string {
		return createColorEscape(45) + str + createColorEscape(49);
	},
	bgBrightMagenta(str: string): string {
		return createColorEscape(105) + str + createColorEscape(49);
	},
	bgCyan(str: string): string {
		return createColorEscape(46) + str + createColorEscape(49);
	},
	bgBrightCyan(str: string): string {
		return createColorEscape(106) + str + createColorEscape(49);
	},
	bgWhite(str: string): string {
		return createColorEscape(47) + str + createColorEscape(49);
	},
	bgBrightWhite(str: string): string {
		return createColorEscape(107) + str + createColorEscape(49);
	},
};

export const ansiEscapes = {
	clearScreen: "\x1bc",
	eraseLine: `${ESC}[2K`,
	cursorUp(count: number = 1): string {
		return `${ESC}[${count}A`;
	},
	cursorDown(count: number = 1): string {
		return `${ESC}[${count}B`;
	},
	cursorTo(x: number, y?: number): string {
		if (y === undefined) {
			return `${ESC}[${x + 1}G`;
		}

		return `${ESC}[${y + 1};${x + 1}H`;
	},
};

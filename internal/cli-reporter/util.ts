/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ReporterProgress} from "./types";
import Reporter from "./Reporter";
import readline = require("readline");
import {AnyMarkup, markup} from "@internal/markup";
import {VoidCallback} from "@internal/typescript-helpers";

export function mergeProgresses(
	progresses: Array<ReporterProgress>,
): ReporterProgress {
	if (progresses.length === 1) {
		return progresses[0];
	}

	return {
		render: () => {
			for (const progress of progresses) {
				progress.render();
			}
		},
		setCurrent: (current: number) => {
			for (const progress of progresses) {
				progress.setCurrent(current);
			}
		},
		setTotal: (total: number, approximate?: boolean) => {
			for (const progress of progresses) {
				progress.setTotal(total, approximate);
			}
		},
		setText: (text: AnyMarkup) => {
			for (const progress of progresses) {
				progress.setText(text);
			}
		},
		pushText: (text: AnyMarkup) => {
			let id = "";
			for (const progress of progresses) {
				progress.pushText(text, id);
			}
			return id;
		},
		popText: (id: string) => {
			for (const progress of progresses) {
				progress.popText(id);
			}
		},
		setApproximateETA: (duration: number) => {
			for (const progress of progresses) {
				progress.setApproximateETA(duration);
			}
		},
		tick: () => {
			for (const progress of progresses) {
				progress.tick();
			}
		},
		end: () => {
			for (const progress of progresses) {
				progress.end();
			}
		},
		pause: () => {
			for (const progress of progresses) {
				progress.pause();
			}
		},
		resume: () => {
			for (const progress of progresses) {
				progress.resume();
			}
		},
	};
}

type Key = {
	name: string;
	ctrl: boolean;
};

export function onKeypress(
	reporter: Reporter,
	callback: (key: Key) => void,
): {
	finish: VoidCallback;
} {
	const stdin = reporter.getStdin();

	setRawMode(stdin, true);
	readline.emitKeypressEvents(stdin);

	function onkeypress(chunk: Buffer, key: Key) {
		switch (key.name) {
			case "c": {
				if (key.ctrl) {
					reporter.br({force: true});
					reporter.warn(markup`Cancelled by user`);
					process.exit(1);
				}
				return;
			}

			case "escape": {
				reporter.br({force: true});
				reporter.warn(markup`Cancelled by user`);
				process.exit(1);
				return;
			}
		}

		callback(key);
	}

	stdin.addListener("keypress", onkeypress);

	return {
		finish() {
			stdin.removeListener("keypress", onkeypress);
			setRawMode(stdin, false);
		},
	};
}

export function setRawMode(stdin: NodeJS.ReadStream, raw: boolean) {
	if (stdin.isTTY && stdin.setRawMode !== undefined) {
		stdin.setRawMode(raw);
	}

	if (raw) {
		stdin.resume();
	} else {
		stdin.pause();
	}
}

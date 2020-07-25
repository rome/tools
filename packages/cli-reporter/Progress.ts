/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {humanizeNumber, humanizeTime} from "@romefrontend/string-utils";
import {Reporter} from "@romefrontend/cli-reporter";
import {
	ReporterProgressOptions,
	ReporterStream,
	ReporterStreamLineSnapshot,
} from "./types";
import ProgressBase from "./ProgressBase";
import {Markup, formatAnsi} from "@romefrontend/cli-layout";
import {Number1, ob1Get1} from "@romefrontend/ob1";
import * as streamUtils from "./stream";

type BoldRanges = Array<[number, number]>;

type SplitBar = Array<[number, string]>;

// 30 columns a second
const BOUNCER_INTERVAL = 1_000 / 30;
const BOUNCER_WIDTH = 20;

export default class Progress extends ProgressBase {
	constructor(
		reporter: Reporter,
		opts: ReporterProgressOptions = {},
		onEnd?: () => void,
	) {
		super(reporter, opts);

		this.firstRenderLineSnapshot = undefined;
		this.startTime = Date.now();
		this.lastRenderTime = Date.now();
		this.lastRenderCurrent = 0;

		this.closed = false;
		this.onEnd = onEnd;

		this.delay = 60;
		this.renderEvery = 0;

		this.streamToBouncerStart = new Map();
		this.startBouncer();

		this.queueRender(opts.initDelay);
		this.initName(opts.name);
	}

	closed: boolean;
	delay: number;
	renderTimer: undefined | NodeJS.Timeout;
	firstRenderLineSnapshot: undefined | ReporterStreamLineSnapshot;

	streamToBouncerStart: Map<ReporterStream, number>;
	bouncerTimer: undefined | NodeJS.Timeout;

	onEnd: undefined | (() => void);
	renderEvery: number;
	startTime: number;
	lastRenderCurrent: number;
	lastRenderTime: number;

	initName(name: undefined | string) {
		if (name === undefined) {
			return;
		}

		// TODO fetch approximate total and eta based on `name`
	}

	getElapsedTime(): number {
		return Date.now() - this.startTime - this.pausedElapsed;
	}

	getBouncerPosition(stream: ReporterStream): number {
		const start = this.streamToBouncerStart.get(stream);
		if (start === undefined) {
			return 0;
		} else {
			return start;
		}
	}

	startBouncer() {
		const queueTick = () => {
			this.bouncerTimer = setTimeout(tick, BOUNCER_INTERVAL);
		};

		const tick = this.reporter.wrapCallback(() => {
			if (this.paused) {
				queueTick();
				return;
			}

			const elapsedTime = this.getElapsedTime();
			const elapsedFrames = Math.round(elapsedTime / BOUNCER_INTERVAL);

			for (const {stream} of this.reporter.streamHandles) {
				if (
					!streamUtils.isANSICursorStream(stream) ||
					stream.features.columns === undefined
				) {
					continue;
				}

				// We remove the bouncer width from the total columns since we'll append it
				const width = ob1Get1(stream.features.columns) - BOUNCER_WIDTH;

				// Position to place the bouncer
				let position = elapsedFrames % width;

				// Every odd complete bounce should reverse direction
				const totalBounces = Math.floor(elapsedFrames / width);
				if (totalBounces % 2 === 1) {
					position = width - position;
				}

				this.streamToBouncerStart.set(stream, position);
			}

			queueTick();
			this.render();
		});

		queueTick();
	}

	setCurrent(current: number) {
		if (this.closed) {
			return;
		}

		super.setCurrent(current);

		if (this.isRenderDue()) {
			this.render();
		}
	}

	setTotal(total: number, approximate: boolean = false) {
		super.setTotal(total, approximate);
		this.renderEvery = Math.round(total / 100);
		this.endBouncer();
	}

	setText(text: Markup) {
		if (this.closed) {
			return;
		}

		super.setText(text);
	}

	queueRender(delay: number = this.delay) {
		if (this.closed) {
			// Progress bar has been removed
			return;
		}

		if (this.renderTimer !== undefined) {
			// Render already queued
			return;
		}

		this.renderTimer = setTimeout(
			this.reporter.wrapCallback(() => {
				this.render();
			}),
			delay,
		);
	}

	endBouncer() {
		if (this.bouncerTimer !== undefined) {
			clearTimeout(this.bouncerTimer);
		}
		this.bouncerTimer = undefined;
	}

	endRender() {
		if (this.renderTimer !== undefined) {
			clearTimeout(this.renderTimer);
		}
		this.renderTimer = undefined;
	}

	end() {
		this.closed = true;
		this.endBouncer();
		this.endRender();

		const {firstRenderLineSnapshot} = this;
		if (firstRenderLineSnapshot !== undefined) {
			this.reporter.removeLine(firstRenderLineSnapshot);
			firstRenderLineSnapshot.close();
		}

		if (this.onEnd !== undefined) {
			this.onEnd();
		}
	}

	// Ensure that we update the progress bar after a certain amount of ticks
	// This allows us to use the progress bar for sync work where the event loop is always blocked
	isRenderDue(): boolean {
		const isDue: boolean =
			this.current > this.lastRenderCurrent + this.renderEvery;
		if (isDue) {
			// We also make sure that we never force update more often than once a second
			// This is to ensure that the progress bar isn't negatively effecting performance
			const timeSinceLastRender: number = Date.now() - this.lastRenderTime;
			return timeSinceLastRender > 1_000;
		} else {
			return false;
		}
	}

	isBoldCharacter(i: number, ranges: BoldRanges): boolean {
		for (const [start, end] of ranges) {
			if (start >= i && end <= i) {
				return true;
			}
		}

		return false;
	}

	splitCharacters(str: string, boldRanges: BoldRanges): SplitBar {
		return str.split("").map((char, i) => {
			if (this.isBoldCharacter(i, boldRanges)) {
				return [i, formatAnsi.bold(char)];
			} else {
				return [i, char];
			}
		});
	}

	buildProgressBouncer(stream: ReporterStream, bar: SplitBar): string {
		let start = this.getBouncerPosition(stream);
		let fullBar = "";
		for (const [i, char] of bar) {
			const isBounce = i >= start && i < start + BOUNCER_WIDTH;

			if (isBounce) {
				if (this.paused) {
					fullBar += formatAnsi.inverse(char);
				} else {
					fullBar += formatAnsi.white(formatAnsi.bgYellow(char));
				}
			} else {
				fullBar += char;
			}
		}
		return fullBar;
	}

	buildProgressBar(
		stream: ReporterStream,
		columns: Number1,
		bar: SplitBar,
		total: number,
	): string {
		const ratio = Math.min(Math.max(this.current / total, 0), 1);

		const completeLength = Math.round(ob1Get1(columns) * ratio);
		let fullBar = "";
		for (const [i, char] of bar) {
			if (i < completeLength) {
				if (this.paused) {
					fullBar += formatAnsi.inverse(char);
				} else {
					fullBar += formatAnsi.white(formatAnsi.bgGreen(char));
				}
			} else {
				fullBar += char;
			}
		}
		return fullBar;
	}

	buildBar(stream: ReporterStream, columns: Number1) {
		const {total, current, title} = this;

		// Text ranges that we should make bold
		const boldRanges: BoldRanges = [];

		// Text to prefix to the bar
		let prefix = "";
		if (title !== undefined) {
			prefix += title;

			// Only the title should be bold, not the subtext
			boldRanges.push([0, prefix.length - 1]);
		}

		const text = this.getText();
		if (text !== undefined) {
			// Separate a title and it's text with a colon
			if (title !== undefined) {
				prefix += ": ";
			}
			prefix += text;
		}

		// Text to put at the end of the bar
		let suffix = "";

		// Total time since the progress bar was created
		const elapsed = this.getElapsedTime();

		// Time elapsed eg: elapsed 1m5s
		if (this.opts.elapsed) {
			suffix += `elapsed ${humanizeTime(elapsed)} `;
		}

		// Don't bother with a suffix if we haven't completed a single item
		if (current > 0) {
			// How many milliseconds spent per total items
			const averagePerItem = elapsed / current;

			// ETA eg: 1m5s
			if (this.opts.eta) {
				if (this.approximateETA !== undefined && elapsed < this.approximateETA) {
					// Approximate ETA
					const left = elapsed - this.approximateETA;
					suffix += `eta ~${humanizeTime(left)} `;
				} else if (total !== undefined) {
					// How many items we have left
					const itemsLeft = total - current;

					// Total estimated time left
					const eta = itemsLeft * averagePerItem;
					suffix += `eta ${humanizeTime(eta)} `;
				} else {
					const ops = Math.round(1_000 / averagePerItem);
					suffix += `${humanizeNumber(ops)} op/s `;
				}
			}

			// Counter eg: 5/100
			suffix += `${humanizeNumber(current)}`;
			if (total !== undefined) {
				suffix += "/";
				if (this.approximateTotal) {
					suffix += "~";
				}
				suffix += humanizeNumber(total);
			}
		}

		// Get the full width of the bar. We take off 3 for padding.
		const width = ob1Get1(columns) - 3;

		// The amount of spaces to put between the title and counter
		const spacerLength = Math.max(0, width - prefix.length - suffix.length);
		const spacer = " ".repeat(spacerLength);

		// Trim the prefix if it will overflow
		prefix = prefix.slice(0, width - spacerLength - suffix.length);

		// The full raw bar without any coloring
		const raw = ` ${prefix}${spacer} ${suffix}`;

		// Make sure the counter is bold
		boldRanges.push([raw.length - suffix.length, raw.length - 1]);

		// Split the raw bar into an array of formatted characters
		const chars = this.splitCharacters(raw, boldRanges);

		if (total === undefined) {
			return this.buildProgressBouncer(stream, chars);
		} else {
			return this.buildProgressBar(stream, columns, chars, total);
		}
	}

	render() {
		if (this.closed) {
			return;
		}

		this.endRender();

		this.lastRenderCurrent = this.current;
		this.lastRenderTime = Date.now();

		if (this.firstRenderLineSnapshot === undefined) {
			this.firstRenderLineSnapshot = this.reporter.getLineSnapshot(false);
		}

		for (const {stream} of this.reporter.getStreamHandles()) {
			if (
				streamUtils.isANSICursorStream(stream) &&
				stream.features.columns !== undefined
			) {
				streamUtils.log(
					stream,
					this.buildBar(stream, stream.features.columns),
					{
						replaceLineSnapshot: this.firstRenderLineSnapshot,
						preferNoNewline: true,
					},
				);
			}
		}
	}
}

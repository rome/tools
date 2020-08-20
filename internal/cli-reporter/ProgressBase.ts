/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Reporter from "./Reporter";
import {ReporterProgress, ReporterProgressOptions} from "./types";
import {mergeObjects} from "@internal/typescript-helpers";
import {AnyMarkup, AnyMarkups, isEmptyMarkup, markup} from "@internal/markup";

const DEFAULT_PROGRESS_OPTIONS: ReporterProgressOptions = {
	name: undefined,
	title: undefined,
	initDelay: undefined,
	elapsed: true,
	eta: true,
	persistent: false,
};

export default class ProgressBase implements ReporterProgress {
	constructor(reporter: Reporter, opts: ReporterProgressOptions = {}) {
		this.total = undefined;
		this.reporter = reporter;
		this.current = 0;

		this.approximateTotal = false;
		this.approximateETA = undefined;

		this.textIdCounter = 0;
		this.textIdStack = [];
		this.textStack = [];
		this.textIds = new Set();

		this.text = undefined;
		this.title =
			opts.title === undefined ? undefined : reporter.stripMarkup(opts.title);

		this.paused = false;
		this.pausedStart = undefined;
		this.pausedElapsed = 0;

		this.opts = mergeObjects(DEFAULT_PROGRESS_OPTIONS, opts);
	}

	protected reporter: Reporter;
	protected opts: ReporterProgressOptions;

	protected current: number;
	protected total: undefined | number;

	protected text: undefined | AnyMarkup;
	protected textIdCounter: number;
	protected textIdStack: Array<string>;
	protected textStack: AnyMarkups;
	protected textIds: Set<string>;

	protected title: undefined | string;

	protected approximateETA: undefined | number;
	protected approximateTotal: boolean;

	protected pausedStart: undefined | number;
	protected pausedElapsed: number;
	protected paused: boolean;

	public setCurrent(current: number) {
		this.current = current;
		this.queueRender();

		// Progress complete
		if (
			this.total !== undefined &&
			this.current >= this.total &&
			!this.opts.persistent
		) {
			this.end();
		}
	}

	public getText(): undefined | string {
		const {text} = this;
		if (text === undefined || isEmptyMarkup(text)) {
			return undefined;
		} else {
			return this.reporter.stripMarkup(text);
		}
	}

	public setText(text: AnyMarkup) {
		this.text = text;
		this.queueRender();
	}

	public setApproximateETA(duration: number) {
		this.approximateETA = duration;
	}

	public setTotal(total: number, approximate: boolean = false) {
		this.total = total;
		this.approximateTotal = approximate;
		this.queueRender();
	}

	public pushText(text: AnyMarkup, id?: string): string {
		if (id === undefined) {
			id = String(this.textIdCounter++);
		}
		if (this.textIds.has(id)) {
			throw new Error(`Progress bar text ${id} already exists`);
		}
		this.setText(text);
		this.textStack.push(text);
		this.textIdStack.push(id);
		this.textIds.add(id);
		return id;
	}

	public popText(id: string) {
		// Find
		const {textStack, textIdStack, textIds} = this;
		const index = textIdStack.indexOf(id);
		if (index === -1) {
			throw new Error(`No pushed text for id ${id}`);
		}

		// Remove
		textStack.splice(index, 1);
		textIdStack.splice(index, 1);
		textIds.delete(id);

		// Set last
		const last: undefined | AnyMarkup = textStack[textStack.length - 1];
		this.setText(last ?? markup``);
	}

	public tick() {
		this.setCurrent(this.current + 1);
	}

	public resume() {
		if (!this.paused || this.pausedStart === undefined) {
			return;
		}

		this.pausedElapsed += Date.now() - this.pausedStart;
		this.pausedStart = undefined;
		this.paused = false;
		this.render();
	}

	public pause() {
		if (this.paused) {
			return;
		}

		this.pausedStart = Date.now();
		this.paused = true;
		this.render();
	}

	protected queueRender() {
		this.render();
	}

	public end() {}

	public render() {}
}

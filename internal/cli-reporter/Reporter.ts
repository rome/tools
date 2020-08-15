/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	UserGridOptions,
	ansiEscapes,
	markupToAnsi,
	markupToHtml,
	markupToJoinedPlainText,
	markupToPlainText,
} from "@internal/cli-layout";
import {
	AnyMarkup,
	AnyMarkups,
	MarkupFormatOptions,
	MarkupTagName,
	StaticMarkup,
	concatMarkup,
	convertToMarkupFromRandomString,
	isEmptyMarkup,
	markup,
	markupTag,
	normalizeMarkup,
	readMarkup,
} from "@internal/markup";
import {
	ReporterCaptureStream,
	ReporterConditionalStream,
	ReporterDerivedStreams,
	ReporterListOptions,
	ReporterNamespace,
	ReporterProgress,
	ReporterProgressOptions,
	ReporterStepCallback,
	ReporterStream,
	ReporterStreamAttached,
	ReporterStreamHandle,
	ReporterStreamLineSnapshot,
	ReporterStreamState,
	SelectArguments,
	SelectOptions,
	SelectOptionsKeys,
} from "./types";
import Progress from "./Progress";
import prettyFormat from "@internal/pretty-format";
import stream = require("stream");
import {CWD_PATH} from "@internal/path";
import readline = require("readline");
import select from "./select";
import {onKeypress} from "./util";

import {
	DEFAULT_TERMINAL_FEATURES,
	Stdout,
	TerminalFeatures,
	inferTerminalFeatures,
} from "@internal/cli-environment";
import * as streamUtils from "./stream";
import {
	AsyncVoidCallback,
	VoidCallback,
	mergeObjects,
} from "@internal/typescript-helpers";
import highlightShell from "@internal/markup-syntax-highlight/highlightShell";

// rome-ignore lint/ts/noExplicitAny
type WrapperFactory = <T extends (...args: Array<any>) => any>(callback: T) => T;

export type ReporterOptions = {
	streams?: Array<ReporterStreamAttached>;
	stdin?: NodeJS.ReadStream;
	markupOptions?: MarkupFormatOptions;
	wrapperFactory?: WrapperFactory;
};

export type LogOptions = {
	replaceLineSnapshot?: ReporterStreamLineSnapshot;
	handles?: Array<ReporterStreamHandle>;
	stderr?: boolean;
	noNewline?: boolean;
	preferNoNewline?: boolean;
};

export type LogCategoryUserOptions = Pick<LogOptions, "stderr" | "handles">;

export type LogCategoryOptions = LogCategoryUserOptions & {
	unicodePrefix: string;
	rawPrefix: string;
	markupTag: MarkupTagName;
};

type QuestionValidateResult<T> = [false, StaticMarkup] | [true, T];

type QuestionOptions = {
	hint?: string;
	default?: string;
	yes?: boolean;
	normalize?: (value: string) => string;
};

export default class Reporter implements ReporterNamespace {
	constructor(opts: ReporterOptions = {}) {
		this.activeElements = new Set();
		this.indentLevel = 0;
		this.markupOptions =
			opts.markupOptions === undefined ? {} : opts.markupOptions;
		this.shouldRedirectOutToErr = false;
		this.stdin = opts.stdin;
		this.wrapperFactory = opts.wrapperFactory;
		this.streamHandles = new Set();

		if (opts.streams !== undefined) {
			for (const stream of opts.streams) {
				this.addStream(stream);
			}
		}
	}

	public markupOptions: MarkupFormatOptions;

	private indentLevel: number;
	private shouldRedirectOutToErr: boolean;
	private wrapperFactory: undefined | WrapperFactory;
	private streamHandles: Set<ReporterStreamHandle>;
	private stdin: undefined | NodeJS.ReadStream;

	// Store active progress indicators so we can easily bail out and destroy them
	private activeElements: Set<{
		render: VoidCallback;
		end: VoidCallback;
	}>;

	public static fromProcess(opts: ReporterOptions = {}): Reporter {
		const reporter = new Reporter({
			...opts,
			markupOptions: {
				cwd: CWD_PATH,
				...opts.markupOptions,
			},
		});

		reporter.attachStdoutStreams(process.stdout, process.stderr);

		return reporter;
	}

	// Produce a new Reporter with all the streams of the input reporters. Streams will NOT be in sync.
	public static concat(reporters: Array<Reporter>): Reporter {
		const reporter = new Reporter();
		for (const otherReporter of reporters) {
			for (const {stream} of otherReporter.getStreamHandles()) {
				reporter.addAttachedStream(stream);
			}
		}
		return reporter;
	}

	public getLineSnapshot(populate: boolean = true): ReporterStreamLineSnapshot {
		const snapshot: ReporterStreamLineSnapshot = {
			close: () => {
				for (const {stream} of this.getStreamHandles()) {
					stream.state.lineSnapshots.delete(snapshot);
				}
			},
		};

		if (populate) {
			for (const {stream} of this.getStreamHandles()) {
				stream.state.lineSnapshots.set(snapshot, stream.state.currentLine);
			}
		}

		return snapshot;
	}

	public attachStdoutStreams(
		stdout?: Stdout,
		stderr?: Stdout,
		force: Partial<TerminalFeatures> & {
			format?: ReporterStream["format"];
		} = {},
	): ReporterDerivedStreams {
		const {features, updateEvent, setupUpdateEvent, closeUpdateEvent} = inferTerminalFeatures(
			stdout,
			force,
		);

		const {format = features.colorDepth > 1 ? "ansi" : "none"} = force;

		setupUpdateEvent();

		const handle = this.addStream({
			format,
			features,
			write: (chunk, error) => {
				if (error) {
					if (stderr !== undefined) {
						// @ts-ignore
						stderr.write(chunk);
					}
				} else {
					if (stdout !== undefined) {
						// @ts-ignore
						stdout.write(chunk);
					}
				}
			},
			init: setupUpdateEvent,
			teardown: closeUpdateEvent,
		});

		updateEvent.subscribe((features) => {
			handle.stream.updateFeatures(features);
		});

		return {
			format,
			features,
			featuresUpdated: updateEvent,
			handle,
		};
	}

	public attachConditionalStream(
		stream: ReporterStream,
		check: () => boolean,
	): ReporterConditionalStream {
		let handle: undefined | ReporterStreamHandle;

		const cond: ReporterConditionalStream = {
			update: () => {
				if (check()) {
					if (handle === undefined) {
						handle = this.addStream(stream);
					}
					return true;
				} else {
					if (handle !== undefined) {
						handle.remove();
						handle = undefined;
					}
					return false;
				}
			},
		};

		cond.update();

		return cond;
	}

	public attachCaptureStream(
		format: ReporterStream["format"] = "none",
		features: Partial<TerminalFeatures> = {},
	): ReporterCaptureStream {
		let buff = "";

		const stream = this.addStream({
			format,
			features: {
				...DEFAULT_TERMINAL_FEATURES,
				...features,
			},
			write(chunk) {
				buff += chunk;
			},
		});

		return {
			read() {
				return buff;
			},
			readAsMarkup() {
				return format === "markup"
					? convertToMarkupFromRandomString(buff)
					: markup`${buff}`;
			},
			remove: stream.remove,
		};
	}

	protected getMessagePrefix(): AnyMarkup {
		return markup``;
	}

	public redirectOutToErr(should: boolean): boolean {
		const old = this.shouldRedirectOutToErr;
		this.shouldRedirectOutToErr = should;
		return old;
	}

	private refreshActiveElements() {
		for (const elem of this.activeElements) {
			elem.render();
		}
	}

	public addAttachedStream(stream: ReporterStreamAttached): ReporterStreamHandle {
		const handle: ReporterStreamHandle = {
			stream,
			remove: () => {
				if (!this.streamHandles.has(handle)) {
					return;
				}

				this.streamHandles.delete(handle);
				stream.handles.delete(handle);

				// Only teardown when all handles have been removed
				if (stream.teardown !== undefined && stream.handles.size === 0) {
					stream.teardown();
				}
			},
		};
		stream.handles.add(handle);
		this.streamHandles.add(handle);
		return handle;
	}

	public addStream(
		unattached: ReporterStream,
		state?: Partial<ReporterStreamState>,
	): ReporterStreamHandle {
		if (unattached.init !== undefined) {
			unattached.init();
		}

		const stream: ReporterStreamAttached = {
			...unattached,
			handles: new Set(),
			state: mergeObjects(streamUtils.createStreamState(), state),
			updateFeatures: (newFeatures) => {
				stream.features = newFeatures;
				this.refreshActiveElements();
			},
		};
		return this.addAttachedStream(stream);
	}

	public getStdin(): NodeJS.ReadStream {
		const {stdin} = this;
		if (stdin === undefined) {
			throw new Error("This operation expected a stdin but we have none");
		}
		return stdin;
	}

	public async question(
		message: AnyMarkup,
		{hint, default: def = "", yes = false}: QuestionOptions = {},
	): Promise<string> {
		if (yes) {
			return def;
		}

		const stdin = this.getStdin();

		const origPrompt = markup`<dim emphasis>?</dim> <emphasis>${message}</emphasis>`;
		let prompt = origPrompt;
		if (hint !== undefined) {
			prompt = markup`${prompt} <dim>${hint}</dim>`;
		}
		if (def !== "") {
			prompt = markup`${prompt} (${def})`;
		}
		prompt = markup`${prompt}: `;
		this.log(
			prompt,
			{
				noNewline: true,
			},
		);

		const rl = readline.createInterface({
			input: stdin,
			output: new stream.Writable({
				write: (chunk, encoding, callback) => {
					this.write(chunk);
					callback();
				},
			}),
			terminal: false,
		});

		return new Promise((resolve) => {
			rl.on(
				"line",
				(line) => {
					rl.close();

					const normalized = line === "" ? def : line;

					// Replace initial prompt
					this.write(ansiEscapes.cursorUp());
					this.write(ansiEscapes.eraseLine);

					let prompt = origPrompt;
					prompt = markup`${prompt}: `;
					if (normalized === "") {
						prompt = markup`${prompt}<dim>empty</dim>`;
					} else {
						prompt = markup`${prompt}${normalized}`;
					}
					this.log(prompt);

					resolve(normalized);
				},
			);
		});
	}

	public async questionValidate<T>(
		message: AnyMarkup,
		validate: (value: string) => QuestionValidateResult<T>,
		options: Omit<QuestionOptions, "normalize"> = {},
	): Promise<T> {
		while (true) {
			let res: undefined | QuestionValidateResult<T>;

			await this.question(
				message,
				{
					...options,
					normalize: (value: string): string => {
						res = validate(value);

						if (res[0] === true && typeof res[1] === "string") {
							return res[1];
						} else {
							return value;
						}
					},
				},
			);

			if (res === undefined) {
				throw new Error("normalize should have been called");
			}

			if (res[0] === false) {
				this.error(res[1]);
			} else {
				return res[1];
			}
		}
	}

	public async radioConfirm(message: AnyMarkup): Promise<boolean> {
		const answer = await this.radio(
			message,
			{
				options: {
					yes: {
						label: markup`Yes`,
						shortcut: "y",
					},
					no: {
						label: markup`No`,
						shortcut: "n",
					},
				},
			},
		);
		return answer === "yes";
	}

	public async confirm(
		message: string = "Press any key to continue",
	): Promise<void> {
		this.log(markup`<dim>${message}</dim>`, {noNewline: true});

		await new Promise((resolve) => {
			const keypress = onKeypress(
				this,
				() => {
					keypress.finish();
					resolve();
				},
			);
		});

		// Newline
		this.log(markup``);
	}

	public async radio<Options extends SelectOptions>(
		message: AnyMarkup,
		arg: SelectArguments<Options>,
	): Promise<SelectOptionsKeys<Options>> {
		const set = await this.select(message, {...arg, radio: true});

		// Should always have at least one element
		return Array.from(set)[0];
	}

	public async select<Options extends SelectOptions>(
		message: AnyMarkup,
		args: SelectArguments<Options>,
	): Promise<Set<SelectOptionsKeys<Options>>> {
		return select(this, message, args);
	}

	public hasStreamHandles(opts?: LogCategoryUserOptions): boolean {
		if (opts !== undefined && opts.handles !== undefined) {
			return opts.handles.length > 0;
		}

		return this.streamHandles.size > 0;
	}

	public getStreamHandles(
		opts?: LogCategoryUserOptions,
	): Iterable<ReporterStreamHandle> {
		if (opts !== undefined && opts.handles !== undefined) {
			return opts.handles;
		}

		return this.streamHandles;
	}

	public teardown() {
		for (const handle of this.streamHandles) {
			handle.remove();
		}

		for (const elem of this.activeElements) {
			elem.end();
		}
		this.activeElements.clear();
	}

	public fork(opts: Partial<ReporterOptions> = {}) {
		return new Reporter({
			streams: [...Array.from(this.streamHandles, (handle) => handle.stream)],
			markupOptions: this.markupOptions,
			wrapperFactory: this.wrapperFactory,
			...opts,
		});
	}

	public table(head: AnyMarkups, rawBody: Array<AnyMarkups>) {
		let body: AnyMarkups = [];

		if (head.length > 0) {
			body.push(markup`<tr>`);
			for (const field of head) {
				body.push(markup`<td><emphasis>${field}</emphasis></td>`);
			}
			body.push(markup`</tr>`);
		}

		for (const row of rawBody) {
			body.push(markup`<tr>`);
			for (let field of row) {
				body.push(markup`<td>${field}</td>`);
			}
			body.push(markup`</tr>`);
		}

		this.log(markup`<table>${concatMarkup(body)}</table>`);
	}

	public inspect(value: unknown, opts?: LogOptions) {
		if (!this.hasStreamHandles(opts)) {
			return;
		}

		let formatted;
		if (typeof value !== "number" && typeof value !== "string") {
			formatted = markup`${prettyFormat(value)}`;
		} else {
			formatted = markup`${String(value)}`;
		}

		for (const {stream} of this.getStreamHandles(opts)) {
			this._logMarkup(stream, formatted, opts);
		}
	}

	public write(msg: string, stderr: boolean = false) {
		for (const {stream} of this.getStreamHandles()) {
			stream.write(msg, stderr || this.shouldRedirectOutToErr);
		}
	}

	public clearScreen(opts?: LogCategoryUserOptions) {
		for (const {stream} of this.getStreamHandles(opts)) {
			streamUtils.clearScreen(stream);
		}
	}

	public heading(text: AnyMarkup) {
		this.br();
		this.log(markup`<inverse><emphasis> ${text} </emphasis></inverse>`);
		this.br();
	}

	public async indent(callback: AsyncVoidCallback) {
		this.indentLevel++;

		try {
			await callback();
		} finally {
			this.indentLevel--;
		}
	}

	public indentSync(callback: VoidCallback) {
		this.indentLevel++;

		try {
			callback();
		} finally {
			this.indentLevel--;
		}
	}

	public async section(
		title: undefined | StaticMarkup,
		callback: AsyncVoidCallback,
		opts?: LogCategoryUserOptions,
	): Promise<void> {
		this.hr(
			title === undefined ? undefined : markup`<emphasis>${title}</emphasis>`,
			opts,
		);
		await this.indent(callback);
		this.br(opts);
	}

	public hr(text: AnyMarkup = markup``, opts?: LogCategoryUserOptions) {
		for (const {stream} of this.getStreamHandles(opts)) {
			this.br();
			this._logMarkup(stream, markup`<hr>${text}</hr>`);
			this._logMarkup(stream, markup``);
		}
	}

	public removeLine(
		snapshot: ReporterStreamLineSnapshot,
		opts?: LogCategoryUserOptions,
	) {
		for (const {stream} of this.getStreamHandles(opts)) {
			streamUtils.removeLine(
				stream,
				snapshot,
				opts !== undefined && opts.stderr,
			);
		}
	}

	public async steps(
		callbacks: Array<ReporterStepCallback>,
		clear: boolean = true,
	) {
		let total = 0;
		let current = 1;

		const filteredCallbacks: Array<ReporterStepCallback> = [];
		for (const item of callbacks) {
			if (item.test === undefined || (await item.test())) {
				filteredCallbacks.push(item);
				total++;
			}
		}

		for (const {message, callback} of filteredCallbacks) {
			const lineSnapshot = this.getLineSnapshot();

			try {
				this.step(current, total, message);

				await callback();

				if (clear) {
					this.removeLine(lineSnapshot);
				}
			} finally {
				lineSnapshot.close();
			}
		}
	}

	public step(current: number, total: number, msg: AnyMarkup, opts?: LogOptions) {
		this.log(
			markup`<dim>[${String(current)}/${String(total)}]</dim> ${msg}`,
			opts,
		);
	}

	public br(
		opts?: LogCategoryUserOptions & {
			force?: boolean;
		},
	) {
		const force = opts !== undefined && opts.force;
		for (const {stream} of this.getStreamHandles(opts)) {
			if (streamUtils.getLeadingNewlineCount(stream) < 2 || force) {
				this._logMarkup(stream, markup``, opts);
			}
		}
	}

	public wrapCallback: WrapperFactory = (callback) => {
		const {wrapperFactory} = this;
		if (wrapperFactory === undefined) {
			return callback;
		} else {
			return wrapperFactory(callback);
		}
	};

	public stripMarkup(str: AnyMarkup): string {
		return markupToJoinedPlainText(str, this.markupOptions);
	}

	private format(stream: ReporterStreamAttached, str: AnyMarkup): Array<string> {
		if (isEmptyMarkup(str)) {
			return [""];
		}

		const prefix = this.getMessagePrefix();
		let built = isEmptyMarkup(prefix)
			? str
			: markup`${prefix}<view>${str}</view>`;

		const shouldIndent = this.indentLevel > 0;

		if (shouldIndent) {
			for (let i = 0; i < this.indentLevel; i++) {
				built = markup`<indent>${built}</indent>`;
			}
		}

		const gridMarkupOptions: UserGridOptions = {
			...this.markupOptions,
			columns: stream.features.columns,
			features: stream.features,
			// Printing to a terminal or something so convert tabs to spaces
			convertTabs: true,
		};

		switch (stream.format) {
			case "ansi":
				return markupToAnsi(built, gridMarkupOptions).lines;

			case "html":
				return markupToHtml(built, gridMarkupOptions).lines;

			case "none":
				return markupToPlainText(built, gridMarkupOptions).lines;

			case "markup":
				return [readMarkup(normalizeMarkup(built, this.markupOptions).text)];
		}
	}

	public log(msg: AnyMarkup, opts: LogOptions = {}) {
		for (const {stream} of this.getStreamHandles(opts)) {
			this._logMarkup(stream, msg, opts);
		}
	}

	public logRaw(msg: string, opts: LogOptions = {}) {
		opts = {
			...opts,
			stderr: opts.stderr || this.shouldRedirectOutToErr,
		};

		for (const {stream} of this.getStreamHandles(opts)) {
			streamUtils.log(stream, msg, opts);
		}
	}

	private _logMarkup(
		stream: ReporterStreamAttached,
		msg: AnyMarkup,
		opts: LogOptions = {},
	) {
		const lines = this.format(stream, msg);
		for (let i = 0; i < lines.length; i++) {
			streamUtils.log(
				stream,
				lines[i],
				{
					...opts,
					stderr: opts.stderr || this.shouldRedirectOutToErr,
					noNewline: i === lines.length - 1 ? opts.noNewline : false,
				},
				i,
			);
		}
	}

	private logCategory(rawInner: AnyMarkup, opts: LogCategoryOptions) {
		if (!this.hasStreamHandles(opts)) {
			return;
		}

		let inner = markupTag(opts.markupTag, rawInner);

		for (const {stream} of this.getStreamHandles(opts)) {
			const prefixInner = stream.features.unicode
				? markup`${opts.unicodePrefix}`
				: markup`${opts.rawPrefix}`;
			const prefix = markupTag(
				"emphasis",
				markupTag(opts.markupTag, prefixInner),
			);
			const prefixedInner = markup`${prefix}<view>${inner}</view>`;
			this._logMarkup(
				stream,
				prefixedInner,
				{
					stderr: opts.stderr,
				},
			);
		}
	}

	public success(msg: AnyMarkup, opts?: LogCategoryUserOptions) {
		this.logCategory(
			msg,
			mergeObjects<LogCategoryOptions>(
				{
					unicodePrefix: "\u2714 ",
					rawPrefix: "\u221a ",
					markupTag: "success",
				},
				opts,
			),
		);
	}

	public error(msg: AnyMarkup, opts?: LogOptions) {
		this.logCategory(
			msg,
			mergeObjects<LogCategoryOptions>(
				{
					markupTag: "error",
					unicodePrefix: "\u2716 ",
					rawPrefix: "\xd7 ",
					stderr: true,
				},
				opts,
			),
		);
	}

	public info(msg: AnyMarkup, opts?: LogOptions) {
		this.logCategory(
			msg,
			mergeObjects<LogCategoryOptions>(
				{
					unicodePrefix: "\u2139 ",
					rawPrefix: "i ",
					markupTag: "info",
				},
				opts,
			),
		);
	}

	public warn(msg: AnyMarkup, opts?: LogOptions) {
		this.logCategory(
			msg,
			mergeObjects<LogCategoryOptions>(
				{
					unicodePrefix: "\u26a0 ",
					rawPrefix: "! ",
					markupTag: "warn",
					stderr: true,
				},
				opts,
			),
		);
	}

	public command(command: string, prefix: boolean = true) {
		let highlighted = concatMarkup(
			highlightShell({
				input: command,
			}),
			markup`\n`,
		);

		if (prefix) {
			highlighted = markup`<emphasis>$</emphasis> <view>${highlighted}</view>`;
		}

		this.log(highlighted);
	}

	public namespace(prefix: AnyMarkup): ReporterNamespace {
		return {
			success: (suffix) => this.success(markup`${prefix} ${suffix}`),
			info: (suffix) => this.info(markup`${prefix} ${suffix}`),
			error: (suffix) => this.error(markup`${prefix} ${suffix}`),
			warn: (suffix) => this.warn(markup`${prefix} ${suffix}`),
			log: (suffix) => this.log(markup`${prefix} ${suffix}`),
			list: (items, opts = {}) => {
				const suffix = opts.prefix ?? markup``;
				this.list(
					items,
					{
						...opts,
						prefix: markup`${prefix} ${suffix}`,
					},
				);
			},
		};
	}

	public processedList<T>(
		items: Array<T>,
		callback: (reporter: Reporter, item: T) => void | AnyMarkup,
		opts: ReporterListOptions = {},
	): {
		truncated: boolean;
	} {
		if (items.length === 0) {
			// We make some assumptions that there's at least one item
			return {truncated: false};
		}

		let truncatedCount = 0;

		let start = opts.start || 0;
		if (opts.truncate !== undefined && items.length > opts.truncate) {
			truncatedCount = items.length - opts.truncate;
			items = items.slice(0, opts.truncate);
			start += truncatedCount;
		}

		let buff: AnyMarkup = markup``;

		for (let i = 0; i < items.length; i++) {
			const item = items[i];
			const reporter = this.fork({
				streams: [],
			});
			const stream = reporter.attachCaptureStream("markup");
			const str = callback(reporter, item);
			stream.remove();

			let inner =
				str === undefined
					? convertToMarkupFromRandomString(stream.read().trimRight())
					: str;
			if (opts.pad && i !== items.length - 1) {
				inner = markup`${inner}\n`;
			}
			buff = markup`${buff}<li>${inner}</li>`;
		}

		if (opts.ordered) {
			buff = markupTag("ol", buff, {start, reversed: opts.reverse});
		} else {
			buff = markup`<ul>${buff}</ul>`;
		}

		if (truncatedCount > 0) {
			buff = markup`${buff}<dim>and ${truncatedCount} others...</dim>`;
		}

		if (opts.prefix !== undefined) {
			buff = markup`${opts.prefix}${buff}`;
		}

		this.log(buff);

		return {truncated: truncatedCount > 0};
	}

	public list(items: AnyMarkups, opts: ReporterListOptions = {}) {
		return this.processedList(
			items,
			(reporter, str) => {
				return str;
			},
			opts,
		);
	}

	public progress(
		opts?: ReporterProgressOptions,
		onEnd?: VoidCallback,
	): ReporterProgress {
		const bar = new Progress(
			this,
			opts,
			() => {
				this.activeElements.delete(bar);
				if (onEnd !== undefined) {
					onEnd();
				}
			},
		);
		this.activeElements.add(bar);
		return bar;
	}
}

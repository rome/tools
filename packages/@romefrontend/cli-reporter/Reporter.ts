/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Markup,
	MarkupFormatOptions,
	MarkupTagName,
	UserMarkupFormatGridOptions,
	ansiEscapes,
	concatMarkup,
	convertToMarkupFromRandomString,
	isEmptyMarkup,
	markup,
	markupTag,
	markupToAnsi,
	markupToPlainText,
} from "@romefrontend/cli-layout";
import {
	RemoteReporterClientMessage,
	RemoteReporterReceiveMessage as RemoteReporterServerMessage,
	ReporterConditionalStream,
	ReporterDerivedStreams,
	ReporterProgress,
	ReporterProgressOptions,
	ReporterStream,
	ReporterStreamMeta,
	SelectArguments,
	SelectOptions,
} from "./types";
import Progress from "./Progress";
import prettyFormat from "@romefrontend/pretty-format";
import stream = require("stream");
import {CWD_PATH} from "@romefrontend/path";
import {Event} from "@romefrontend/events";
import readline = require("readline");
import select from "./select";
import {onKeypress} from "./util";
import {
	joinMarkupLines,
	markupToHtml,
	normalizeMarkup,
} from "@romefrontend/cli-layout/format";
import {
	DEFAULT_TERMINAL_FEATURES,
	Stdout,
	TerminalFeatures,
	inferTerminalFeatures,
} from "@romefrontend/cli-environment";

type ListOptions = {
	reverse?: boolean;
	truncate?: number;
	ordered?: boolean;
	start?: number;
};

// rome-ignore lint/js/noExplicitAny
type WrapperFactory = <T extends (...args: Array<any>) => any>(callback: T) => T;

export type ReporterOptions = {
	streams?: Array<ReporterStream>;
	stdin?: NodeJS.ReadStream;
	programName?: string;
	hasClearScreen?: boolean;
	programVersion?: string;
	markupOptions?: MarkupFormatOptions;
	useRemoteProgressBars?: boolean;
	startTime?: number;
	wrapperFactory?: WrapperFactory;
};

export type LogOptions = {
	stderr?: boolean;
	newline?: boolean;
};

export type LogCategoryOptions = LogOptions & {
	unicodePrefix: string;
	rawPrefix: string;
	markupTag: MarkupTagName;
};

type QuestionValidateResult<T> = [false, Markup] | [true, T];

type QuestionOptions = {
	hint?: string;
	default?: string;
	yes?: boolean;
	normalize?: (value: string) => string;
};

let remoteProgressIdCounter = 0;

export default class Reporter {
	constructor(opts: ReporterOptions = {}) {
		this.programName =
			opts.programName === undefined ? "rome" : opts.programName;
		this.programVersion = opts.programVersion;
		this.startTime = opts.startTime === undefined ? Date.now() : opts.startTime;
		this.hasClearScreen =
			opts.hasClearScreen === undefined ? true : opts.hasClearScreen;
		this.activeElements = new Set();
		this.indentLevel = 0;
		this.markupOptions =
			opts.markupOptions === undefined ? {} : opts.markupOptions;
		this.streamsWithDoubleNewlineEnd = new Set();
		this.streamsWithNewlineEnd = new Set();
		this.shouldRedirectOutToErr = false;
		this.stdin = opts.stdin;

		this.wrapperFactory = opts.wrapperFactory;

		this.remoteClientProgressBars = new Map();
		this.remoteServerProgressBars = new Map();

		this.sendRemoteServerMessage = new Event({
			name: "sendRemoteServerMessage",
		});
		this.sendRemoteClientMessage = new Event({
			name: "sendRemoteClientMessage",
		});

		this.isRemote = opts.useRemoteProgressBars === true;

		this.outStreams = new Set();
		this.errStreams = new Set();
		this.streams = new Set();

		if (opts.streams !== undefined) {
			for (const stream of opts.streams) {
				this.addStream(stream);
			}
		}
	}

	attachStdoutStreams(
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

		const stdoutWrite: ReporterDerivedStreams["stdoutWrite"] = (chunk) => {
			if (stdout !== undefined) {
				// @ts-ignore
				stdout.write(chunk);
			}
		};

		const stderrWrite: ReporterDerivedStreams["stderrWrite"] = (chunk) => {
			if (stderr !== undefined) {
				// @ts-ignore
				stderr.write(chunk);
			}
		};

		setupUpdateEvent();

		let outStream: ReporterStream = {
			type: "out",
			format,
			features,
			write: stdoutWrite,
			init: setupUpdateEvent,
			teardown: closeUpdateEvent,
		};

		let errStream: ReporterStream = {
			type: "error",
			format,
			features,
			write: stderrWrite,
		};

		this.addStream(outStream);
		this.addStream(errStream);

		updateEvent.subscribe((features) => {
			[outStream, errStream] = this.updateStreamsFeatures(
				[outStream, errStream],
				features,
			);
		});

		return {
			format,
			features,
			featuresUpdated: updateEvent,
			stdoutWrite,
			stderrWrite,
		};
	}

	attachConditionalStream(
		stream: ReporterStream,
		check: () => boolean,
	): ReporterConditionalStream {
		const cond: ReporterConditionalStream = {
			update: () => {
				if (check()) {
					if (!this.streams.has(stream)) {
						this.addStream(stream);
					}
					return true;
				} else {
					if (this.streams.has(stream)) {
						this.removeStream(stream);
					}
					return false;
				}
			},
		};

		cond.update();

		return cond;
	}

	attachCaptureStream(
		format: ReporterStream["format"] = "none",
		features: Partial<TerminalFeatures> = {},
	): {
		read: () => string;
		remove: () => void;
	} {
		let buff = "";

		const stream: ReporterStream = {
			format,
			type: "all",
			features: {
				...DEFAULT_TERMINAL_FEATURES,
				...features,
			},
			write(chunk) {
				buff += chunk;
			},
		};

		this.addStream(stream);

		return {
			read() {
				return buff;
			},
			remove: () => {
				this.removeStream(stream);
			},
		};
	}

	static fromProcess(opts: ReporterOptions = {}): Reporter {
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

	programName: string;
	programVersion: string | undefined;
	markupOptions: MarkupFormatOptions;

	isRemote: boolean;
	streamsWithNewlineEnd: Set<ReporterStreamMeta>;
	streamsWithDoubleNewlineEnd: Set<ReporterStreamMeta>;
	indentLevel: number;
	startTime: number;
	shouldRedirectOutToErr: boolean;
	wrapperFactory: undefined | WrapperFactory;
	outStreams: Set<ReporterStream>;
	errStreams: Set<ReporterStream>;
	streams: Set<ReporterStream>;
	sendRemoteServerMessage: Event<RemoteReporterServerMessage, void>;
	sendRemoteClientMessage: Event<RemoteReporterClientMessage, void>;
	stdin: undefined | NodeJS.ReadStream;

	remoteClientProgressBars: Map<string, Progress>;
	remoteServerProgressBars: Map<
		string,
		{
			end: () => void;
		}
	>;

	// track whether we've output anything, we need this to avoid outputting multiple spacers etc
	hasClearScreen: boolean;

	//Store active progress indicators so we can easily bail out and destroy them
	activeElements: Set<{
		render: () => void;
		end: () => void;
	}>;

	processRemoteClientMessage(msg: RemoteReporterClientMessage) {
		if (msg.type === "PROGRESS_CREATE") {
			this.remoteClientProgressBars.set(
				msg.id,
				this.progressLocal(
					msg.opts,
					() => {
						this.sendRemoteServerMessage.call({
							type: "ENDED",
							id: msg.id,
						});
					},
				),
			);
			return;
		}

		let bar = this.remoteClientProgressBars.get(msg.id);
		if (bar === undefined) {
			throw new Error(
				`Remote reporter message for progress bar ${msg.id} that does not exist`,
			);
		}

		bar.processRemoteClientMessage(msg);

		if (msg.type === "PROGRESS_END") {
			this.remoteClientProgressBars.delete(msg.id);
		}
	}

	receivedRemoteServerMessage(msg: RemoteReporterServerMessage) {
		// Currently the only message a remote Reporter can send is that it has ended
		switch (msg.type) {
			case "ENDED": {
				const progress = this.remoteServerProgressBars.get(msg.id);
				if (progress !== undefined) {
					progress.end();
				}
			}
		}
	}

	getMessagePrefix(): string {
		return "";
	}

	redirectOutToErr(should: boolean): boolean {
		const old = this.shouldRedirectOutToErr;
		this.shouldRedirectOutToErr = should;
		return old;
	}

	updateStreamsFeatures(
		streams: Array<ReporterStream>,
		features: TerminalFeatures,
		format?: ReporterStream["format"],
	): Array<ReporterStream> {
		const newStreams: Array<ReporterStream> = streams.map((stream) => {
			this.removeStream(stream);
			const newStream: ReporterStream = {
				...stream,
				format: format === undefined ? stream.format : format,
				features,
			};
			this.addStream(stream);
			return newStream;
		});
		this.refreshActiveElements();
		return newStreams;
	}

	refreshActiveElements() {
		for (const elem of this.activeElements) {
			elem.render();
		}
	}

	addStream(stream: ReporterStream) {
		if (stream.init !== undefined) {
			stream.init();
		}

		this.streamsWithNewlineEnd.add(stream);
		this.streams.add(stream);

		if (stream.type === "error" || stream.type === "all") {
			this.errStreams.add(stream);
		}

		if (stream.type === "out" || stream.type === "all") {
			this.outStreams.add(stream);
		}
	}

	removeStream(stream: ReporterStream) {
		if (stream.teardown !== undefined) {
			stream.teardown();
		}
		this.streams.delete(stream);
		this.outStreams.delete(stream);
		this.errStreams.delete(stream);
	}

	//# Stdin
	getStdin(): NodeJS.ReadStream {
		const {stdin} = this;
		if (stdin === undefined) {
			throw new Error("This operation expected a stdin but we have none");
		}
		return stdin;
	}

	async question(
		message: Markup,
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
		this.logAll(
			prompt,
			{
				newline: false,
			},
		);

		const rl = readline.createInterface({
			input: stdin,
			output: new stream.Writable({
				write: (chunk, encoding, callback) => {
					this.writeAll(chunk);
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
					this.writeAll(ansiEscapes.cursorUp());
					this.writeAll(ansiEscapes.eraseLine);

					let prompt = origPrompt;
					prompt = markup`${prompt}: `;
					if (normalized === "") {
						prompt = markup`${prompt}<dim>empty</dim>`;
					} else {
						prompt = markup`${prompt}${normalized}`;
					}
					this.logAll(prompt);

					resolve(normalized);
				},
			);
		});
	}

	async questionValidate<T>(
		message: Markup,
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
				continue;
			} else {
				return res[1];
			}
		}
	}

	async radioConfirm(message: Markup): Promise<boolean> {
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

	async confirm(message: string = "Press any key to continue"): Promise<void> {
		this.logAll(markup`<dim>${message}</dim>`, {newline: false});

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
		this.logAll(markup``);
	}

	async radio<Options extends SelectOptions>(
		message: Markup,
		arg: SelectArguments<Options>,
	): Promise<keyof Options> {
		const set = await this.select(message, {...arg, radio: true});

		// Should always have at least one element
		return Array.from(set)[0];
	}

	async select<Options extends SelectOptions>(
		message: Markup,
		args: SelectArguments<Options>,
	): Promise<Set<keyof Options>> {
		return select(this, message, args);
	}

	//# Control

	getAllStreams(): Set<ReporterStream> {
		return this.streams;
	}

	getStreams(stderr: undefined | boolean): Set<ReporterStream> {
		if (this.shouldRedirectOutToErr) {
			return this.errStreams;
		}

		if (stderr) {
			return this.errStreams;
		}

		return this.outStreams;
	}

	//# LIFECYCLE
	teardown() {
		for (const stream of this.streams) {
			this.removeStream(stream);
		}

		for (const elem of this.activeElements) {
			elem.end();
		}
		this.activeElements.clear();
	}

	fork(opts: Partial<ReporterOptions> = {}) {
		return new Reporter({
			streams: [...this.streams],
			markupOptions: this.markupOptions,
			wrapperFactory: this.wrapperFactory,
			...opts,
		});
	}

	//# INDENTATION METHODS
	indent(callback: () => void) {
		this.indentLevel++;

		try {
			callback();
		} finally {
			this.indentLevel--;
		}
	}

	noIndent(callback: () => void) {
		const prevIndentLevel = this.indentLevel;
		this.indentLevel = 0;

		try {
			callback();
		} finally {
			this.indentLevel = prevIndentLevel;
		}
	}

	table(head: Array<Markup>, rawBody: Array<Array<Markup>>) {
		let body: Array<Markup> = [];

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

		this.logAll(markup`<table>${concatMarkup(body)}</table>`);
	}

	inspect(value: unknown) {
		const streams = this.getStreams(false);
		if (streams.size === 0) {
			return;
		}

		let formatted;
		if (typeof value !== "number" && typeof value !== "string") {
			formatted = prettyFormat(value);
		} else {
			formatted = markup`${String(value)}`;
		}

		for (const stream of streams) {
			this.logOne(stream, formatted);
		}
	}

	//# ESCAPE HATCHES
	clearLineAll() {
		for (const stream of this.getStreams(false)) {
			this.clearLineSpecific(stream);
		}
	}

	clearLineSpecific(stream: ReporterStream) {
		if (stream.format === "ansi" && stream.features.cursor) {
			stream.write(ansiEscapes.eraseLine);
			stream.write(ansiEscapes.cursorTo(0));
		}
	}

	writeAll(msg: string, opts: LogOptions = {}) {
		for (const stream of this.getStreams(opts.stderr)) {
			this.writeSpecific(stream, msg, opts);
		}
	}

	writeSpecific(stream: ReporterStream, msg: string, opts: LogOptions) {
		this.hasClearScreen = false;

		if (this.activeElements.size > 0 && opts.newline) {
			// A progress bar is active and has probably drawn to the screen
			this.clearLineSpecific(stream);
		}

		stream.write(msg);
	}

	//# UTILITIES
	getTotalTime(): number {
		return Date.now() - this.startTime;
	}

	clearScreen() {
		for (const stream of this.getStreams(false)) {
			if (stream.format === "ansi" && stream.features.cursor) {
				stream.write(ansiEscapes.clearScreen);
			}
		}
		this.hasClearScreen = true;
	}

	//# SECTIONS
	heading(text: Markup) {
		this.br();
		this.logAll(markup`<inverse><emphasis> ${text} </emphasis></inverse>`);
		this.br();
	}

	section(title: undefined | Markup, callback: () => void) {
		this.hr(
			title === undefined ? undefined : markup`<emphasis>${title}</emphasis>`,
		);
		this.indent(() => {
			callback();
			this.br();
		});
	}

	hr(text: Markup = markup``) {
		const {hasClearScreen} = this;

		this.br();

		if (hasClearScreen && text === undefined) {
			return;
		}

		this.logAll(markup`<hr>${text}</hr>`);
		this.br();
	}

	async steps(
		callbacks: Array<{
			message: Markup;
			callback: () => Promise<void>;
			clear?: boolean;
		}>,
	) {
		const total = callbacks.length;
		let current = 1;
		for (const {clear, message, callback} of callbacks) {
			this.step(current, total, message);

			if (clear) {
				this.hasClearScreen = true;
			}

			await callback();
			current++;

			// If a step doesn't produce any output, or just progress bars that are cleared, we can safely remove the previous `step` message line
			if (clear && this.hasClearScreen) {
				for (const stream of this.getStreams(false)) {
					if (stream.format === "ansi" && stream.features.cursor) {
						stream.write(ansiEscapes.cursorTo(0));
						stream.write(ansiEscapes.cursorUp());
						stream.write(ansiEscapes.eraseLine);
					}
				}
			}
		}
	}

	step(current: number, total: number, msg: Markup) {
		this.logAll(markup`<dim>[${String(current)}/${String(total)}]</dim> ${msg}`);
	}

	br(force: boolean = false) {
		for (const stream of this.getStreams(false)) {
			if (!this.streamsWithDoubleNewlineEnd.has(stream) || force) {
				this.logOne(stream, markup``);
			}
		}
	}

	wrapCallback: WrapperFactory = (callback) => {
		const {wrapperFactory} = this;
		if (wrapperFactory === undefined) {
			return callback;
		} else {
			return wrapperFactory(callback);
		}
	};

	stripMarkup(str: Markup): string {
		return joinMarkupLines(markupToPlainText(str, this.markupOptions));
	}

	format(stream: ReporterStreamMeta, str: Markup): Array<string> {
		if (isEmptyMarkup(str)) {
			return [""];
		}

		const prefix = this.getMessagePrefix();
		let built = prefix === "" ? str : markup`${prefix}<view>${str}</view>`;

		const shouldIndent =
			this.indentLevel > 0 && this.streamsWithNewlineEnd.has(stream);

		if (shouldIndent) {
			for (let i = 0; i < this.indentLevel; i++) {
				built = markup`<indent>${built}</indent>`;
			}
		}

		const gridMarkupOptions: UserMarkupFormatGridOptions = {
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
				return [normalizeMarkup(built, this.markupOptions).text.value];
		}
	}

	logAll(msg: Markup, opts: LogOptions = {}) {
		for (const stream of this.getStreams(opts.stderr)) {
			this.logOne(stream, msg, opts);
		}
	}

	logAllRaw(msg: string, opts: LogOptions = {}) {
		for (const stream of this.getStreams(opts.stderr)) {
			this.logOneRaw(stream, msg, opts);
		}
	}

	logOne(stream: ReporterStream, msg: Markup, opts: LogOptions = {}) {
		const lines = this.format(stream, msg);
		for (let i = 0; i < lines.length; i++) {
			this.logOneRaw(
				stream,
				lines[i],
				{
					...opts,
					newline: i === lines.length - 1 ? opts.newline : true,
				},
			);
		}
	}

	logOneRaw(stream: ReporterStream, msg: string, opts: LogOptions = {}) {
		if (opts.newline !== false) {
			msg += "\n";
		}

		// Track if there's going to be a completely empty line
		const hasDoubleNewline = msg === "\n" || msg.endsWith("\n\n");
		if (hasDoubleNewline) {
			this.streamsWithDoubleNewlineEnd.add(stream);
		} else {
			this.streamsWithDoubleNewlineEnd.delete(stream);
		}
		if (msg.endsWith("\n")) {
			this.streamsWithNewlineEnd.add(stream);
		} else {
			this.streamsWithNewlineEnd.delete(stream);
		}

		this.writeSpecific(stream, msg, opts);
	}

	logAllWithCategory(rawInner: Markup, opts: LogCategoryOptions) {
		const streams = this.getStreams(opts.stderr);
		if (streams.size === 0) {
			return;
		}

		let inner = markupTag(opts.markupTag, rawInner);

		for (const stream of streams) {
			const prefixInner = stream.features.unicode
				? opts.unicodePrefix
				: opts.rawPrefix;
			const prefix = markupTag(
				"emphasis",
				markupTag(opts.markupTag, markup`${prefixInner}`),
			);
			const prefixedInner = markup`${prefix}<view>${inner}</view>`;
			this.logOne(stream, prefixedInner);
		}
	}

	success(msg: Markup) {
		this.logAllWithCategory(
			msg,
			{
				unicodePrefix: "\u2714 ",
				rawPrefix: "\u221a ",
				markupTag: "success",
			},
		);
	}

	error(msg: Markup) {
		this.logAllWithCategory(
			msg,
			{
				markupTag: "error",
				unicodePrefix: "\u2716 ",
				rawPrefix: "\xd7 ",
				stderr: true,
			},
		);
	}

	errorObj(err: Error) {
		this.error(
			markup`${err.stack || err.message || err.name || "Unknown Error"}`,
		);
	}

	info(msg: Markup) {
		this.logAllWithCategory(
			msg,
			{
				unicodePrefix: "\u2139 ",
				rawPrefix: "i ",
				markupTag: "info",
			},
		);
	}

	warn(msg: Markup) {
		this.logAllWithCategory(
			msg,
			{
				unicodePrefix: "\u26a0 ",
				rawPrefix: "! ",
				markupTag: "warn",
				stderr: true,
			},
		);
	}

	command(command: string) {
		this.logAll(markup`<dim>$ ${command}</dim>`);
	}

	processedList<T>(
		items: Array<T>,
		callback: (reporter: Reporter, item: T) => void | Markup,
		opts: ListOptions = {},
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

		let buff = markup``;

		for (const item of items) {
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
			buff = markup`${buff}<li>${inner}</li>`;
		}

		if (opts.ordered) {
			this.logAll(markupTag("ol", buff, {start, reversed: opts.reverse}));
		} else {
			this.logAll(markup`<ul>${buff}</ul>`);
		}

		if (truncatedCount > 0) {
			this.logAll(markup`<dim>and ${truncatedCount} others...</dim>`);
			return {truncated: true};
		} else {
			return {truncated: false};
		}
	}

	list(items: Array<Markup>, opts: ListOptions = {}) {
		return this.processedList(
			items,
			(reporter, str) => {
				return str;
			},
			opts,
		);
	}

	progress(opts?: ReporterProgressOptions): ReporterProgress {
		if (this.isRemote) {
			return this.progressRemote(opts);
		} else {
			return this.progressLocal(opts);
		}
	}

	progressLocal(opts?: ReporterProgressOptions, onEnd?: () => void): Progress {
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

	progressRemote(opts?: ReporterProgressOptions): ReporterProgress {
		const id: string = `${process.pid}:${remoteProgressIdCounter++}`;

		this.sendRemoteClientMessage.send({
			type: "PROGRESS_CREATE",
			opts,
			id,
		});

		let closed = false;

		const dispatch = (message: RemoteReporterClientMessage) => {
			if (!closed) {
				this.sendRemoteClientMessage.send(message);
			}
		};

		const end = () => {
			this.activeElements.delete(progress);
			this.remoteServerProgressBars.delete(id);
			closed = true;
		};

		let textIdCounter = 0;

		const progress: ReporterProgress = {
			render() {
				// Don't do anything
				// This is called when columns have updated and we want to force a rerender
			},
			setCurrent: (current: number) => {
				dispatch({
					type: "PROGRESS_SET_CURRENT",
					current,
					id,
				});
			},
			setTotal: (total: number, approximate: boolean = false) => {
				dispatch({
					type: "PROGRESS_SET_TOTAL",
					total,
					approximate,
					id,
				});
			},
			setText: (text: Markup) => {
				dispatch({
					type: "PROGRESS_SET_TEXT",
					text,
					id,
				});
			},
			setApproximateETA: (duration: number) => {
				dispatch({
					type: "PROGRESS_SET_APPROXIMATE_ETA",
					duration,
					id,
				});
			},
			pushText: (text: Markup, textId?: string) => {
				if (textId === undefined) {
					textId = String(textIdCounter++);
				}
				dispatch({
					type: "PROGRESS_PUSH_TEXT",
					textId,
					text,
					id,
				});
				return textId;
			},
			popText: (textId: string) => {
				dispatch({
					type: "PROGRESS_POP_TEXT",
					textId,
					id,
				});
			},
			tick: () => {
				dispatch({
					type: "PROGRESS_TICK",
					id,
				});
			},
			end: () => {
				dispatch({
					type: "PROGRESS_END",
					id,
				});
			},
			pause: () => {
				dispatch({
					type: "PROGRESS_PAUSE",
					id,
				});
			},
			resume: () => {
				dispatch({
					type: "PROGRESS_RESUME",
					id,
				});
			},
		};

		this.remoteServerProgressBars.set(
			id,
			{
				end,
			},
		);

		this.activeElements.add(progress);

		return progress;
	}
}

/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	MarkupFormatGridOptions,
	MarkupFormatOptions,
	MarkupLinesAndWidth,
	MarkupTagName,
	ansiEscapes,
	markupTag,
	markupToAnsi,
	markupToPlainText,
} from "@romefrontend/string-markup";
import {
	RemoteReporterClientMessage,
	RemoteReporterReceiveMessage as RemoteReporterServerMessage,
	ReporterConditionalStream,
	ReporterDerivedStreams,
	ReporterProgress,
	ReporterProgressOptions,
	ReporterStream,
	ReporterStreamMeta,
	ReporterTableField,
	SelectArguments,
	SelectOptions,
	Stdout,
} from "./types";
import {removeSuffix} from "@romefrontend/string-utils";
import Progress from "./Progress";
import prettyFormat from "@romefrontend/pretty-format";
import stream = require("stream");
import {CWD_PATH} from "@romefrontend/path";
import {Event} from "@romefrontend/events";
import readline = require("readline");
import select from "./select";
import {onKeypress, isCI} from "./util";
import {markupToHtml} from "@romefrontend/string-markup/format";

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
	verbose?: boolean;
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

type QuestionValidateResult<T> = [false, string] | [true, T];

type QuestionOptions = {
	hint?: string;
	default?: string;
	yes?: boolean;
	normalize?: (value: string) => string;
};

type MessagePrefix = {
	value: string;
	width: number;
};

const EMPTY_PREFIX: MessagePrefix = {
	value: "",
	width: 0,
};

let remoteProgressIdCounter = 0;

function getStreamFormat(stdout: undefined | Stdout): ReporterStream["format"] {
	if (stdout === undefined) {
		return "none";
	}

	if (stdout.isTTY || isCI()) {
		return "ansi";
	}

	return "none";
}

export default class Reporter {
	constructor(opts: ReporterOptions = {}) {
		this.programName =
			opts.programName === undefined ? "rome" : opts.programName;
		this.programVersion = opts.programVersion;

		this.noProgress = process.env.CI === "1";
		this.isVerbose = Boolean(opts.verbose);

		this.startTime = opts.startTime === undefined ? Date.now() : opts.startTime;
		this.hasClearScreen =
			opts.hasClearScreen === undefined ? true : opts.hasClearScreen;
		this.activeElements = new Set();
		this.indentLevel = 0;
		this.indentString = "";
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

		this.streamToMessagePrefix = new WeakMap();
		this.outStreams = new Set();
		this.errStreams = new Set();
		this.streams = new Set();

		if (opts.streams !== undefined) {
			for (const stream of opts.streams) {
				this.addStream(stream);
			}
		}
	}

	static DEFAULT_COLUMNS = 100;

	attachStdoutStreams(
		stdout?: Stdout,
		stderr?: Stdout,
		force: {
			columns?: number;
			format?: ReporterStream["format"];
		} = {},
	): ReporterDerivedStreams {
		let columns =
			stdout === undefined || stdout.columns === undefined
				? Reporter.DEFAULT_COLUMNS
				: stdout.columns;

		// Force set columns
		if (force.columns !== undefined) {
			columns = force.columns;
		}

		const columnsUpdated: Event<number, void> = new Event({
			name: "columnsUpdated",
		});

		// Windows terminals are awful
		const unicode =
			stdout !== undefined && stdout.unicode !== undefined
				? stdout.unicode
				: process.platform !== "win32";

		const outStream: ReporterStream = {
			type: "out",
			format: force.format || getStreamFormat(stdout),
			columns,
			unicode,
			write(chunk) {
				if (stdout !== undefined) {
					stdout.write(chunk);
				}
			},
			teardown() {},
		};

		const errStream: ReporterStream = {
			type: "error",
			format: force.format || getStreamFormat(stderr),
			columns,
			unicode,
			write(chunk) {
				if (stderr !== undefined) {
					stderr.write(chunk);
				}
			},
		};

		// Watch for resizing, unless force.columns has been set and we'll consider it to be fixed
		if (
			outStream.format === "ansi" &&
			stdout !== undefined &&
			force.columns !== undefined
		) {
			const onStdoutResize = () => {
				if (stdout?.columns !== undefined) {
					const {columns} = stdout;
					columnsUpdated.send(columns);
					this.setStreamColumns([outStream, errStream], columns);
				}
			};

			outStream.teardown = () => {
				stdout.off("resize", onStdoutResize);
			};

			stdout.on("resize", onStdoutResize);
		}

		this.addStream(outStream);
		this.addStream(errStream);

		return {
			columnsUpdated,
			stdout: outStream,
			stderr: errStream,
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
	): {
		read: () => string;
		remove: () => void;
	} {
		let buff = "";

		const stream: ReporterStream = {
			format,
			type: "all",
			columns: Reporter.DEFAULT_COLUMNS,
			unicode: true,
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
	noProgress: boolean;
	isVerbose: boolean;
	streamsWithNewlineEnd: Set<ReporterStreamMeta>;
	streamsWithDoubleNewlineEnd: Set<ReporterStreamMeta>;
	indentLevel: number;
	indentString: string;
	startTime: number;
	shouldRedirectOutToErr: boolean;
	wrapperFactory: undefined | WrapperFactory;
	outStreams: Set<ReporterStream>;
	errStreams: Set<ReporterStream>;
	streams: Set<ReporterStream>;
	streamToMessagePrefix: WeakMap<ReporterStreamMeta, MessagePrefix>;
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

	// Lazily create and memoize a message prefix per stream
	getMessagePrefix(stream: ReporterStreamMeta): MessagePrefix {
		const cached = this.streamToMessagePrefix.get(stream);
		if (cached !== undefined) {
			return cached;
		}

		const rawPrefix = this._getMessagePrefix();
		const {lines, width} = this.format(stream, rawPrefix, {partial: true});
		if (lines.length !== 1) {
			throw new Error(
				`Expected only 1 line but got ${lines.length} for prefix string "${rawPrefix}" for stream ${stream.format}`,
			);
		}

		const prefix: MessagePrefix = {
			width,
			value: lines[0],
		};
		this.streamToMessagePrefix.set(stream, prefix);
		return prefix;
	}

	_getMessagePrefix(): string {
		return "";
	}

	redirectOutToErr(should: boolean): boolean {
		const old = this.shouldRedirectOutToErr;
		this.shouldRedirectOutToErr = should;
		return old;
	}

	setStreamColumns(streams: Array<ReporterStream>, columns: number) {
		for (const stream of streams) {
			if (!this.streams.has(stream)) {
				throw new Error(
					"Trying to setStreamColumns on a stream that isn't attached to this Reporter",
				);
			}

			stream.columns = columns;
		}

		for (const elem of this.activeElements) {
			elem.render();
		}
	}

	addStream(stream: ReporterStream) {
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
		message: string,
		{hint, default: def = "", yes = false}: QuestionOptions = {},
	): Promise<string> {
		if (yes) {
			return def;
		}

		const stdin = this.getStdin();

		const origPrompt = `<dim emphasis>?</dim> <emphasis>${message}</emphasis>`;
		let prompt = origPrompt;
		if (hint !== undefined) {
			prompt += ` <dim>${hint}</dim>`;
		}
		if (def !== "") {
			prompt += ` (${def})`;
		}
		prompt += ": ";
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
					prompt += ": ";
					if (normalized === "") {
						prompt += "<dim>empty</dim>";
					} else {
						prompt += normalized;
					}
					this.logAll(prompt);

					resolve(normalized);
				},
			);
		});
	}

	async questionValidate<T>(
		message: string,
		validate: (value: string) => QuestionValidateResult<T>,
		options: Omit<QuestionOptions, "normalize"> = {},
	): Promise<T> {
		while (true) {
			let res: undefined | QuestionValidateResult<T>;

			await this.question(
				`${message}`,
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

	async radioConfirm(message: string): Promise<boolean> {
		const answer = await this.radio(
			message,
			{
				options: {
					yes: {
						label: "Yes",
						shortcut: "y",
					},
					no: {
						label: "No",
						shortcut: "n",
					},
				},
			},
		);
		return answer === "yes";
	}

	async confirm(message: string = "Press any key to continue"): Promise<void> {
		this.logAll(`<dim>${message}</dim>`, {newline: false});

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
		this.logAll("");
	}

	async radio<Options extends SelectOptions>(
		message: string,
		arg: SelectArguments<Options>,
	): Promise<keyof Options> {
		const set = await this.select(message, {...arg, radio: true});

		// Should always have at least one element
		return Array.from(set)[0];
	}

	async select<Options extends SelectOptions>(
		message: string,
		args: SelectArguments<Options>,
	): Promise<Set<keyof Options>> {
		return select(this, message, args);
	}

	//# Control

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
			verbose: this.isVerbose,
			markupOptions: this.markupOptions,
			wrapperFactory: this.wrapperFactory,
			...opts,
		});
	}

	//# INDENTATION METHODS
	indent(callback: () => void) {
		this.indentLevel++;
		this.updateIndent();

		callback();
		this.indentLevel--;
		this.updateIndent();
	}

	noIndent(callback: () => void) {
		const prevIndentLevel = this.indentLevel;
		this.indentLevel = 0;
		this.updateIndent();
		callback();
		this.indentLevel = prevIndentLevel;
		this.updateIndent();
	}

	updateIndent() {
		this.indentString = "  ".repeat(this.indentLevel);
	}

	//# INTERNAL
	prependEmoji(
		stream: ReporterStream,
		msg: string,
		emoji: string,
		fallback?: string,
	): string {
		if (stream.format === "none") {
			return `${emoji} ${msg}`;
		} else {
			if (fallback === undefined) {
				return msg;
			} else {
				return `${fallback} ${msg}`;
			}
		}
	}

	//# VISUALISATION
	table(
		head: Array<ReporterTableField>,
		rawBody: Array<Array<ReporterTableField>>,
	) {
		let body = "";

		if (head.length > 0) {
			body += "<tr>";
			for (const field of head) {
				body += `<td><emphasis>${field}</emphasis></td>`;
			}
			body += "</tr>";
		}

		for (const row of rawBody) {
			body += "<tr>";
			for (let field of row) {
				if (typeof field === "string" || typeof field === "number") {
					field = {align: "left", value: field};
				}

				let {value, align} = field;
				if (typeof value === "number") {
					value = `<number>${value}</number>`;
				}
				body += `<td align="${align}">${value}</td>`;
			}
			body += "</tr>";
		}

		this.logAll(`<table>${body}</table>`);
	}

	verboseInspect(val: unknown) {
		if (this.isVerbose) {
			this.inspect(val);
		}
	}

	inspect(value: unknown) {
		const streams = this.getStreams(false);
		if (streams.size === 0) {
			return;
		}

		let formatted = value;

		if (typeof formatted !== "number" && typeof formatted !== "string") {
			formatted = prettyFormat(formatted, {markup: true});
		}

		for (const stream of streams) {
			this.logOne(stream, String(formatted));
		}
	}

	//# ESCAPE HATCHES
	clearLineAll() {
		for (const stream of this.getStreams(false)) {
			this.clearLineSpecific(stream);
		}
	}

	clearLineSpecific(stream: ReporterStream) {
		if (stream.format === "ansi") {
			stream.write(ansiEscapes.eraseLine);
			stream.write(ansiEscapes.cursorTo(0));
		}
	}

	writeAll(msg: string, opts: LogOptions = {}) {
		for (const stream of this.getStreams(opts.stderr)) {
			this.writeSpecific(stream, msg);
		}
	}

	writeSpecific(stream: ReporterStream, msg: string) {
		this.hasClearScreen = false;

		if (stream.format === "ansi" && this.activeElements.size > 0) {
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
			if (stream.format === "ansi") {
				stream.write(ansiEscapes.clearScreen);
			}
		}
		this.hasClearScreen = true;
	}

	//# SECTIONS
	heading(text: string) {
		this.br();
		this.logAll(`<inverse><emphasis>${text}</emphasis></inverse>`);
		this.br();
	}

	section(title: undefined | string, callback: () => void) {
		this.hr(title === undefined ? undefined : `<emphasis>${title}</emphasis>`);
		this.indent(() => {
			callback();
			this.br();
		});
	}

	hr(text: string = "") {
		const {hasClearScreen} = this;

		this.br();

		if (hasClearScreen && text === undefined) {
			return;
		}

		this.logAll(`<hr>${text}</hr>`);
		this.br();
	}

	async steps(
		callbacks: Array<{
			message: string;
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
					if (stream.format === "ansi") {
						stream.write(ansiEscapes.cursorTo(0));
						stream.write(ansiEscapes.cursorUp());
						stream.write(ansiEscapes.eraseLine);
					}
				}
			}
		}
	}

	step(current: number, total: number, msg: string) {
		if (msg.endsWith("?")) {
			msg = `${removeSuffix(msg, "?")}...?`;
		} else {
			msg += "...";
		}

		this.logAll(`<dim>[${current}/${total}]</dim> ${msg}`);
	}

	br(force: boolean = false) {
		for (const stream of this.getStreams(false)) {
			if (!this.streamsWithDoubleNewlineEnd.has(stream) || force) {
				this.logOne(stream, "");
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

	stripMarkup(str: string): string {
		return markupToPlainText(str, this.markupOptions).lines.join("\n");
	}

	format(
		stream: ReporterStreamMeta,
		str: string,
		{
			partial = false,
			leadingPrefix,
		}: {
			leadingPrefix?: MessagePrefix;
			partial?: boolean;
		} = {},
	): MarkupLinesAndWidth {
		if (str === "") {
			return {lines: [""], width: 0};
		}

		const allLinePrefix: MessagePrefix = partial
			? EMPTY_PREFIX
			: this.getMessagePrefix(stream);
		const viewportShrink = leadingPrefix === undefined ? 0 : leadingPrefix.width;

		const gridMarkupOptions: MarkupFormatGridOptions = {
			...this.markupOptions,
			columns: stream.columns -
			this.indentString.length -
			viewportShrink -
			allLinePrefix.width,
		};

		let res: MarkupLinesAndWidth;

		switch (stream.format) {
			case "ansi": {
				res = markupToAnsi(str, gridMarkupOptions);
				break;
			}

			case "html": {
				res = markupToHtml(str, gridMarkupOptions);
				break;
			}

			case "none": {
				res = markupToPlainText(str, gridMarkupOptions);
				break;
			}

			case "markup": {
				res = {
					width: 0,
					lines: [str],
				};
				break;
			}
		}

		const {indentString} = this;
		const shouldIndent =
			!partial && this.streamsWithNewlineEnd.has(stream) && indentString !== "";

		let prefixIncluded = false;

		const newLines = res.lines.map((line, i) => {
			if (leadingPrefix !== undefined) {
				if (i === 0) {
					line = `${leadingPrefix.value}${line}`;
				} else {
					line = `${" ".repeat(leadingPrefix.width)}${line}`;
				}
			}

			if (shouldIndent && line !== "") {
				line = `${indentString}${line}`;
			}

			prefixIncluded = true;
			return `${allLinePrefix.value}${line}`;
		});

		return {
			width: prefixIncluded ? res.width + allLinePrefix.width : res.width,
			lines: newLines,
		};
	}

	logAll(tty: string, opts: LogOptions = {}) {
		for (const stream of this.getStreams(opts.stderr)) {
			this.logOne(stream, tty, opts);
		}
	}

	logAllRaw(tty: string, opts: LogOptions = {}) {
		for (const stream of this.getStreams(opts.stderr)) {
			this.logOneRaw(stream, tty, opts);
		}
	}

	logOne(stream: ReporterStream, msg: string, opts: LogOptions = {}) {
		const {lines} = this.format(stream, msg);
		for (const line of lines) {
			this.logOneRaw(stream, line, opts);
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

		this.writeSpecific(stream, msg);
	}

	logAllWithCategory(
		rawInner: string,
		args: Array<unknown>,
		opts: LogCategoryOptions,
	) {
		const streams = this.getStreams(opts.stderr);
		if (streams.size === 0) {
			return;
		}

		let inner = markupTag(opts.markupTag, rawInner);

		if (args.length > 0) {
			const formattedArgs: Array<string> = args.map((arg) => {
				if (typeof arg === "string") {
					return arg;
				} else {
					return prettyFormat(arg, {markup: true});
				}
			});

			// Interpolate
			inner = inner.replace(/%s/g, () => formattedArgs.shift()!);

			// Add on left over arguments
			for (const arg of formattedArgs) {
				if (inner[inner.length - 1] !== " ") {
					inner += " ";
				}

				inner += arg;
			}
		}

		for (const stream of streams) {
			// Format the prefix, selecting it depending on if we're a unicode stream
			const prefixInner = stream.unicode ? opts.unicodePrefix : opts.rawPrefix;
			const prefix = markupTag(
				"emphasis",
				markupTag(opts.markupTag, prefixInner),
			);

			// Should only be one line
			const {lines: prefixLines, width: prefixWidth} = this.format(
				stream,
				prefix,
				{partial: true},
			);
			const prefixLine = prefixLines[0];
			if (prefixLines.length !== 1) {
				throw new Error(`Expected 1 prefix line but got ${prefixLines.length}`);
			}

			const {lines} = this.format(
				stream,
				inner,
				{
					leadingPrefix: {
						width: prefixWidth,
						value: prefixLine,
					},
				},
			);
			for (const line of lines) {
				this.logOneRaw(stream, line, opts);
			}
		}
	}

	success(msg: string, ...args: Array<unknown>) {
		this.logAllWithCategory(
			msg,
			args,
			{
				unicodePrefix: "\u2714 ",
				rawPrefix: "\u221a ",
				markupTag: "success",
			},
		);
	}

	error(msg: string, ...args: Array<unknown>) {
		this.logAllWithCategory(
			msg,
			args,
			{
				markupTag: "error",
				unicodePrefix: "\u2716 ",
				rawPrefix: "\xd7 ",
				stderr: true,
			},
		);
	}

	errorObj(err: Error) {
		this.error(err.stack || err.message || err.name || "Unknown Error");
	}

	info(msg: string, ...args: Array<unknown>) {
		this.logAllWithCategory(
			msg,
			args,
			{
				unicodePrefix: "\u2139 ",
				rawPrefix: "i ",
				markupTag: "info",
			},
		);
	}

	warn(msg: string, ...args: Array<unknown>) {
		this.logAllWithCategory(
			msg,
			args,
			{
				unicodePrefix: "\u26a0 ",
				rawPrefix: "! ",
				markupTag: "warn",
				stderr: true,
			},
		);
	}

	verbose(msg: string, ...args: Array<unknown>) {
		if (this.isVerbose) {
			this.verboseForce(msg, ...args);
		}
	}

	verboseForce(msg: string, ...args: Array<unknown>) {
		this.logAllWithCategory(
			msg,
			args,
			{
				unicodePrefix: "\u26a1 ",
				rawPrefix: "* ",
				markupTag: "dim",
			},
		);
	}

	command(command: string) {
		this.logAll(`<dim>$ ${command}</dim>`);
	}

	processedList<T>(
		items: Array<T>,
		callback: (reporter: Reporter, item: T) => void,
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

		let buff = "";

		for (const item of items) {
			const reporter = this.fork({
				streams: [],
			});
			const stream = reporter.attachCaptureStream("markup");
			callback(reporter, item);
			stream.remove();
			buff += `<li>${stream.read()}</li>`;
		}

		if (opts.ordered) {
			this.logAll(markupTag("ol", buff, {start, reversed: opts.reverse}));
		} else {
			this.logAll(`<ul>${buff}</ul>`);
		}

		if (truncatedCount > 0) {
			this.logAll(`<dim>and <number>${truncatedCount}</number> others...</dim>`);
			return {truncated: true};
		} else {
			return {truncated: false};
		}
	}

	list(items: Array<string>, opts: ListOptions = {}) {
		return this.processedList(
			items,
			(reporter, str) => {
				reporter.logAll(str, {newline: false});
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
			setText: (text: string) => {
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
			pushText: (text: string) => {
				dispatch({
					type: "PROGRESS_PUSH_TEXT",
					text,
					id,
				});
			},
			popText: (text: string) => {
				dispatch({
					type: "PROGRESS_POP_TEXT",
					text,
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

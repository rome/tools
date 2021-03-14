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
	Markup,
	MarkupFormatOptions,
	MarkupTagName,
	StaticMarkup,
	convertToMarkupFromRandomString,
	isEmptyMarkup,
	joinMarkup,
	markup,
	markupTag,
	normalizeMarkup,
	readMarkup,
	serializeLazyMarkup,
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
	ReporterStreamState,
	SelectArguments,
	SelectOptions,
	SelectOptionsKeys,
	WrapperFactory,
} from "./types";
import {Event} from "@internal/events";
import Progress from "./Progress";
import prettyFormat from "@internal/pretty-format";
import stream = require("stream");
import {CWD_PATH, HOME_PATH} from "@internal/path";
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
import {
	Resource,
	createResourceContainer,
	createResourceFromCallback,
	createResourceRoot,
} from "@internal/resources";

export type ReporterOptions = {
	shouldRedirectOutToErr?: boolean;
	streams?: ReporterStreamAttached[];
	stdin?: NodeJS.ReadStream;
	markupOptions?: MarkupFormatOptions;
	wrapperFactory?: WrapperFactory;
};

export type LogOptions = {
	streams?: ReporterStreamAttached[];
	stderr?: boolean;
	noNewline?: boolean;
};

export type LogCategoryUserOptions = Pick<LogOptions, "stderr" | "streams">;

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

let cachedFromProcess: undefined | Reporter;

export default class Reporter implements ReporterNamespace {
	constructor(name: string, opts: ReporterOptions = {}) {
		this.activeElements = new Set();
		this.indentLevel = 0;
		this.markupOptions = opts.markupOptions;
		this.shouldRedirectOutToErr = opts.shouldRedirectOutToErr ?? false;
		this.stdin = opts.stdin;
		this.wrapperFactory = opts.wrapperFactory;
		this.streams = new Set();

		this.resources = createResourceRoot(`Reporter<${name}>`);
		this[Symbol.toStringTag] = `Reporter<${name}>`;
		this.name = name;

		if (opts.streams !== undefined) {
			for (const stream of opts.streams) {
				this.addAttachedStream(stream);
			}
		}
	}

	public markupOptions: undefined | MarkupFormatOptions;
	public resources: Resource;
	public [Symbol.toStringTag]: string;

	private name: string;
	private indentLevel: number;
	private shouldRedirectOutToErr: boolean;
	private wrapperFactory: undefined | WrapperFactory;
	private streams: Set<ReporterStreamAttached>;
	private stdin: undefined | NodeJS.ReadStream;

	// Store active progress indicators so we can redraw them on feature updates
	private activeElements: Set<{
		render: VoidCallback;
	}>;

	public static fromProcess(unique: boolean = false): Reporter {
		if (cachedFromProcess !== undefined && !unique) {
			return cachedFromProcess;
		}

		const reporter = new Reporter(
			"Process",
			{
				markupOptions: {
					cwd: CWD_PATH,
					home: HOME_PATH,
				},
			},
		);

		if (!unique) {
			cachedFromProcess = reporter;
		}

		reporter.attachStdoutStreams(process.stdout, process.stderr);

		return reporter;
	}

	// Produce a new Reporter with all the streams of the input reporters. Streams will NOT be in sync.
	public static concat(reporters: Reporter[]): Reporter {
		const streams: Set<ReporterStreamAttached> = new Set();
		for (const reporter of reporters) {
			for (const stream of reporter.getStreams()) {
				streams.add(stream);
			}
		}

		return new Reporter(
			reporters.map((reporter) => reporter[Symbol.toStringTag]).join(" & "),
			{
				streams: Array.from(streams),
			},
		);
	}

	public attachStdoutStreams(
		stdout?: Stdout,
		stderr?: Stdout,
		force: Partial<TerminalFeatures> & {
			format?: ReporterStream["format"];
		} = {},
	): ReporterDerivedStreams {
		const {features, updateEvent, setupUpdateEvent} = inferTerminalFeatures(
			stdout,
			force,
		);

		const {format = features.colorDepth > 1 ? "ansi" : "none"} = force;

		const stream = this.addStream({
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
		});

		stream.resources.add(setupUpdateEvent());

		updateEvent.subscribe((features) => {
			stream.updateFeatures(features);
		});

		return {
			format,
			features,
			featuresUpdated: updateEvent,
			stream,
		};
	}

	public attachConditionalStream(
		stream: ReporterStream,
		check?: () => boolean,
	): ReporterConditionalStream {
		let attached: undefined | ReporterStreamAttached;

		const cond: ReporterConditionalStream = {
			enable: async () => {
				if (attached === undefined) {
					attached = this.addStream(stream);
				}
			},
			async disable() {
				if (attached !== undefined) {
					attached.resources.release();
					attached = undefined;
				}
			},
			async update() {
				if (check !== undefined) {
					if (check()) {
						await cond.enable();
					} else {
						await cond.disable();
					}
				}

				return stream !== undefined;
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
			resources: stream.resources,
		};
	}

	protected getMessagePrefix(): Markup {
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

	public addAttachedStream(stream: ReporterStreamAttached): void {
		stream.resources.add(
			createResourceFromCallback(
				"ReporterStreamHandle",
				() => {
					this.streams.delete(stream);
				},
			),
		);
		stream.featuresUpdated.subscribe(() => {
			this.refreshActiveElements();
		});
		this.streams.add(stream);
	}

	public addStream(
		unattached: ReporterStream,
		state?: Partial<ReporterStreamState>,
	): ReporterStreamAttached {
		const stream: ReporterStreamAttached = {
			...unattached,
			activeElements: new Set(),
			state: mergeObjects(streamUtils.createStreamState(), state),
			updateFeatures: async (newFeatures) => {
				stream.features = newFeatures;
				await stream.featuresUpdated.callOptional(newFeatures);
			},
			featuresUpdated: new Event("ReporterStream.featuresUpdated"),
			resources: createResourceContainer("ReporterStream"),
		};
		this.resources.add(stream);
		this.addAttachedStream(stream);
		return stream;
	}

	public getStdin(): NodeJS.ReadStream {
		const {stdin} = this;
		if (stdin === undefined) {
			throw new Error("This operation expected a stdin but we have none");
		}
		return stdin;
	}

	public async question(
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
			} else {
				return res[1];
			}
		}
	}

	public async radioConfirm(message: Markup): Promise<boolean> {
		const answer = await this.radio(
			message,
			{
				options: {
					no: {
						label: markup`No`,
						shortcut: "n",
					},
					yes: {
						label: markup`Yes`,
						shortcut: "y",
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

		await new Promise<void>((resolve) => {
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
		message: Markup,
		arg: SelectArguments<Options>,
	): Promise<SelectOptionsKeys<Options>> {
		const set = await this.select(message, {...arg, radio: true});

		// Should always have at least one element
		return Array.from(set)[0];
	}

	public async select<Options extends SelectOptions>(
		message: Markup,
		args: SelectArguments<Options>,
	): Promise<Set<SelectOptionsKeys<Options>>> {
		return select(this, message, args);
	}

	public hasStreams(opts?: LogCategoryUserOptions): boolean {
		if (opts?.streams !== undefined) {
			return opts.streams.length > 0;
		}

		return this.streams.size > 0;
	}

	public getStreams(
		opts?: LogCategoryUserOptions,
	): Iterable<ReporterStreamAttached> {
		if (opts?.streams !== undefined) {
			return opts.streams;
		}

		return this.streams;
	}

	public updateMarkupOptions(opts: MarkupFormatOptions) {
		this.markupOptions = {
			...this.markupOptions,
			...opts,
		};
	}

	public fork(opts: Partial<ReporterOptions> = {}) {
		return new Reporter(
			this.name,
			{
				streams: [...this.streams],
				markupOptions: this.markupOptions,
				wrapperFactory: this.wrapperFactory,
				...opts,
			},
		);
	}

	public table(head: Markup[], rawBody: Markup[][]) {
		let body: Markup[] = [];

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

		this.log(markup`<table>${joinMarkup(body)}</table>`);
	}

	public inspect(value: unknown, opts?: LogOptions) {
		if (!this.hasStreams(opts)) {
			return;
		}

		const formatted = markup`${prettyFormat(value)}`;

		for (const stream of this.getStreams(opts)) {
			this._logMarkup(stream, formatted, opts);
		}
	}

	public write(msg: string, stderr: boolean = false) {
		for (const stream of this.getStreams()) {
			stream.write(msg, stderr || this.shouldRedirectOutToErr);
		}
	}

	public clearScreen(opts?: LogCategoryUserOptions) {
		for (const stream of this.getStreams(opts)) {
			streamUtils.clearScreen(stream);
		}
	}

	public heading(text: Markup) {
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

	public hr(text: Markup = markup``, opts?: LogCategoryUserOptions) {
		this.br(opts);
		for (const stream of this.getStreams(opts)) {
			this._logMarkup(stream, markup`<hr>${text}</hr>`);
		}
		this.br(opts);
	}

	public async steps(callbacks: ReporterStepCallback[]) {
		let total = 0;
		let current = 1;

		const filteredCallbacks: ReporterStepCallback[] = [];
		for (const item of callbacks) {
			if (item.test === undefined || (await item.test())) {
				filteredCallbacks.push(item);
				total++;
			}
		}

		for (const {message, callback} of filteredCallbacks) {
			this.step(current, total, message);
			await callback();
			current++;
		}
	}

	public step(current: number, total: number, msg: Markup, opts?: LogOptions) {
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
		const force = opts?.force;
		for (const stream of this.getStreams(opts)) {
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

	public stripMarkup(str: Markup): string {
		return markupToJoinedPlainText(str, this.markupOptions);
	}

	private format(stream: ReporterStreamAttached, str: Markup): string[] {
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

			case "markup": {
				const lazy = serializeLazyMarkup(built);
				if (this.markupOptions === undefined) {
					return [readMarkup(lazy)];
				} else {
					return [readMarkup(normalizeMarkup(lazy, this.markupOptions).text)];
				}
			}
		}
	}

	public log(msg: Markup, opts: LogOptions = {}) {
		for (const stream of this.getStreams(opts)) {
			this._logMarkup(stream, msg, opts);
		}
	}

	public logRaw(msg: string, opts: LogOptions = {}) {
		opts = {
			...opts,
			stderr: opts.stderr || this.shouldRedirectOutToErr,
		};

		for (const stream of this.getStreams(opts)) {
			streamUtils.log(stream, msg.split("\n"), opts);
		}
	}

	private _logMarkup(
		stream: ReporterStreamAttached,
		msg: Markup,
		opts: LogOptions = {},
	) {
		const lines = this.format(stream, msg);
		streamUtils.log(
			stream,
			lines,
			{
				...opts,
				stderr: opts.stderr || this.shouldRedirectOutToErr,
				noNewline: opts.noNewline,
			},
		);
	}

	private logCategory(rawInner: Markup, opts: LogCategoryOptions) {
		if (!this.hasStreams(opts)) {
			return;
		}

		let inner = markupTag(opts.markupTag, rawInner);

		for (const stream of this.getStreams(opts)) {
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

	public success(msg: Markup, opts?: LogCategoryUserOptions) {
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

	public error(msg: Markup, opts?: LogOptions) {
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

	public info(msg: Markup, opts?: LogOptions) {
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

	public warn(msg: Markup, opts?: LogOptions) {
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
		let highlighted = joinMarkup(
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

	public namespace(...prefixes: Markup[]): ReporterNamespace {
		const prefix = joinMarkup(prefixes.map((prefix) => markup`[${prefix}]`));

		return {
			namespace: (...addPrefixes) => {
				return this.namespace(...prefixes, ...addPrefixes);
			},
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
		iterable: Iterable<T>,
		callback: (reporter: Reporter, item: T) => void | Markup,
		opts: ReporterListOptions = {},
	): {
		truncated: boolean;
	} {
		// Avoid the overhead if there's nobody listening
		if (!this.hasStreams()) {
			return {truncated: false};
		}

		let items: T[] = Array.from(iterable);

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

		let buff: Markup = markup``;

		for (let i = 0; i < items.length; i++) {
			const item = items[i];
			const reporter = this.fork({
				streams: [],
			});
			const stream = reporter.attachCaptureStream("markup");
			const str = callback(reporter, item);
			reporter.resources.release();

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

	public list(items: Iterable<Markup>, opts: ReporterListOptions = {}) {
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
			this.activeElements.size,
			opts,
			() => {
				this.activeElements.delete(bar);
				onEnd?.();
			},
		);

		this.activeElements.add(bar);
		this.resources.add(bar);
		return bar;
	}
}

/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticLanguage,
	DiagnosticLocation,
	DiagnosticSourceType,
	Diagnostics,
	DiagnosticsProcessor,
	deriveRootAdviceFromDiagnostic,
} from "@internal/diagnostics";
import {
	MarkupRGB,
	StaticMarkup,
	joinMarkupLines,
	markup,
} from "@internal/markup";
import {Reporter} from "@internal/cli-reporter";
import {
	DiagnosticsFileReaders,
	DiagnosticsPrinterFlags,
	DiagnosticsPrinterOptions,
} from "./types";
import {formatAnsiRGB, markupToPlainText} from "@internal/cli-layout";
import {ToLines, toLines} from "./utils";
import {printAdvice} from "./printAdvice";
import {default as successBanner} from "./banners/success.json";
import {default as errorBanner} from "./banners/error.json";
import {
	AbsoluteFilePath,
	CWD_PATH,
	UnknownPath,
	UnknownPathMap,
	UnknownPathSet,
	createUnknownPath,
} from "@internal/path";
import {Number0, Number1, ob1Get0, ob1Get1} from "@internal/ob1";
import {exists, lstat, readFileText} from "@internal/fs";

import {inferDiagnosticLanguageFromFilename} from "@internal/core/common/file-handlers";
import {markupToJoinedPlainText} from "@internal/cli-layout/format";

type RawBanner = {
	palettes: Array<MarkupRGB>;
	rows: Array<Array<number | MarkupRGB>>;
};

type PositionLike = {
	line?: undefined | Number1;
	column?: undefined | Number0;
};

const DEFAULT_FILE_READERS: DiagnosticsFileReaders = {
	async read(path) {
		if ((await exists(path)) && (await lstat(path)).isFile()) {
			return await readFileText(path);
		} else {
			return undefined;
		}
	},

	async getMtime(path) {
		if (await exists(path)) {
			const stats = await lstat(path);
			return stats.mtimeMs;
		} else {
			return undefined;
		}
	},
};

function equalPosition(
	a: undefined | PositionLike,
	b: undefined | PositionLike,
): boolean {
	if (a === undefined || b === undefined) {
		return false;
	}

	if (a.line !== b.line || a.column !== b.column) {
		return false;
	}

	return true;
}

type FooterPrintCallback = (reporter: Reporter, error: boolean) => Promise<void>;

export const DEFAULT_PRINTER_FLAGS: DiagnosticsPrinterFlags = {
	auxiliaryDiagnosticFormat: undefined,
	grep: "",
	inverseGrep: false,
	showAllDiagnostics: true,
	fieri: false,
	verboseDiagnostics: false,
	maxDiagnostics: 20,
};

// Dependency that may not be included in the output diagnostic but whose changes may effect the validity of this one
type ChangeFileDependency = {
	type: "change";
	path: UnknownPath;
	mtime: undefined | number;
};

// Dependency that will have a code frame in the output diagnostic
type ReferenceFileDependency = {
	type: "reference";
	path: UnknownPath;
	mtime: undefined | number;
	sourceTypeJS: undefined | DiagnosticSourceType;
	language: undefined | DiagnosticLanguage;
	sourceText: undefined | string;
};

type FileDependency = ChangeFileDependency | ReferenceFileDependency;

function hasFrame(loc: DiagnosticLocation): boolean {
	return loc.start !== undefined && loc.end !== undefined;
}

export type DiagnosticsPrinterFileSources = UnknownPathMap<{
	sourceText: string;
	lines: ToLines;
}>;

export type DiagnosticsPrinterFileMtimes = UnknownPathMap<number>;

export default class DiagnosticsPrinter extends Error {
	constructor(opts: DiagnosticsPrinterOptions) {
		super(
			"Diagnostics printer. If you're seeing this then it wasn't caught and printed correctly.",
		);
		const {cwd, reporter, flags = DEFAULT_PRINTER_FLAGS} = opts;

		this.options = opts;

		this.reporter = reporter;
		this.flags = flags;
		this.fileReaders =
			opts.fileReaders === undefined
				? [DEFAULT_FILE_READERS]
				: [opts.fileReaders, DEFAULT_FILE_READERS];
		this.cwd = cwd ?? CWD_PATH;
		this.processor = opts.processor;

		this.displayedCount = 0;
		this.problemCount = 0;
		this.filteredCount = 0;
		this.truncatedCount = 0;

		this.defaultFooterEnabled = true;
		this.hasTruncatedDiagnostics = false;
		this.missingFileSources = new UnknownPathSet();
		this.fileSources = new UnknownPathMap();
		this.fileMtimes = new UnknownPathMap();
		this.onFooterPrintCallbacks = [];
	}

	public processor: DiagnosticsProcessor;
	public flags: DiagnosticsPrinterFlags;
	public defaultFooterEnabled: boolean;

	private options: DiagnosticsPrinterOptions;
	private reporter: Reporter;
	private onFooterPrintCallbacks: Array<{
		callback: FooterPrintCallback;
		after: boolean;
	}>;
	private cwd: AbsoluteFilePath;
	private fileReaders: Array<DiagnosticsFileReaders>;
	private hasTruncatedDiagnostics: boolean;
	private missingFileSources: UnknownPathSet;
	private fileSources: DiagnosticsPrinterFileSources;
	private fileMtimes: DiagnosticsPrinterFileMtimes;

	private displayedCount: number;
	private problemCount: number;
	private filteredCount: number;
	private truncatedCount: number;

	public createFilePath(filename: string): UnknownPath {
		const {normalizePosition} = this.reporter.markupOptions;

		if (normalizePosition === undefined) {
			return createUnknownPath(filename);
		} else {
			return createUnknownPath(
				normalizePosition(filename, undefined, undefined).filename,
			);
		}
	}

	private getDisplayedProblemsCount() {
		return this.problemCount - this.filteredCount;
	}

	private shouldTruncate(): boolean {
		return (
			!this.flags.showAllDiagnostics &&
			this.displayedCount > this.flags.maxDiagnostics
		);
	}

	private shouldIgnore(diag: Diagnostic): boolean {
		const {grep, inverseGrep} = this.flags;

		// An empty grep pattern means show everything
		if (grep === undefined || grep === "") {
			return false;
		}

		// Match against the supplied grep pattern
		let ignored =
			markupToJoinedPlainText(diag.description.message).toLowerCase().includes(
				grep,
			) === false;
		if (inverseGrep) {
			ignored = !ignored;
		}
		return ignored;
	}

	// Only highlight if we have a reporter stream enabled that isn't format: "none"
	public shouldHighlight(): boolean {
		for (const {stream} of this.reporter.getStreamHandles()) {
			if (stream.format !== "none") {
				return true;
			}
		}
		return false;
	}

	private async addFileSource(
		dep: ChangeFileDependency | ReferenceFileDependency,
	) {
		const path = dep.path.assertAbsolute();

		let mtime;
		for (const reader of this.fileReaders) {
			if (mtime !== undefined) {
				break;
			}
			mtime = await reader.getMtime(path);
		}
		if (mtime === undefined) {
			this.missingFileSources.add(path);
			return;
		}

		this.fileMtimes.set(dep.path, mtime);

		if (dep.type === "reference") {
			let sourceText = dep.sourceText;
			for (const reader of this.fileReaders) {
				if (sourceText !== undefined) {
					break;
				}
				sourceText = await reader.read(path);
			}
			if (sourceText === undefined) {
				this.missingFileSources.add(path);
				return;
			}

			this.fileSources.set(
				dep.path,
				{
					sourceText,
					lines: toLines({
						highlight: this.shouldHighlight(),
						path: dep.path,
						input: sourceText,
						sourceTypeJS: dep.sourceTypeJS,
						language: inferDiagnosticLanguageFromFilename(
							dep.path,
							dep.language,
						),
					}),
				},
			);
		}
	}

	private getDependenciesFromDiagnostics(
		diagnostics: Diagnostics,
	): Array<FileDependency> {
		const deps: Array<FileDependency> = [];

		for (const diag of diagnostics) {
			const {
				dependencies,
				description: {advice},
				location: {language, sourceTypeJS, sourceText, mtime, filename},
			} = diag;

			if (filename !== undefined) {
				const path = this.createFilePath(filename);
				if (hasFrame(diag.location)) {
					deps.push({
						type: "reference",
						path,
						mtime,
						language,
						sourceTypeJS,
						sourceText,
					});
				} else {
					deps.push({
						type: "change",
						path,
						mtime,
					});
				}
			}

			if (dependencies !== undefined) {
				for (const {filename, mtime} of dependencies) {
					deps.push({
						type: "change",
						path: this.createFilePath(filename),
						mtime,
					});
				}
			}

			for (const item of advice) {
				if (item.type === "frame") {
					const {location} = item;
					if (location.filename !== undefined) {
						const path = this.createFilePath(location.filename);
						if (hasFrame(location)) {
							deps.push({
								type: "reference",
								path,
								language: location.language,
								sourceTypeJS: location.sourceTypeJS,
								mtime: location.mtime,
								sourceText: location.sourceText,
							});
						} else {
							deps.push({
								type: "change",
								path,
								mtime,
							});
						}
					}
				}

				if (item.type === "stacktrace") {
					for (const {filename, line, column, sourceText} of item.frames) {
						if (filename !== undefined) {
							const path = this.createFilePath(filename);
							if (line !== undefined && column !== undefined) {
								deps.push({
									type: "reference",
									path,
									language: undefined,
									sourceTypeJS: undefined,
									mtime: undefined,
									sourceText,
								});
							} else {
								deps.push({
									type: "change",
									path,
									mtime: undefined,
								});
							}
						}
					}
				}
			}
		}

		const depsMap: UnknownPathMap<FileDependency> = new UnknownPathMap();

		// Remove non-absolute filenames and normalize sourceType and language for conflicts
		for (const dep of deps) {
			const path = dep.path;
			if (!path.isAbsolute()) {
				continue;
			}

			const existing = depsMap.get(path);

			// "reference" dependency can override "change" since it has more metadata that needs conflict resolution
			if (existing === undefined || existing.type === "change") {
				depsMap.set(dep.path, dep);
				continue;
			}

			if (dep.type === "reference") {
				if (
					dep.sourceText !== undefined &&
					existing.sourceText !== undefined &&
					dep.sourceText !== existing.sourceText
				) {
					throw new Error(
						`Found multiple sourceText entires for ${dep.path.join()} that didn't match`,
					);
				}

				if (existing.sourceText === undefined) {
					existing.sourceText = dep.sourceText;
				}

				if (existing.sourceTypeJS !== dep.sourceTypeJS) {
					existing.sourceTypeJS = "unknown";
				}

				if (existing.language !== dep.language) {
					existing.language = "unknown";
				}
			}
		}

		return Array.from(depsMap.values());
	}

	public async fetchFileSources(diagnostics: Diagnostics) {
		for (const dep of this.getDependenciesFromDiagnostics(diagnostics)) {
			const {path} = dep;
			if (!path.isAbsolute()) {
				continue;
			}

			await this.wrapError(
				`addFileSource(${dep.path.join()})`,
				() => this.addFileSource(dep),
			);
		}
	}

	public async print() {
		await this.wrapError(
			"root",
			async () => {
				const filteredDiagnostics = this.filterDiagnostics();
				await this.fetchFileSources(filteredDiagnostics);
				await this.printDiagnostics(filteredDiagnostics);
			},
		);
	}

	private async wrapError(reason: string, callback: () => Promise<void>) {
		const {reporter} = this;
		try {
			await callback();
		} catch (err) {
			if (!this.options.wrapErrors) {
				throw err;
			}

			// Sometimes we'll run into issues displaying diagnostics
			// We can safely catch them here since the presence of diagnostics is considered a critical failure anyway
			// Display diagnostics is idempotent meaning we can bail at any point
			// We don't use reporter.error here since the error could have been thrown by cli-layout
			reporter.logRaw(
				`Encountered an error during diagnostics printing in ${reason}`,
			);
			reporter.logRaw(err.stack);
		}
	}

	private async printDiagnostics(diagnostics: Diagnostics) {
		const {reporter} = this;
		const restoreRedirect = reporter.redirectOutToErr(true);

		for (const diag of diagnostics) {
			this.printAuxiliaryDiagnostic(diag);
		}

		for (const diag of diagnostics) {
			await this.wrapError(
				"printDiagnostic",
				async () => this.printDiagnostic(diag),
			);
		}

		reporter.redirectOutToErr(restoreRedirect);
	}

	public getOutdatedFiles(diag: Diagnostic): UnknownPathSet {
		let outdatedFiles: UnknownPathSet = new UnknownPathSet();
		for (const {
			path,
			mtime: expectedMtime,
		} of this.getDependenciesFromDiagnostics([diag])) {
			const mtime = this.fileMtimes.get(path);

			// Consider us outdated if the other file is newer than what we want or doesn't have an mtime
			const isOutdated =
				mtime === undefined ||
				(expectedMtime !== undefined && mtime > expectedMtime);
			if (isOutdated) {
				outdatedFiles.add(path);
			}
		}
		return outdatedFiles;
	}

	private printAuxiliaryDiagnostic(diag: Diagnostic) {
		const {description: {message}, location: {start, filename}} = diag;

		switch (this.flags.auxiliaryDiagnosticFormat) {
			// https://docs.github.com/en/actions/reference/workflow-commands-for-github-actions#setting-an-error-message
			// Format: \:\:error file=app.js,line=10,col=15::Something went wrong
			// TODO escaping
			case "github-actions": {
				const parts = [];

				if (filename !== undefined) {
					const path = createUnknownPath(filename);

					if (path.isAbsolute() && path.isRelativeTo(this.cwd)) {
						parts.push(`file=${this.cwd.relative(path).join()}`);
					} else {
						parts.push(`file=${filename}`);
					}
				}

				if (start !== undefined) {
					if (start.line !== undefined) {
						parts.push(`line=${ob1Get1(start.line)}`);
					}

					if (start.column !== undefined) {
						parts.push(`col=${ob1Get0(start.column)}`);
					}
				}

				let log = `::error ${parts.join(",")}::${joinMarkupLines(
					markupToPlainText(message),
				)}`;
				this.reporter.logRaw(log);
				break;
			}
		}
	}

	public printDiagnostic(diag: Diagnostic) {
		const {reporter} = this;
		const {start, end, filename} = diag.location;
		let advice = [...diag.description.advice];

		// Remove stacktrace from beginning if it contains only one frame that matches the root diagnostic location
		const firstAdvice = advice[0];
		if (
			firstAdvice !== undefined &&
			firstAdvice.type === "stacktrace" &&
			firstAdvice.frames.length === 1
		) {
			const frame = firstAdvice.frames[0];
			if (frame.filename === filename && equalPosition(frame, start)) {
				advice.shift();
			}
		}

		// Determine if we should skip showing the frame at the top of the diagnostic output
		// We check if there are any frame advice entries that match us exactly, this is
		// useful for simplifying stacktraces
		let skipFrame = false;
		if (start !== undefined && end !== undefined) {
			adviceLoop: for (const item of advice) {
				if (
					item.type === "frame" &&
					item.location.filename === filename &&
					equalPosition(item.location.start, start) &&
					equalPosition(item.location.end, end)
				) {
					skipFrame = true;
					break;
				}

				if (item.type === "stacktrace") {
					for (const frame of item.frames) {
						if (frame.filename === filename && equalPosition(frame, start)) {
							skipFrame = true;
							break adviceLoop;
						}
					}
				}
			}
		}

		// Check for outdated files
		const outdatedAdvice: DiagnosticAdvice = [];
		const outdatedFiles = this.getOutdatedFiles(diag);

		// Check if this file doesn't even exist
		if (filename !== undefined) {
			const path = this.createFilePath(filename);
			const isMissing = this.missingFileSources.has(path);
			if (isMissing) {
				outdatedAdvice.push({
					type: "log",
					category: "warn",
					text: markup`This diagnostic refers to a file that does not exist`,
				});
				// Don't need to duplicate this path
				outdatedFiles.delete(path);
				skipFrame = true;
			}
		}

		// List outdated
		const isOutdated = outdatedFiles.size > 0;
		if (isOutdated) {
			const outdatedFilesArr = Array.from(outdatedFiles, (path) => path.join());

			if (outdatedFilesArr.length === 1 && outdatedFilesArr[0] === filename) {
				outdatedAdvice.push({
					type: "log",
					category: "warn",
					text: markup`This file has been changed since the diagnostic was produced and may be out of date`,
				});
			} else {
				outdatedAdvice.push({
					type: "log",
					category: "warn",
					text: markup`This diagnostic may be out of date as it relies on the following files that have been changed since the diagnostic was generated`,
				});

				outdatedAdvice.push({
					type: "list",
					list: outdatedFilesArr.map((filename) =>
						markup`<filelink target="${filename}" />`
					),
				});
			}
		}

		const derived = deriveRootAdviceFromDiagnostic(
			diag,
			{
				skipFrame,
				includeHeaderInAdvice: false,
				outdated: isOutdated,
			},
		);

		reporter.hr(derived.header);

		reporter.indentSync(() => {
			// Concat all the advice together
			const allAdvice: DiagnosticAdvice = [
				...derived.advice,
				...outdatedAdvice,
				...advice,
				...derived.lastAdvice,
			];

			const {truncated} = printAdvice(
				allAdvice,
				{
					printer: this,
					flags: this.flags,
					missingFileSources: this.missingFileSources,
					fileSources: this.fileSources,
					diagnostic: diag,
					reporter,
				},
			);

			if (truncated) {
				this.hasTruncatedDiagnostics = true;
			}

			// Print verbose information
			if (this.flags.verboseDiagnostics === true) {
				const {origins} = diag;

				if (origins !== undefined && origins.length > 0) {
					reporter.br();
					reporter.info(markup`Why are you seeing this diagnostic?`);
					reporter.br();
					reporter.list(
						origins.map((origin) => {
							let res = markup`<emphasis>${origin.category}</emphasis>`;
							if (origin.message !== undefined) {
								res = markup`${res}: ${origin.message}`;
							}
							return res;
						}),
						{ordered: true},
					);
				}
			}
		});
	}

	private filterDiagnostics(): Diagnostics {
		const diagnostics = this.processor.getDiagnostics();
		const filteredDiagnostics: Diagnostics = [];

		for (const diag of diagnostics) {
			this.problemCount++;

			if (this.shouldIgnore(diag)) {
				this.filteredCount++;
			} else if (this.shouldTruncate()) {
				this.truncatedCount++;
			} else {
				this.displayedCount++;
				filteredDiagnostics.push(diag);
			}
		}

		return filteredDiagnostics;
	}

	public inject(title: StaticMarkup, printer: DiagnosticsPrinter) {
		this.processor.addDiagnostics(printer.processor.getDiagnostics());

		const {onFooterPrintCallbacks} = printer;
		if (onFooterPrintCallbacks.length === 0) {
			return;
		}

		this.onFooterPrint(
			async (reporter) => {
				reporter.br();
				reporter.log(markup`<emphasis>${title}</emphasis>`);
				reporter.br();

				await reporter.indent(async () => {
					// Include a more specific "X problems found" for each command
					const hasProblems = printer.hasProblems();
					if (hasProblems) {
						printer.defaultFooter();
					}

					for (const {callback} of onFooterPrintCallbacks) {
						await callback(reporter, hasProblems);
					}
				});

				reporter.br();
			},
			true,
		);
	}

	public onFooterPrint(callback: FooterPrintCallback, after: boolean = false) {
		this.onFooterPrintCallbacks.push({callback, after});
	}

	public hasProblems(): boolean {
		return this.problemCount > 0;
	}

	public disableDefaultFooter() {
		this.defaultFooterEnabled = false;
	}

	public defaultFooter() {
		const {reporter} = this;
		if (!this.defaultFooterEnabled) {
			return;
		}

		if (this.hasProblems()) {
			const {reporter, filteredCount} = this;

			const displayableProblems = this.getDisplayedProblemsCount();
			let str = markup`Found <emphasis>${displayableProblems}</emphasis> <grammarNumber plural="problems" singular="problem">${String(
				displayableProblems,
			)}</grammarNumber>`;

			if (filteredCount > 0) {
				str = markup`${str} <dim>(${filteredCount} filtered)</dim>`;
			}

			reporter.error(str);
		} else {
			reporter.success(markup`No known problems!`);
		}
	}

	public async footer() {
		await this.wrapError(
			"footer",
			async () => {
				const {reporter} = this;
				const isError = this.hasProblems();

				if (isError) {
					const restoreRedirect = reporter.redirectOutToErr(true);
					reporter.hr();
					reporter.redirectOutToErr(restoreRedirect);
				}

				const displayableProblems = this.getDisplayedProblemsCount();
				if (this.truncatedCount > 0) {
					const {maxDiagnostics} = this.flags;
					reporter.warn(
						markup`Only <emphasis>${maxDiagnostics}</emphasis> errors shown. Add the <code>--show-all-diagnostics</code> flag or specify <code>--max-diagnostics ${"<num>"}</code> to view the remaining ${displayableProblems -
						maxDiagnostics} errors`,
					);
				}

				if (this.hasTruncatedDiagnostics) {
					reporter.warn(
						markup`Some diagnostics have been truncated. Use the --verbose-diagnostics flag to disable truncation.`,
					);
				}

				if (this.hasTruncatedDiagnostics || this.truncatedCount > 0) {
					reporter.br();
				}

				if (isError) {
					if (this.flags.fieri) {
						this.showBanner((errorBanner as RawBanner));
					}
				} else {
					if (this.flags.fieri) {
						this.showBanner((successBanner as RawBanner));
					}
				}

				for (const {callback, after} of this.onFooterPrintCallbacks) {
					if (!after) {
						await callback(reporter, isError);
					}
				}

				this.defaultFooter();

				for (const {callback, after} of this.onFooterPrintCallbacks) {
					if (after) {
						await callback(reporter, isError);
					}
				}
			},
		);
	}

	private showBanner(banner: RawBanner) {
		for (const {stream} of this.reporter.getStreamHandles()) {
			if (stream.format !== "ansi") {
				continue;
			}

			const text = "FLAVORTOWN";
			let textIndex = 0;
			let height = 0;
			let width = 0;

			let image: Array<Array<MarkupRGB>> = [];

			// Decompress banner
			for (const row of banner.rows) {
				const unpackedRow: Array<MarkupRGB> = [];

				for (const field of row) {
					let palleteIndex;
					let times = 1;
					if (Array.isArray(field)) {
						[palleteIndex, times] = field;
					} else {
						palleteIndex = field;
					}

					const pallete = banner.palettes[palleteIndex];
					for (let i = 0; i < times; i++) {
						unpackedRow.push(pallete);
					}
				}

				image.push(unpackedRow);
				if (unpackedRow.length > width) {
					width = unpackedRow.length;
				}
				height++;
			}

			// Calculate scale
			let scale =
				stream.features.columns === undefined
					? 1
					: ob1Get1(stream.features.columns) / height;
			if (scale > 1) {
				scale = 1;
			}

			function averageColors(colors: Array<MarkupRGB>): MarkupRGB {
				let averageColor: MarkupRGB = [0, 0, 0];

				for (const color of colors) {
					averageColor[0] += color[0];
					averageColor[1] += color[1];
					averageColor[2] += color[2];
				}

				return [
					Math.round(averageColor[0] / colors.length),
					Math.round(averageColor[1] / colors.length),
					Math.round(averageColor[2] / colors.length),
				];
			}

			// Scale image if necessary
			if (scale < 1) {
				const scaledHeight = Math.floor(height * scale);
				const scaledWidth = Math.floor(width * scale);
				const scaledImage: Array<Array<MarkupRGB>> = [];

				const heightRatio = width / scaledHeight;
				const widthRatio = width / scaledWidth;

				for (let i = 1; i <= scaledHeight; i++) {
					const start = Math.floor(i * heightRatio);
					const end = Math.ceil(i * heightRatio);

					// Scale height
					const scaledHeightRow: Array<MarkupRGB> = [];
					for (let i = 0; i < width; i++) {
						const colors: Array<MarkupRGB> = [];

						for (let x = start; x <= end; x++) {
							const color = image[x - 1][i];
							if (color !== undefined) {
								colors.push(color);
							}
						}

						scaledHeightRow.push(averageColors(colors));
					}

					// Scale width
					const scaledRow: Array<MarkupRGB> = [];
					for (let i = 1; i <= scaledWidth; i++) {
						const start = Math.floor(i * widthRatio);
						const end = Math.ceil(i * widthRatio);

						const colors: Array<MarkupRGB> = [];
						for (let i = start; i <= end; i++) {
							colors.push(scaledHeightRow[i - 1]);
						}

						scaledRow.push(averageColors(colors));
					}

					scaledImage.push(scaledRow);
				}

				image = scaledImage;
			}

			// Print image
			for (const row of image) {
				let line = "";

				for (const color of row) {
					let char = text[textIndex];
					textIndex++;
					if (textIndex === text.length) {
						textIndex = 0;
					}

					char = formatAnsiRGB({
						background: false,
						features: stream.features,
						value: char,
						color,
					});

					line += formatAnsiRGB({
						background: true,
						features: stream.features,
						value: char,
						color,
					});
				}

				stream.write(`${line}\n`, false);
			}
		}
	}
}

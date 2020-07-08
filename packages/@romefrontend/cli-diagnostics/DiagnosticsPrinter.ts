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
	DiagnosticSourceType,
	Diagnostics,
	DiagnosticsProcessor,
	deriveRootAdviceFromDiagnostic,
} from "@romefrontend/diagnostics";
import {Reporter} from "@romefrontend/cli-reporter";
import {
	DiagnosticsFileReader,
	DiagnosticsFileReaderStats,
	DiagnosticsPrinterFlags,
	DiagnosticsPrinterOptions,
} from "./types";
import {
	escapeMarkup,
	formatAnsi,
	markup,
	markupToPlainTextString,
} from "@romefrontend/string-markup";
import {ToLines, toLines} from "./utils";
import {printAdvice} from "./printAdvice";
import {default as successBanner} from "./banners/success.json";
import {default as errorBanner} from "./banners/error.json";
import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	UnknownFilePath,
	UnknownFilePathMap,
	UnknownFilePathSet,
	createAbsoluteFilePath,
	createUnknownFilePath,
} from "@romefrontend/path";
import {Number0, Number1} from "@romefrontend/ob1";
import {existsSync, lstatSync, readFileTextSync} from "@romefrontend/fs";

type Banner = {
	// Array<number> should really be [number, number, number], but TypeScript widens the imported types
	palettes: Array<Array<number>>;
	// Array<number> should really be [number, number], same reason as above
	rows: Array<Array<number | Array<number>>>;
};

type PositionLike = {
	line?: undefined | Number1;
	column?: undefined | Number0;
};

export function readDiagnosticsFileLocal(
	path: AbsoluteFilePath,
): ReturnType<DiagnosticsFileReader> {
	if (!existsSync(path)) {
		return;
	}

	const src = readFileTextSync(path);
	const mtime = lstatSync(path).mtimeMs;
	return {content: src, mtime};
}

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

type FooterPrintCallback = (
	reporter: Reporter,
	error: boolean,
) => void | boolean;

export const DEFAULT_PRINTER_FLAGS: DiagnosticsPrinterFlags = {
	grep: "",
	inverseGrep: false,
	showAllDiagnostics: true,
	fieri: false,
	verboseDiagnostics: false,
	maxDiagnostics: 100,
};

// Dependency that may not be included in the output diagnostic but whose changes may effect the validity of this one
type ChangeFileDependency = {
	type: "change";
	path: UnknownFilePath;
	mtime: number;
};

// Dependency that will have a code frame in the output diagnostic
type ReferenceFileDependency = {
	type: "reference";
	path: UnknownFilePath;
	mtime: undefined | number;
	sourceType: undefined | DiagnosticSourceType;
	language: undefined | DiagnosticLanguage;
};

type FileDependency = ChangeFileDependency | ReferenceFileDependency;

export type DiagnosticsPrinterFileSources = UnknownFilePathMap<{
	sourceText: string;
	lines: ToLines;
}>;

export type DiagnosticsPrinterFileMtimes = UnknownFilePathMap<number>;

export default class DiagnosticsPrinter extends Error {
	constructor(opts: DiagnosticsPrinterOptions) {
		super(
			"Diagnostics printer. If you're seeing this then it wasn't caught and printed correctly.",
		);
		const {cwd, reporter, flags = DEFAULT_PRINTER_FLAGS} = opts;

		this.reporter = reporter;
		this.flags = flags;
		this.readFile =
			opts.readFile === undefined ? readDiagnosticsFileLocal : opts.readFile;
		this.cwd = cwd === undefined ? createAbsoluteFilePath(process.cwd()) : cwd;
		this.processor =
			opts.processor === undefined ? new DiagnosticsProcessor() : opts.processor;

		this.displayedCount = 0;
		this.problemCount = 0;
		this.filteredCount = 0;
		this.truncatedCount = 0;

		this.hasTruncatedDiagnostics = false;
		this.missingFileSources = new AbsoluteFilePathSet();
		this.fileSources = new UnknownFilePathMap();
		this.fileMtimes = new UnknownFilePathMap();
		this.onFooterPrintCallbacks = [];
	}

	reporter: Reporter;
	processor: DiagnosticsProcessor;
	onFooterPrintCallbacks: Array<FooterPrintCallback>;
	flags: DiagnosticsPrinterFlags;
	cwd: AbsoluteFilePath;
	readFile: DiagnosticsFileReader;
	hasTruncatedDiagnostics: boolean;
	missingFileSources: AbsoluteFilePathSet;
	fileSources: DiagnosticsPrinterFileSources;
	fileMtimes: DiagnosticsPrinterFileMtimes;

	displayedCount: number;
	problemCount: number;
	filteredCount: number;
	truncatedCount: number;

	createFilePath(filename: undefined | string): UnknownFilePath {
		if (filename === undefined) {
			filename = "unknown";
		}

		const {normalizePosition} = this.reporter.markupOptions;

		if (normalizePosition === undefined) {
			return createUnknownFilePath(filename);
		} else {
			return createUnknownFilePath(
				normalizePosition(filename, undefined, undefined).filename,
			);
		}
	}

	throwIfAny() {
		if (this.hasDiagnostics()) {
			throw this;
		}
	}

	hasDiagnostics(): boolean {
		return this.processor.hasDiagnostics();
	}

	getDisplayedProblemsCount() {
		return this.problemCount - this.filteredCount;
	}

	shouldTruncate(): boolean {
		if (
			!this.flags.showAllDiagnostics &&
			this.displayedCount > this.flags.maxDiagnostics
		) {
			return true;
		} else {
			return false;
		}
	}

	getDiagnostics(): Diagnostics {
		return this.processor.getSortedDiagnostics();
	}

	shouldIgnore(diag: Diagnostic): boolean {
		const {grep, inverseGrep} = this.flags;

		// An empty grep pattern means show everything
		if (grep === undefined || grep === "") {
			return false;
		}

		// Match against the supplied grep pattern
		let ignored =
			markupToPlainTextString(diag.description.message.value).toLowerCase().includes(
				grep,
			) === false;
		if (inverseGrep) {
			ignored = !ignored;
		}
		return ignored;
	}

	addFileSource(
		info: ChangeFileDependency | ReferenceFileDependency,
		stats: DiagnosticsFileReaderStats,
	) {
		this.fileMtimes.set(info.path, stats.mtime);

		if (info.type === "reference") {
			this.fileSources.set(
				info.path,
				{
					sourceText: stats.content,
					lines: toLines({
						path: info.path,
						input: stats.content,
						sourceType: info.sourceType,
						language: info.language,
					}),
				},
			);
		}
	}

	getDependenciesFromDiagnostics(
		diagnostics: Diagnostics,
	): Array<FileDependency> {
		const deps: Array<FileDependency> = [];

		for (const diag of diagnostics) {
			const {
				dependencies,
				description: {advice},
				location: {language, sourceTypeJS: sourceType, mtime, filename},
			} = diag;

			if (filename !== undefined) {
				deps.push({
					type: "reference",
					path: this.createFilePath(filename),
					mtime,
					language,
					sourceType,
				});
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
					if (
						location.filename !== undefined &&
						location.sourceText === undefined
					) {
						deps.push({
							type: "reference",
							path: this.createFilePath(location.filename),
							language: location.language,
							sourceType: location.sourceTypeJS,
							mtime: location.mtime,
						});
					}
				}
			}
		}

		const depsMap: UnknownFilePathMap<FileDependency> = new UnknownFilePathMap();

		// Remove non-absolute filenames and normalize sourceType and language for conflicts
		for (const dep of deps) {
			const path = dep.path;
			if (!path.isAbsolute()) {
				continue;
			}

			const existing = depsMap.get(path);

			// reference dependency can override change since it has more metadata that needs conflict resolution
			if (existing === undefined || existing.type === "change") {
				depsMap.set(dep.path, dep);
				continue;
			}

			if (dep.type === "reference") {
				if (existing.sourceType !== dep.sourceType) {
					existing.sourceType = "unknown";
				}

				if (existing.language !== dep.language) {
					existing.language = "unknown";
				}
			}
		}

		return Array.from(depsMap.values());
	}

	fetchFileSources(diagnostics: Diagnostics) {
		for (const dep of this.getDependenciesFromDiagnostics(diagnostics)) {
			const {path} = dep;
			if (!path.isAbsolute()) {
				continue;
			}

			const abs = path.assertAbsolute();
			const stats = this.readFile(abs);
			if (stats === undefined) {
				this.missingFileSources.add(abs);
			} else {
				this.wrapError(() => this.addFileSource(dep, stats));
			}
		}
	}

	print() {
		const filteredDiagnostics = this.filterDiagnostics();
		this.fetchFileSources(filteredDiagnostics);
		this.displayDiagnostics(filteredDiagnostics);
	}

	wrapError(callback: () => void) {
		const {reporter} = this;
		try {
			callback();
		} catch (err) {
			// Sometimes we'll run into issues displaying diagnostics
			// We can safely catch them here since the presence of diagnostics is considered a critical failure
			// Display diagnostics is idempotent
			reporter.error("Encountered an error displaying this diagnostic");
			reporter.error(escapeMarkup(err.stack));
		}
	}

	displayDiagnostics(diagnostics: Diagnostics) {
		const {reporter} = this;
		const restoreRedirect = reporter.redirectOutToErr(true);

		for (const diag of diagnostics) {
			this.wrapError(() => this.displayDiagnostic(diag));
		}

		reporter.redirectOutToErr(restoreRedirect);
	}

	getOutdatedFiles(diag: Diagnostic): UnknownFilePathSet {
		let outdatedFiles: UnknownFilePathSet = new UnknownFilePathSet();
		for (const {
			path,
			mtime: expectedMtime,
		} of this.getDependenciesFromDiagnostics([diag])) {
			const mtime = this.fileMtimes.get(path);
			if (
				mtime !== undefined &&
				expectedMtime !== undefined &&
				mtime > expectedMtime
			) {
				outdatedFiles.add(path);
			}
		}
		return outdatedFiles;
	}

	displayDiagnostic(diag: Diagnostic) {
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
		// useful for stuff like reporting call stacks
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

		const outdatedAdvice: DiagnosticAdvice = [];
		const outdatedFiles = this.getOutdatedFiles(diag);
		const isOutdated = outdatedFiles.size > 0;
		if (isOutdated) {
			const outdatedFilesArr = Array.from(outdatedFiles, (path) => path.join());

			if (outdatedFilesArr.length === 1 && outdatedFilesArr[0] === filename) {
				outdatedAdvice.push({
					type: "log",
					category: "warn",
					text: "This file has been changed since the diagnostic was produced and may be out of date",
				});
			} else {
				outdatedAdvice.push({
					type: "log",
					category: "warn",
					text: "This diagnostic may be out of date as it relies on the following files that have been changed since the diagnostic was generated",
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

		reporter.indent(() => {
			// Concat all the advice together
			const allAdvice: DiagnosticAdvice = [
				...derived.advice,
				...outdatedAdvice,
				...advice,
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
			if (this.flags.verboseDiagnostics) {
				const {origins} = diag;

				if (origins !== undefined && origins.length > 0) {
					reporter.br();
					reporter.info("Why are you seeing this diagnostic?");
					reporter.br();
					reporter.list(
						origins.map((origin) => {
							let res = `<emphasis>${origin.category}</emphasis>`;
							if (origin.message !== undefined) {
								res += `: ${origin.message}`;
							}
							return res;
						}),
						{ordered: true},
					);
				}
			}
		});
	}

	filterDiagnostics(): Diagnostics {
		const diagnostics = this.getDiagnostics();
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

	onFooterPrint(fn: FooterPrintCallback) {
		this.onFooterPrintCallbacks.push(fn);
	}

	footer() {
		const {reporter, problemCount} = this;

		const isError = problemCount > 0;

		if (isError) {
			const restoreRedirect = reporter.redirectOutToErr(true);
			reporter.hr();
			reporter.redirectOutToErr(restoreRedirect);
		}

		if (this.hasTruncatedDiagnostics) {
			reporter.warn(
				"Some diagnostics have been truncated. Use the --verbose-diagnostics flag to disable truncation.",
			);
		}

		if (isError) {
			if (this.flags.fieri) {
				this.showBanner(errorBanner);
			}
		} else {
			if (this.flags.fieri) {
				this.showBanner(successBanner);
			}
		}

		for (const handler of this.onFooterPrintCallbacks) {
			const stop = handler(reporter, isError);
			if (stop) {
				return;
			}
		}

		if (isError) {
			this.footerError();
		} else {
			reporter.success("No known problems!");
		}
	}

	showBanner(banner: Banner) {
		for (const stream of this.reporter.getStreams(false)) {
			for (const row of banner.rows) {
				for (const field of row) {
					let palleteIndex;
					let times = 1;
					if (Array.isArray(field)) {
						[palleteIndex, times] = field;
					} else {
						palleteIndex = field;
					}

					const pallete = banner.palettes[palleteIndex];
					stream.write(
						formatAnsi.bgRgb(" ", [pallete[0], pallete[1], pallete[2]]).repeat(
							times,
						),
					);
				}
				stream.write("\n");
			}
		}
	}

	footerError() {
		const {reporter, filteredCount} = this;

		const displayableProblems = this.getDisplayedProblemsCount();
		let str = `Found <number emphasis>${displayableProblems}</number> <grammarNumber plural="problems" singular="problem">${displayableProblems}</grammarNumber>`;

		if (filteredCount > 0) {
			str += `<dim> (${filteredCount} filtered)</dim>`;
		}

		reporter.error(str);

		if (this.truncatedCount > 0) {
			const {maxDiagnostics} = this.flags;
			reporter.warn(
				`Only <number>${maxDiagnostics}</number> errors shown, add the <emphasis>--show-all-diagnostics</emphasis> flag to view the remaining <number>${displayableProblems -
				maxDiagnostics}</number> errors`,
			);
		}
	}
}

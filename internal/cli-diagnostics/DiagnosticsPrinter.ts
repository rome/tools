/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticIntegrity,
	DiagnosticLanguage,
	DiagnosticLocation,
	DiagnosticSourceType,
	DiagnosticsProcessor,
} from "@internal/diagnostics";
import {MarkupRGB, StaticMarkup, markup} from "@internal/markup";
import {Reporter} from "@internal/cli-reporter";
import {
	DiagnosticsFileHandler,
	DiagnosticsPrinterFlags,
	DiagnosticsPrinterOptions,
} from "./types";
import {formatAnsiRGB} from "@internal/cli-layout";
import {ToLines, concatFileHandlers, toLines} from "./utils";
import {printAdvice} from "./printAdvice";
import {default as successBanner} from "./banners/success.json";
import {default as errorBanner} from "./banners/error.json";
import {
	AbsoluteFilePath,
	CWD_PATH,
	MixedPathMap,
	MixedPathSet,
	Path,
	UNKNOWN_PATH,
} from "@internal/path";
import {inferDiagnosticLanguageFromPath} from "@internal/core/common/file-handlers";
import {markupToJoinedPlainText} from "@internal/cli-layout/format";
import {sha256} from "@internal/string-utils";
import {GlobalLock} from "@internal/async";
import {buildDisplayDiagnostic} from "./displayDiagnostic";

type RawBanner = {
	palettes: MarkupRGB[];
	rows: Array<Array<number | MarkupRGB>>;
};

const DEFAULT_FILE_HANDLER: Required<DiagnosticsFileHandler> = {
	async read(path) {
		if (path.isAbsolute()) {
			if ((await path.exists()) && (await path.lstat()).isFile()) {
				return path.createReadStream();
			}
		}

		return undefined;
	},
	async exists(path) {
		if (path.isAbsolute()) {
			return await path.exists();
		} else {
			return undefined;
		}
	},
};

type FooterPrintCallback = (reporter: Reporter, error: boolean) => Promise<void>;

// Dependency that may not be included in the output diagnostic but whose changes may effect the validity of this one
type ChangeFileDependency = {
	type: "change";
	path: Path;
	integrity: undefined | DiagnosticIntegrity;
};

// Dependency that will have a code frame in the output diagnostic
type ReferenceFileDependency = {
	type: "reference";
	path: Path;
	integrity: undefined | DiagnosticIntegrity;
	sourceTypeJS: undefined | DiagnosticSourceType;
	language: undefined | DiagnosticLanguage;
	sourceText: undefined | string;
};

type FileDependency = ChangeFileDependency | ReferenceFileDependency;

function hasFrame(loc: DiagnosticLocation): boolean {
	return loc.start !== undefined || loc.end !== undefined;
}

export type DiagnosticsPrinterFileSources = MixedPathMap<{
	sourceText: string;
	lines: ToLines;
}>;

export type DiagnosticsPrinterFileHashes = MixedPathMap<string>;

export const DEFAULT_PRINTER_FLAGS: DiagnosticsPrinterFlags = {
	auxiliaryDiagnosticFormat: undefined,
	fieri: false,
	truncateDiagnostics: true,
	verboseDiagnostics: false,
};

export default class DiagnosticsPrinter extends Error {
	constructor(opts: DiagnosticsPrinterOptions) {
		super(
			"Diagnostics printer. If you're seeing this then it wasn't caught and printed correctly.",
		);
		const {cwd, reporter, flags} = opts;

		this.options = opts;

		this.reporter = reporter;
		this.flags = {...DEFAULT_PRINTER_FLAGS, ...flags};
		this.fileHandler =
			opts.fileHandlers === undefined
				? DEFAULT_FILE_HANDLER
				: concatFileHandlers([...opts.fileHandlers, DEFAULT_FILE_HANDLER]);
		this.cwd = cwd ?? CWD_PATH;
		this.processor = opts.processor;

		// Ensure we print sequentially
		this.printLock = new GlobalLock();

		this.seenDiagnostics = new Set();
		this.streaming = opts.streaming ?? false;
		this.defaultFooterEnabled = true;
		this.hasTruncatedDiagnostics = false;
		this.missingFileSources = new MixedPathSet();
		this.fileDependencies = new MixedPathMap();
		this.fileSources = new MixedPathMap();
		this.fileHashes = new MixedPathMap();
		this.dependenciesByDiagnostic = new Map();
		this.onFooterPrintCallbacks = [];

		if (this.streaming) {
			if (this.processor.hasDiagnostics()) {
				this.printBody(this.processor.getDiagnostics());
			}

			this.processor.guaranteedDiagnosticsEvent.subscribe((diags) => {
				this.printBody(diags);
			});
		}
	}

	public processor: DiagnosticsProcessor;
	public flags: DiagnosticsPrinterFlags;
	public defaultFooterEnabled: boolean;

	private streaming: boolean;
	private seenDiagnostics: Set<Diagnostic>;
	private printLock: GlobalLock;
	private options: DiagnosticsPrinterOptions;
	private reporter: Reporter;
	private onFooterPrintCallbacks: {
		callback: FooterPrintCallback;
		after: boolean;
	}[];
	private cwd: AbsoluteFilePath;
	private fileHandler: Required<DiagnosticsFileHandler>;
	private hasTruncatedDiagnostics: boolean;
	private missingFileSources: MixedPathSet;
	private fileDependencies: MixedPathMap<FileDependency>;
	private fileSources: DiagnosticsPrinterFileSources;
	private fileHashes: DiagnosticsPrinterFileHashes;
	private dependenciesByDiagnostic: Map<Diagnostic, FileDependency[]>;

	public normalizePath(path: Path): Path {
		const normalizePosition = this.reporter.markupOptions?.normalizePosition;

		if (normalizePosition === undefined) {
			return path;
		} else {
			return normalizePosition(path, undefined, undefined).path;
		}
	}

	// Only highlight if we have a reporter stream enabled that isn't format: "none"
	public shouldHighlight(): boolean {
		for (const stream of this.reporter.getStreams()) {
			if (stream.format !== "none") {
				return true;
			}
		}
		return false;
	}

	private async checkMissing(path: Path): Promise<void> {
		let exists: undefined | boolean = await this.fileHandler.exists(path);
		if (exists === undefined && path.isUID()) {
			exists = true;
		}
		if (!exists) {
			this.missingFileSources.add(path);
		}
	}

	private async addFileSource(dep: FileDependency) {
		const {path} = dep;

		const hasExisting = this.fileDependencies.get(path);
		this.fileDependencies.set(path, dep);

		let needsHash = dep.integrity !== undefined && !this.fileHashes.get(path);
		let needsSource = dep.type === "reference" && !this.fileSources.has(path);

		// If we don't need the source then just do an existence check
		if (!(needsSource || needsHash)) {
			// If we've already seen this file source then we've already checked if it exists
			if (!hasExisting) {
				await this.checkMissing(path);
			}
			return;
		}

		// Fetch the source
		let sourceText: undefined | string;
		if (dep.type === "reference") {
			sourceText = dep.sourceText;
		}
		if (needsSource || needsHash) {
			if (sourceText === undefined) {
				const stream = await this.fileHandler.read(path);

				if (stream !== undefined) {
					if (typeof stream === "string") {
						sourceText = stream;
						if (needsHash) {
							this.fileHashes.set(path, sha256.sync(stream));
						}
					} else {
						let buff = "";

						stream.on(
							"data",
							(chunk) => {
								buff += chunk.toString();
							},
						);

						if (needsHash) {
							// Stream a hash. This will finish when the stream has ended so we don't need to manually attach events
							const hash = await sha256.async(stream);
							this.fileHashes.set(path, hash);
						} else {
							await new Promise<void>((resolve, reject) => {
								stream.on(
									"error",
									(err) => {
										reject(err);
									},
								);

								stream.on(
									"end",
									() => {
										resolve();
									},
								);
							});
						}

						sourceText = buff;
					}
				}
			}
			if (sourceText === undefined) {
				// Perform an explicit exists test
				if (!hasExisting) {
					await this.checkMissing(path);
				}
				return;
			}

			if (dep.type === "reference") {
				this.setFileSource(dep, sourceText);
			}
		}
	}

	private setFileSource(dep: ReferenceFileDependency, sourceText: string): void {
		this.fileSources.set(
			dep.path,
			{
				sourceText,
				lines: toLines({
					highlight: this.shouldHighlight(),
					path: dep.path,
					input: sourceText,
					sourceTypeJS: dep.sourceTypeJS,
					language: inferDiagnosticLanguageFromPath(dep.path, dep.language),
				}),
			},
		);
	}

	private getDependenciesFromDiagnostic(diag: Diagnostic): FileDependency[] {
		const cached = this.dependenciesByDiagnostic.get(diag);
		if (cached !== undefined) {
			return cached;
		}

		const deps: FileDependency[] = [];

		const {
			dependencies,
			description: {advice},
			location: {language, sourceTypeJS, sourceText, integrity, path},
		} = diag;

		if (path !== undefined) {
			const normalPath = this.normalizePath(path);

			if (hasFrame(diag.location)) {
				deps.push({
					type: "reference",
					path: normalPath,
					integrity,
					language,
					sourceTypeJS,
					sourceText,
				});
			} else {
				deps.push({
					type: "change",
					path: normalPath,
					integrity,
				});
			}
		}

		if (dependencies !== undefined) {
			for (const {path, integrity} of dependencies) {
				deps.push({
					type: "change",
					path: this.normalizePath(path),
					integrity,
				});
			}
		}

		for (const item of advice) {
			if (item.type === "frame") {
				const {location} = item;
				if (location.path !== undefined) {
					const path = this.normalizePath(location.path);
					if (hasFrame(location)) {
						deps.push({
							type: "reference",
							path,
							language: location.language,
							sourceTypeJS: location.sourceTypeJS,
							integrity: location.integrity,
							sourceText: location.sourceText,
						});
					} else {
						deps.push({
							type: "change",
							path,
							integrity,
						});
					}
				}
			}

			if (item.type === "stacktrace") {
				for (const {path, line, column, sourceText} of item.frames) {
					if (path !== undefined) {
						const normalPath = this.normalizePath(path);
						if (line !== undefined && column !== undefined) {
							deps.push({
								type: "reference",
								path: normalPath,
								language: undefined,
								sourceTypeJS: undefined,
								integrity: undefined,
								sourceText,
							});
						} else {
							deps.push({
								type: "change",
								path: normalPath,
								integrity: undefined,
							});
						}
					}
				}
			}
		}

		this.dependenciesByDiagnostic.set(diag, deps);
		return deps;
	}

	private getDependenciesFromDiagnostics(
		diagnostics: Diagnostic[],
	): FileDependency[] {
		let deps: FileDependency[] = [];
		for (const diag of diagnostics) {
			deps = [...deps, ...this.getDependenciesFromDiagnostic(diag)];
		}

		const depsMap: MixedPathMap<FileDependency> = new MixedPathMap();

		// Remove non-absolute filenames and normalize sourceType and language for conflicts
		for (const dep of deps) {
			const path = dep.path;
			if (UNKNOWN_PATH.equal(path)) {
				continue;
			}

			const existing = depsMap.get(path) ?? this.fileDependencies.get(path);

			// "reference" dependency can override "change" since it has more metadata that needs conflict resolution
			if (existing === undefined || existing.type === "change") {
				depsMap.set(path, dep);
				continue;
			}

			if (dep.type === "reference") {
				if (
					dep.sourceText !== undefined &&
					existing.sourceText !== undefined &&
					dep.sourceText !== existing.sourceText
				) {
					throw new Error(
						`Found multiple sourceText entries for ${dep.path.join()} that didn't match`,
					);
				}

				let language = existing.language ?? dep.language;
				if (
					dep.language === undefined &&
					existing.language !== undefined &&
					dep.language !== existing.language
				) {
					language = "unknown";
				}

				let sourceTypeJS = existing.sourceTypeJS ?? dep.sourceTypeJS;
				if (
					dep.sourceTypeJS === undefined &&
					existing.sourceTypeJS !== undefined &&
					dep.sourceTypeJS !== existing.sourceTypeJS
				) {
					sourceTypeJS = "unknown";
				}

				depsMap.set(
					path,
					{
						...existing,
						sourceText: existing.sourceText ?? dep.sourceText,
						sourceTypeJS,
						language,
					},
				);
			}
		}

		return Array.from(depsMap.values());
	}

	public async fetchFileSources(diagnostics: Diagnostic[]) {
		for (const dep of this.getDependenciesFromDiagnostics(diagnostics)) {
			await this.wrapErrorAsync(
				`addFileSource(${dep.path.join()})`,
				() => this.addFileSource(dep),
			);
		}
	}

	public async printBody(diagnostics: Diagnostic[]) {
		await this.wrapErrorAsync(
			"root",
			async () => {
				await this.printLock.series(async () => {
					const filteredDiagnostics = this.filterDiagnostics(diagnostics);
					if (filteredDiagnostics.length === 0) {
						return;
					}

					await this.fetchFileSources(filteredDiagnostics);
					await this.printDiagnostics(filteredDiagnostics);
				});
			},
		);
	}

	private async wrapErrorAsync(reason: string, callback: () => Promise<void>) {
		try {
			await callback();
		} catch (err) {
			this.handleErrorInWrap(reason, err);
		}
	}

	private wrapErrorSync(reason: string, callback: () => void) {
		try {
			callback();
		} catch (err) {
			this.handleErrorInWrap(reason, err);
		}
	}

	private handleErrorInWrap(reason: string, err: Error): void {
		if (!this.options.wrapErrors) {
			throw err;
		}

		// Sometimes we'll run into issues displaying diagnostics
		// We can safely catch them here since the presence of diagnostics is considered a critical failure anyway
		// Display diagnostics is idempotent meaning we can bail at any point
		// We don't use reporter.error here since the error could have been thrown by cli-layout
		const {reporter} = this;
		reporter.logRaw(
			`Encountered an error during diagnostics printing in ${reason}`,
		);
		reporter.logRaw(err.stack || err.message);
	}

	private async printDiagnostics(diagnostics: Diagnostic[]): Promise<void> {
		const reporter = new Reporter("DiagnosticsPrinter");
		const stream = reporter.attachCaptureStream("markup");

		for (const diag of diagnostics) {
			this.printAuxiliaryDiagnostic(diag);
		}

		for (const diag of diagnostics) {
			this.wrapErrorSync(
				"printDiagnostic",
				() => this.printDiagnostic(diag, reporter, true),
			);
		}

		this.reporter.log(stream.readAsMarkup(), {stderr: true, noNewline: true});
		await reporter.resources.release();
	}

	public getDiagnosticDependencyMeta(
		diag: Diagnostic,
	): {
		missingPaths: MixedPathSet;
		outdatedPaths: MixedPathSet;
	} {
		const outdatedPaths: MixedPathSet = new MixedPathSet();
		const missingPaths: MixedPathSet = new MixedPathSet();

		for (const {
			path,
			integrity: expectedIntegrity,
		} of this.getDependenciesFromDiagnostics([diag])) {
			if (this.missingFileSources.has(path)) {
				missingPaths.add(path);
				continue;
			}

			if (expectedIntegrity === undefined) {
				continue;
			}

			const actualHash = this.fileHashes.get(path);
			const isOutdated = actualHash !== expectedIntegrity.hash;
			if (isOutdated) {
				outdatedPaths.add(path);
			}
		}

		return {outdatedPaths, missingPaths};
	}

	private printAuxiliaryDiagnostic(diag: Diagnostic) {
		const {description: {message}, location: {start, path}} = diag;

		switch (this.flags.auxiliaryDiagnosticFormat) {
			// https://docs.github.com/en/actions/reference/workflow-commands-for-github-actions#setting-an-error-message
			// Format: \:\:error file=app.js,line=10,col=15::Something went wrong
			// TODO escaping
			case "github-actions": {
				const parts = [];

				if (path?.isAbsolute()) {
					parts.push(`file=${this.cwd.relative(path).join()}`);

					if (start !== undefined) {
						if (start.line !== undefined) {
							parts.push(`line=${start.line.valueOf()}`);
						}

						if (start.column !== undefined) {
							parts.push(`col=${start.column.valueOf()}`);
						}
					}
				}

				let log = `::error ${parts.join(",")}::${markupToJoinedPlainText(
					message,
				)}`;
				this.reporter.logRaw(log);
				break;
			}
		}
	}

	printDiagnostic(
		diag: Diagnostic,
		reporter: Reporter,
		validateDependencies: boolean,
	): void {
		let outdatedPaths: MixedPathSet;
		let missingPaths: MixedPathSet;
		if (validateDependencies) {
			({outdatedPaths, missingPaths} = this.getDiagnosticDependencyMeta(diag));
		} else {
			outdatedPaths = new MixedPathSet();
			missingPaths = new MixedPathSet();
		}
		const {advice, header} = buildDisplayDiagnostic({
			diagnostic: diag,
			flags: this.flags,
			outdatedPaths,
			missingPaths,
		});

		reporter.hr(header);

		reporter.indentSync(() => {
			const {truncated} = printAdvice(
				advice,
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
		});
	}

	private filterDiagnostics(diagnostics: Diagnostic[]): Diagnostic[] {
		const filteredDiagnostics: Diagnostic[] = [];

		for (const diag of diagnostics) {
			if (!this.seenDiagnostics.has(diag)) {
				this.seenDiagnostics.add(diag);
				filteredDiagnostics.push(diag);
			}
		}

		return filteredDiagnostics;
	}

	public inject(title: StaticMarkup, printer: DiagnosticsPrinter) {
		for (const diag of printer.seenDiagnostics) {
			this.seenDiagnostics.add(diag);
		}
		this.processor.addDiagnostics(printer.processor.getDiagnostics());

		// We include a more specific "X problems found" for each command
		this.disableDefaultFooter();
		this.onFooterPrint(
			async (reporter) => {
				reporter.br();
				reporter.log(markup`<emphasis>${title}</emphasis>`);
				reporter.br();

				await reporter.indent(async () => {
					const hasProblems = printer.hasProblems();

					for (const {callback} of printer.onFooterPrintCallbacks) {
						await callback(reporter, hasProblems);
					}

					if (hasProblems) {
						printer.printDefaultFooter();
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
		return this.seenDiagnostics.size > 0 || this.processor.hasDiagnostics();
	}

	public disableDefaultFooter() {
		this.defaultFooterEnabled = false;
	}

	public printDefaultFooter() {
		const {reporter} = this;
		if (!this.defaultFooterEnabled) {
			return;
		}

		const calculated = this.processor.calculateVisibile();
		const displayableProblems = calculated.total - calculated.filtered;

		let suffix = markup``;
		if (calculated.filtered > 0) {
			suffix = markup` <dim>(${calculated.filtered} filtered)</dim>`;
		}

		if (displayableProblems > 0) {
			const {reporter} = this;

			reporter.error(
				markup`Found <emphasis>${displayableProblems}</emphasis> <grammarNumber plural="problems" singular="problem">${String(
					displayableProblems,
				)}</grammarNumber>${suffix}`,
			);
		} else {
			reporter.success(markup`No known problems!${suffix}`);
		}
	}

	private async printFooter() {
		await this.printLock.wait();

		await this.wrapErrorAsync(
			"footer",
			async () => {
				const {reporter} = this;
				const isError = this.hasProblems();

				if (isError) {
					const restoreRedirect = reporter.redirectOutToErr(true);
					reporter.hr();
					reporter.redirectOutToErr(restoreRedirect);
				}

				const calculated = this.processor.calculateVisibile();
				if (calculated.truncated > 0) {
					reporter.warn(
						markup`Only <emphasis>${calculated.diagnostics.length}</emphasis> of <emphasis>${calculated.total}</emphasis> diagnostics shown. Use <code>--show-all-diagnostics</code> or <code>--max-diagnostics ${"<num>"}</code> flag to view remaining.`,
					);
				}

				if (this.hasTruncatedDiagnostics) {
					reporter.warn(
						markup`Some diagnostics have been truncated. Use the <code>--no-truncate-diagnostics</code> flag to disable truncation.`,
					);
				}

				if (this.hasTruncatedDiagnostics || calculated.truncated > 0) {
					reporter.br();
				}

				if (isError) {
					if (this.flags.fieri) {
						this.showBanner(errorBanner as RawBanner);
					}
				} else {
					if (this.flags.fieri) {
						this.showBanner(successBanner as RawBanner);
					}
				}

				for (const {callback, after} of this.onFooterPrintCallbacks) {
					if (!after) {
						await callback(reporter, isError);
					}
				}

				this.printDefaultFooter();

				for (const {callback, after} of this.onFooterPrintCallbacks) {
					if (after) {
						await callback(reporter, isError);
					}
				}
			},
		);
	}

	public async print(
		{showFooter = true}: {
			showFooter?: boolean;
		} = {},
	): Promise<void> {
		await this.printBody(this.processor.getDiagnostics());

		if (showFooter) {
			await this.printFooter();
		}
	}

	private showBanner(banner: RawBanner) {
		for (const stream of this.reporter.getStreams()) {
			if (stream.format !== "ansi") {
				continue;
			}

			const text = "FLAVORTOWN";
			let textIndex = 0;
			let height = 0;
			let width = 0;

			let image: (MarkupRGB[])[] = [];

			// Decompress banner
			for (const row of banner.rows) {
				const unpackedRow: MarkupRGB[] = [];

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
					: stream.features.columns.valueOf() / height;
			if (scale > 1) {
				scale = 1;
			}

			function averageColors(colors: MarkupRGB[]): MarkupRGB {
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
				const scaledImage: (MarkupRGB[])[] = [];

				const heightRatio = width / scaledHeight;
				const widthRatio = width / scaledWidth;

				for (let i = 1; i <= scaledHeight; i++) {
					const start = Math.floor(i * heightRatio);
					const end = Math.ceil(i * heightRatio);

					// Scale height
					const scaledHeightRow: MarkupRGB[] = [];
					for (let i = 0; i < width; i++) {
						const colors: MarkupRGB[] = [];

						for (let x = start; x <= end; x++) {
							const color = image[x - 1][i];
							if (color !== undefined) {
								colors.push(color);
							}
						}

						scaledHeightRow.push(averageColors(colors));
					}

					// Scale width
					const scaledRow: MarkupRGB[] = [];
					for (let i = 1; i <= scaledWidth; i++) {
						const start = Math.floor(i * widthRatio);
						const end = Math.ceil(i * widthRatio);

						const colors: MarkupRGB[] = [];
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

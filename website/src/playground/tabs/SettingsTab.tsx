import {
	IndentStyle,
	LintRules,
	PlaygroundState,
	QuoteProperties,
	QuoteStyle,
	Semicolons,
	SourceType,
	TrailingComma,
} from "../types";
import type { Dispatch, SetStateAction } from "react";
import React, { useState } from "react";
import {
	modifyFilename,
	createPlaygroundSettingsSetter,
	isJSXFilename,
	isScriptFilename,
	isTypeScriptFilename,
	classnames,
	getFileState,
	normalizeFilename,
} from "../utils";

export interface SettingsTabProps {
	state: PlaygroundState;
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>;
	onReset: () => void;
}

export default function SettingsTab({
	setPlaygroundState,
	onReset,
	state: {
		singleFileMode,
		currentFile,
		files,
		settings: {
			lineWidth,
			indentWidth,
			indentStyle,
			quoteStyle,
			quoteProperties,
			trailingComma,
			semicolons,
			lintRules,
			enabledLinting,
		},
	},
}: SettingsTabProps) {
	const setLineWidth = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"lineWidth",
	);
	const setIndentWidth = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"indentWidth",
	);
	const setIndentStyle = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"indentStyle",
	);
	const setQuoteStyle = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"quoteStyle",
	);
	const setQuoteProperties = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"quoteProperties",
	);
	const setTrailingComma = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"trailingComma",
	);
	const setSemicolons = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"semicolons",
	);
	const setLintRules = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"lintRules",
	);
	const setEnabledLinting = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"enabledLinting",
	);

	function setCurrentFilename(newFilename: string) {
		setPlaygroundState((state) => {
			if (state.currentFile === newFilename) {
				return state;
			}

			const { [state.currentFile]: _, ...otherFiles } = state.files;

			const files: PlaygroundState["files"] = {
				...otherFiles,
				[newFilename]: state.files[state.currentFile]!,
			};

			return {
				...state,
				currentFile: newFilename,
				files,
			};
		});
	}

	function deleteFile(filename: string) {
		setPlaygroundState((state) => {
			const { [filename]: _, ...files } = state.files;
			let currentFile = state.currentFile;

			if (currentFile === filename) {
				const files = Object.keys(state.files);
				const index = files.indexOf(filename);
				currentFile = files[index + 1] ?? files[index - 1] ?? currentFile;
			}

			return {
				...state,
				currentFile,
				files: {
					...files,
					// Make sure currentFile is still accessible
					[currentFile]: getFileState(state, currentFile),
				},
			};
		});
	}

	function createFile(filename: string) {
		filename = normalizeFilename(filename);

		setPlaygroundState((state) => ({
			...state,
			currentFile: filename,
			files: {
				...state.files,
				[filename]: getFileState({ files: {} }, filename),
			},
		}));
	}

	function renameFile(oldFilename: string, newFilename: string) {
		newFilename = normalizeFilename(newFilename);

		setPlaygroundState((state) => {
			const { [oldFilename]: oldFile, ...files } = state.files;

			return {
				...state,
				currentFile:
					state.currentFile === oldFilename ? newFilename : state.currentFile,
				files: {
					...files,
					[newFilename]: oldFile,
				},
			};
		});
	}

	function setCurrentFile(currentFile: string) {
		setPlaygroundState((state) => ({
			...state,
			currentFile,
		}));
	}

	function toggleSingleFileMode() {
		setPlaygroundState((state) => ({
			...state,
			singleFileMode: !state.singleFileMode,
		}));
	}

	return (
		<div className="settings-tab">
			<section className="settings-tab-buttons">
				<button onClick={onReset} onKeyDown={onReset}>
					Reset
				</button>
				<button onClick={toggleSingleFileMode} onKeyDown={toggleSingleFileMode}>
					{singleFileMode ? "Multi-file mode" : "Single-file mode"}
				</button>
			</section>

			{!singleFileMode && (
				<FileView
					currentFile={currentFile}
					files={Object.keys(files)}
					createFile={createFile}
					deleteFile={deleteFile}
					setCurrentFile={setCurrentFile}
					renameFile={renameFile}
				/>
			)}
			<FormatterSettings
				lineWidth={lineWidth}
				setLineWidth={setLineWidth}
				indentStyle={indentStyle}
				setIndentStyle={setIndentStyle}
				indentWidth={indentWidth}
				setIndentWidth={setIndentWidth}
				quoteStyle={quoteStyle}
				setQuoteStyle={setQuoteStyle}
				quoteProperties={quoteProperties}
				setQuoteProperties={setQuoteProperties}
				trailingComma={trailingComma}
				setTrailingComma={setTrailingComma}
				semicolons={semicolons}
				setSemicolons={setSemicolons}
			/>
			<LinterSettings
				lintRules={lintRules}
				setLintRules={setLintRules}
				enabledLinting={enabledLinting}
				setEnabledLinting={setEnabledLinting}
			/>
			<SyntaxSettings filename={currentFile} setFilename={setCurrentFilename} />
		</div>
	);
}

function FileView({
	currentFile,
	createFile,
	deleteFile,
	renameFile,
	setCurrentFile,
	files,
}: {
	createFile: (filename: string) => void;
	deleteFile: (filename: string) => void;
	setCurrentFile: (filename: string) => void;
	renameFile: (oldFilename: string, newFilename: string) => void;
	currentFile: string;
	files: string[];
}) {
	const [isCreatingFile, setCreatingFile] = useState(false);

	return (
		<div className="file-view">
			<h2 className="files-heading">
				Files
				<button onClick={() => setCreatingFile(true)}>
					<span className="sr-only">New</span>
					<span aria-hidden={true}>+</span>
				</button>
			</h2>

			<ul className="files-list">
				{files.map((filename) => {
					return (
						<FileViewItem
							key={filename}
							isActive={filename === currentFile}
							filename={filename}
							canDelete={files.length > 1}
							onClick={() => {
								setCurrentFile(filename);
							}}
							renameFile={(newFilename) => {
								renameFile(filename, newFilename);
							}}
							deleteFile={() => {
								deleteFile(filename);
							}}
						/>
					);
				})}
			</ul>

			{isCreatingFile && (
				<FilenameInput
					onSubmit={(filename) => {
						createFile(filename);
						setCreatingFile(false);
					}}
					onCancel={() => setCreatingFile(false)}
				/>
			)}
		</div>
	);
}

function FileViewItem({
	filename,
	isActive,
	onClick,
	deleteFile,
	renameFile,
	canDelete,
}: {
	filename: string;
	canDelete: boolean;
	isActive: boolean;
	renameFile: (newFilename: string) => void;
	onClick: () => void;
	deleteFile: () => void;
}) {
	const [isRenaming, setIsRenaming] = useState(false);

	const className = classnames(isActive && "active");

	if (isRenaming) {
		return (
			<li>
				<FilenameInput
					onSubmit={(newFilename) => {
						renameFile(newFilename);
						setIsRenaming(false);
					}}
					onCancel={() => {
						setIsRenaming(false);
					}}
					initialValue={filename}
				/>
			</li>
		);
	}

	function onDeleteClick(e: React.MouseEvent | React.KeyboardEvent) {
		e.preventDefault();
		e.stopPropagation();
		deleteFile();
	}

	function onRenameClick(e: React.MouseEvent | React.KeyboardEvent) {
		e.preventDefault();
		e.stopPropagation();
		setIsRenaming(true);
	}

	return (
		<li className={className} onClick={onClick} onKeyDown={onClick}>
			{filename}

			<button onClick={onRenameClick} onKeyDown={onRenameClick}>
				Rename
			</button>

			{canDelete && (
				<button onClick={onDeleteClick} onKeyDown={onDeleteClick}>
					<span className="sr-only">Delete</span>
					<span aria-hidden={true}>X</span>
				</button>
			)}
		</li>
	);
}

function FilenameInput({
	onSubmit,
	onCancel,
	initialValue,
}: {
	onSubmit: (filename: string) => void;
	onCancel: () => void;
	initialValue?: string;
}) {
	const [filename, setFilename] = useState(initialValue ?? "");

	function onKeyDown(e: React.KeyboardEvent) {
		if (e.key === "Escape") {
			onCancel();
		}

		if (e.key === "Enter") {
			onSubmit(filename);
		}
	}

	function onBlur() {
		if (filename === "") {
			onCancel();
		} else {
			onSubmit(filename);
		}
	}

	function onChange(e: React.ChangeEvent<HTMLInputElement>) {
		setFilename(e.target.value);
	}

	return (
		<input
			type="text"
			// rome-ignore lint/a11y/noAutofocus: Not sure how else to do this
			autoFocus={true}
			onKeyDown={onKeyDown}
			onChange={onChange}
			onBlur={onBlur}
			value={filename}
		/>
	);
}

function SyntaxSettings({
	filename,
	setFilename,
}: {
	filename: string;
	setFilename: (filename: string) => void;
}) {
	const isScript = isScriptFilename(filename);

	return (
		<>
			<h2>Syntax</h2>
			<section>
				<div className="field-row">
					<label htmlFor="sourceType">Source Type</label>
					<select
						id="sourceType"
						name="sourceType"
						value={isScript ? "script" : "module"}
						onChange={(e) => {
							setFilename(
								modifyFilename(filename, {
									jsx: false,
									typescript: false,
									script: e.target.value === SourceType.Script,
								}),
							);
						}}
					>
						<option value={SourceType.Module}>Module</option>
						<option value={SourceType.Script}>Script</option>
					</select>
				</div>

				<div className="field-row">
					<input
						id="typescript"
						name="typescript"
						type="checkbox"
						checked={isTypeScriptFilename(filename)}
						onChange={(e) => {
							setFilename(
								modifyFilename(filename, {
									jsx: isJSXFilename(filename),
									typescript: e.target.checked,
									script: false,
								}),
							);
						}}
						disabled={isScript}
					/>
					<label htmlFor="typescript">TypeScript</label>
				</div>

				<div className="field-row">
					<input
						id="jsx"
						name="jsx"
						type="checkbox"
						checked={isJSXFilename(filename)}
						onChange={(e) => {
							setFilename(
								modifyFilename(filename, {
									jsx: e.target.checked,
									typescript: isTypeScriptFilename(filename),
									script: false,
								}),
							);
						}}
						disabled={isScript}
					/>
					<label htmlFor="jsx">JSX</label>
				</div>
			</section>
		</>
	);
}

function FormatterSettings({
	lineWidth,
	setLineWidth,
	indentStyle,
	setIndentStyle,
	indentWidth,
	setIndentWidth,
	quoteStyle,
	setQuoteStyle,
	quoteProperties,
	setQuoteProperties,
	trailingComma,
	setTrailingComma,
	semicolons,
	setSemicolons,
}: {
	lineWidth: number;
	setLineWidth: (value: number) => void;
	indentStyle: IndentStyle;
	setIndentStyle: (value: IndentStyle) => void;
	indentWidth: number;
	setIndentWidth: (value: number) => void;
	quoteStyle: QuoteStyle;
	setQuoteStyle: (value: QuoteStyle) => void;
	quoteProperties: QuoteProperties;
	setQuoteProperties: (value: QuoteProperties) => void;
	trailingComma: TrailingComma;
	setTrailingComma: (value: TrailingComma) => void;
	semicolons: Semicolons;
	setSemicolons: (value: Semicolons) => void;
}) {
	return (
		<>
			<h2>Formatter</h2>
			<section>
				<LineWidthInput lineWidth={lineWidth} setLineWidth={setLineWidth} />

				<div className="field-row">
					<label htmlFor="indentStyle">Indent Style</label>
					<select
						id="location"
						name="location"
						value={indentStyle}
						onChange={(e) => {
							setIndentStyle(e.target.value as IndentStyle);
						}}
					>
						<option value={IndentStyle.Tab}>Tabs</option>
						<option value={IndentStyle.Space}>Spaces</option>
					</select>
				</div>

				<div className="field-row">
					<label htmlFor="indentWidth">Indent Width</label>
					<input
						type="number"
						name="indentWidth"
						id="indentWidth"
						value={indentWidth}
						onChange={(e) => {
							setIndentWidth(parseInt(e.target.value));
						}}
					/>
				</div>

				<div className="field-row">
					<label htmlFor="quoteStyle">Quote Style</label>
					<select
						id="quoteStyle"
						name="quoteStyle"
						value={quoteStyle ?? ""}
						onChange={(e) => setQuoteStyle(e.target.value as QuoteStyle)}
					>
						<option value={QuoteStyle.Double}>Double</option>
						<option value={QuoteStyle.Single}>Single</option>
					</select>
				</div>

				<div className="field-row">
					<label htmlFor="quoteProperties">Quote Properties</label>
					<select
						id="quoteProperties"
						name="quoteProperties"
						value={quoteProperties ?? ""}
						onChange={(e) =>
							setQuoteProperties(e.target.value as QuoteProperties)
						}
					>
						<option value={QuoteProperties.AsNeeded}>As needed</option>
						<option value={QuoteProperties.Preserve}>Preserve</option>
					</select>
				</div>

				<div className="field-row">
					<label htmlFor="trailingComma">Trailing Comma</label>
					<select
						id="trailingComma"
						name="trailingComma"
						value={trailingComma ?? "all"}
						onChange={(e) => setTrailingComma(e.target.value as TrailingComma)}
					>
						<option value={TrailingComma.All}>All</option>
						<option value={TrailingComma.ES5}>ES5</option>
						<option value={TrailingComma.None}>None</option>
					</select>
				</div>

				<div className="field-row">
					<label htmlFor="trailingComma">Semicolons</label>
					<select
						id="semicolons"
						name="semicolons"
						value={semicolons ?? "always"}
						onChange={(e) => setSemicolons(e.target.value as Semicolons)}
					>
						<option value={Semicolons.Always}>Always</option>
						<option value={Semicolons.AsNeeded}>As needed</option>
					</select>
				</div>
			</section>
		</>
	);
}

function LinterSettings({
	lintRules,
	setLintRules,
	enabledLinting,
	setEnabledLinting,
}: {
	lintRules: LintRules;
	setLintRules: (value: LintRules) => void;
	enabledLinting: boolean;
	setEnabledLinting: (value: boolean) => void;
}) {
	return (
		<>
			<h2>Linter</h2>
			<section>
				<div className="field-row">
					<input
						id="linting-enabled"
						name="linting-enabled"
						type="checkbox"
						checked={enabledLinting}
						onChange={(e) => setEnabledLinting(e.target.checked)}
					/>
					<label htmlFor="linting-enabled">Linter enabled</label>
				</div>
				<div className="field-row">
					<label htmlFor="trailingComma">Lint Rules</label>
					<select
						id="lint-rules"
						aria-describedby="lint-rules-description"
						name="lint-rules"
						disabled={!enabledLinting}
						value={lintRules ?? LintRules.Recommended}
						onChange={(e) => setLintRules(e.target.value as LintRules)}
					>
						<option value={LintRules.Recommended}>Recommended</option>
						<option value={LintRules.All}>All</option>
					</select>
				</div>
			</section>
		</>
	);
}

function LineWidthInput({
	lineWidth,
	setLineWidth,
}: {
	lineWidth: number;
	setLineWidth: (lineWidth: number) => void;
}) {
	const [showCustom, setShowCustom] = useState(
		lineWidth !== 80 && lineWidth !== 120,
	);

	return (
		<>
			<div className="field-row">
				<label htmlFor="lineWidth">Line Width</label>

				<div className="input-container">
					<div className="button-group">
						<button
							aria-label="Set line width to 80 characters"
							onClick={() => {
								setLineWidth(80);
								setShowCustom(false);
							}}
							onKeyDown={() => {
								setLineWidth(80);
								setShowCustom(false);
							}}
							disabled={!showCustom && lineWidth === 80}
						>
							80
						</button>

						<button
							aria-label="Set line width to 120 characters"
							onClick={() => {
								setLineWidth(120);
								setShowCustom(false);
							}}
							onKeyDown={() => {
								setLineWidth(120);
								setShowCustom(false);
							}}
							disabled={!showCustom && lineWidth === 120}
						>
							120
						</button>

						<button
							aria-label="Set a custom line width"
							onClick={() => setShowCustom(!showCustom)}
							onKeyDown={() => setShowCustom(!showCustom)}
							disabled={showCustom}
						>
							Custom
						</button>
					</div>

					{showCustom && (
						<input
							type="number"
							name="lineWidth"
							id="lineWidth"
							value={lineWidth}
							onChange={(e) => {
								setLineWidth(parseInt(e.target.value));
							}}
						/>
					)}
				</div>
			</div>
		</>
	);
}

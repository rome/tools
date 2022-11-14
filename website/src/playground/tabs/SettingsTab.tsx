import {
	IndentStyle,
	PlaygroundState,
	QuoteProperties,
	QuoteStyle,
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
			enabledNurseryRules,
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
	const setEnabledNurseryRules = createPlaygroundSettingsSetter(
		setPlaygroundState,
		"enabledNurseryRules",
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

	function createFile(filename: string) {
		if (
			// rome-ignore lint(complexity/useSimplifiedLogicExpression): Not sure how else to do this
			!isScriptFilename(filename) &&
			!isJSXFilename(filename) &&
			!isTypeScriptFilename(filename)
		) {
			filename = `${filename}.js`;
		}

		setPlaygroundState((state) => ({
			...state,
			currentFile: filename,
			files: {
				...state.files,
				[filename]: getFileState({ files: {} }, filename),
			},
		}));
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
					setCurrentFile={setCurrentFile}
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
			/>
			<LinterSettings
				enabledNurseryRules={enabledNurseryRules}
				setEnabledNurseryRules={setEnabledNurseryRules}
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
	setCurrentFile,
	files,
}: {
	createFile: (filename: string) => void;
	setCurrentFile: (filename: string) => void;
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
				{files.map((filename, i) => {
					return (
						<FileViewItem
							key={i}
							isActive={filename === currentFile}
							filename={filename}
							onClick={() => {
								setCurrentFile(filename);
							}}
						/>
					);
				})}
			</ul>

			{isCreatingFile && (
				<NewFileInput
					createFile={(filename) => {
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
}: {
	filename: string;
	isActive: boolean;
	onClick: () => void;
}) {
	return (
		<li
			className={classnames(isActive && "active")}
			onClick={onClick}
			onKeyDown={onClick}
		>
			{filename}
		</li>
	);
}

function NewFileInput({
	createFile,
	onCancel,
}: {
	createFile: (filename: string) => void;
	onCancel: () => void;
}) {
	const [filename, setFilename] = useState("");

	function onKeyDown(e: React.KeyboardEvent) {
		if (e.key === "Escape") {
			onCancel();
		}

		if (e.key === "Enter") {
			createFile(filename);
		}
	}

	function onBlur() {
		if (filename === "") {
			onCancel();
		} else {
			createFile(filename);
		}
	}

	function onChange(e: React.ChangeEvent<HTMLInputElement>) {
		setFilename(e.target.value);
	}

	return (
		<input
			type="text"
			// rome-ignore lint(a11y/noAutofocus): Not sure how else to do this
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
			</section>
		</>
	);
}

function LinterSettings({
	enabledNurseryRules,
	setEnabledNurseryRules,
	enabledLinting,
	setEnabledLinting,
}: {
	enabledNurseryRules: boolean;
	setEnabledNurseryRules: (value: boolean) => void;
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
					<input
						id="nursery-rules"
						aria-describedby="nursery-rules-description"
						name="nursery-rules"
						type="checkbox"
						disabled={!enabledLinting}
						checked={enabledNurseryRules}
						onChange={(e) => setEnabledNurseryRules(e.target.checked)}
					/>
					<label htmlFor="nursery-rules">Nursery lint rules</label>
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

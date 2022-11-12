import {
	IndentStyle,
	PlaygroundSettings,
	PlaygroundState,
	QuoteProperties,
	QuoteStyle,
	SourceType,
	TrailingComma,
} from "../types";
import type { Dispatch, SetStateAction } from "react";
import { useState } from "react";
import { createSetter } from "../utils";

interface Props {
	settings: PlaygroundSettings;
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>;
}

export default function SettingsPane({
	setPlaygroundState,
	settings: {
		lineWidth,
		indentWidth,
		indentStyle,
		quoteStyle,
		quoteProperties,
		trailingComma,
		sourceType,
		typescript: isTypeScript,
		jsx: isJsx,
		enabledNurseryRules,
	},
}: Props) {
	const setIsTypeScript = createSetter(setPlaygroundState, "typescript");
	const setIsJsx = createSetter(setPlaygroundState, "jsx");
	const setSourceType = createSetter(setPlaygroundState, "sourceType");
	const setLineWidth = createSetter(setPlaygroundState, "lineWidth");
	const setIndentWidth = createSetter(setPlaygroundState, "indentWidth");
	const setIndentStyle = createSetter(setPlaygroundState, "indentStyle");
	const setQuoteStyle = createSetter(setPlaygroundState, "quoteStyle");
	const setQuoteProperties = createSetter(
		setPlaygroundState,
		"quoteProperties",
	);
	const setTrailingComma = createSetter(setPlaygroundState, "trailingComma");
	const setEnabledNurseryRules = createSetter(
		setPlaygroundState,
		"enabledNurseryRules",
	);

	return (
		<div className="settings-pane">
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
			/>
			<SyntaxSettings
				sourceType={sourceType}
				setSourceType={setSourceType}
				isTypeScript={isTypeScript}
				setIsTypeScript={setIsTypeScript}
				isJsx={isJsx}
				setIsJsx={setIsJsx}
			/>
		</div>
	);
}

function SyntaxSettings({
	sourceType,
	setSourceType,
	isTypeScript,
	setIsTypeScript,
	isJsx,
	setIsJsx,
}: {
	sourceType: SourceType;
	setSourceType: (sourceType: SourceType) => void;
	isTypeScript: boolean;
	setIsTypeScript: (value: boolean) => void;
	isJsx: boolean;
	setIsJsx: (value: boolean) => void;
}) {
	return (
		<>
			<h2>Syntax</h2>
			<section>
				<div className="field-row">
					<label htmlFor="sourceType">Source Type</label>
					<select
						id="sourceType"
						name="sourceType"
						value={sourceType ?? ""}
						onChange={(e) => setSourceType(e.target.value as SourceType)}
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
						checked={isTypeScript}
						onChange={(e) => {
							setIsTypeScript(e.target.checked);
						}}
						disabled={sourceType === SourceType.Script}
					/>
					<label htmlFor="typescript">TypeScript</label>
				</div>

				<div className="field-row">
					<input
						id="jsx"
						name="jsx"
						type="checkbox"
						checked={isJsx}
						onChange={(e) => setIsJsx(e.target.checked)}
						disabled={sourceType === SourceType.Script}
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
}: {
	enabledNurseryRules: boolean;
	setEnabledNurseryRules: (value: boolean) => void;
}) {
	return (
		<>
			<h2>Linter</h2>
			<section>
				<div className="field-row">
					<input
						id="nursery-rules"
						aria-describedby="nursery-rules-description"
						name="nursery-rules"
						type="checkbox"
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

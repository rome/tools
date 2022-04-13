import { PlaygroundResult } from "../pkg";

export enum IndentStyle { Tab = "tab", Space = "space" }
export enum SourceType { Module = "module", Script = "script" }
export enum QuoteStyle { Double = "double", Single = "single" }
export interface PlaygroundProps {
	code: string,
	setCode: (code: string) => void,
	lineWidth: number,
	setLineWidth: (lineWidth: number) => void,
	indentStyle: IndentStyle,
	setIndentStyle: (indentStyle: IndentStyle) => void,
	indentWidth: number,
	setIndentWidth: (indentWidth: number) => void,
	quoteStyle: QuoteStyle,
	setQuoteStyle: (quoteStyle: QuoteStyle) => void,
	sourceType: SourceType,
	setSourceType: (sourceType: SourceType) => void,
	isTypeScript: boolean,
	setIsTypeScript: (isTypeScript: boolean) => void,
	isJsx: boolean,
	setIsJsx: (isJsx: boolean) => void,
	romeOutput: PlaygroundResult,
}

import { RomeOutput } from "../pkg";
import { Dispatch, SetStateAction } from "react";

export enum IndentStyle { Tab = "tab", Space = "space" }
export enum SourceType { Module = "module", Script = "script" }
export enum QuoteStyle { Double = "double", Single = "single" }

export interface PlaygroundState {
	code: string;
	setCode: (code: string) => void;
	lineWidth: number;
	setLineWidth: (lineWidth: number) => void;
	indentStyle: IndentStyle;
	setIndentStyle: (indentStyle: IndentStyle) => void;
	indentWidth: number;
	setIndentWidth: (indentWidth: number) => void;
	quoteStyle: QuoteStyle;
	setQuoteStyle: (quoteStyle: QuoteStyle) => void;
	sourceType: SourceType;
	setSourceType: (sourceType: SourceType) => void;
	isTypeScript: boolean;
	setIsTypeScript: (isTypeScript: boolean) => void;
	isJsx: boolean;
	setIsJsx: (isJsx: boolean) => void;
	treeStyle: TreeStyle;
	setTreeStyle: Dispatch<SetStateAction<TreeStyle>>;
}

export interface PlaygroundProps {
	playgroundState: PlaygroundState;
	prettierOutput: { code: string; ir: object };
	romeOutput: RomeOutput;
}

export type PlaygroundSettings = Pick<
	PlaygroundState,
		| "lineWidth"
		| "setLineWidth"
		| "indentWidth"
		| "setIndentWidth"
		| "indentStyle"
		| "setIndentStyle"
		| "quoteStyle"
		| "setQuoteStyle"
		| "sourceType"
		| "setSourceType"
		| "isTypeScript"
		| "setIsTypeScript"
		| "isJsx"
		| "setIsJsx"
>;

export enum TreeStyle { Json, Text }

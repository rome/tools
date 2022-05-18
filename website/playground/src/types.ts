import { RomeOutput } from "../pkg";
import { Dispatch, SetStateAction } from "react";

export enum IndentStyle { Tab = "tab", Space = "space" }
export enum SourceType { Module = "module", Script = "script" }
export enum QuoteStyle { Double = "double", Single = "single" }

export interface PlaygroundState {
	code: string;
	lineWidth: number;
	indentStyle: IndentStyle;
	indentWidth: number;
	quoteStyle: QuoteStyle;
	sourceType: SourceType;
	isTypeScript: boolean;
	isJsx: boolean;
	treeStyle: TreeStyle;
}

export interface PlaygroundProps {
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>;
	playgroundState: PlaygroundState;
	prettierOutput: { code: string; ir: string };
	romeOutput: RomeOutput;
}

export type PlaygroundSettings = Pick<
	PlaygroundState,
		| "lineWidth"
		| "indentWidth"
		| "indentStyle"
		| "quoteStyle"
		| "sourceType"
		| "isTypeScript"
		| "isJsx"
>;

export enum TreeStyle { Json, Text }

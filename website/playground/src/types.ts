import { Dispatch, SetStateAction } from "react";

export enum IndentStyle { Tab = "tab", Space = "space" }
export enum SourceType { Module = "module", Script = "script" }
export enum QuoteStyle { Double = "double", Single = "single" }
export enum LoadingState { Loading, Success, Error }

export interface RomeOutput {
	ast: string;
	cst: string;
	errors: string;
	formatted_code: string;
	formatter_ir: string;
	control_flow_graph: string;
}

export interface PlaygroundState {
	code: string;
	lineWidth: number;
	indentStyle: IndentStyle;
	indentWidth: number;
	quoteStyle: QuoteStyle;
	sourceType: SourceType;
	isTypeScript: boolean;
	isJsx: boolean;
	cursorPosition: number;
}

// change `lineWidth` and `indentWidth` to string type, just to fits our `usePlaygroundState` fallback usage
export type RomeConfiguration =
	& Omit<PlaygroundState, "code" | "lineWidth" | "indentWidth">
	& { lineWidth: string; indentWidth: string };

export const defaultRomeConfig: RomeConfiguration = {
	lineWidth: "80",
	indentWidth: "2",
	indentStyle: IndentStyle.Tab,
	quoteStyle: QuoteStyle.Double,
	sourceType: SourceType.Module,
	isTypeScript: false,
	isJsx: false,
	cursorPosition: 0,
};

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

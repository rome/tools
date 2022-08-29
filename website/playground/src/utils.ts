import { Dispatch, SetStateAction, useEffect, useState } from "react";
import prettier, { Options } from "prettier";
// @ts-ignore
import parserBabel from "prettier/esm/parser-babel";
import {
	IndentStyle,
	PlaygroundState,
	QuoteStyle,
	QuoteProperties,
	RomeConfiguration,
	SourceType,
} from "./types";

export function classNames(
	...classes: (string | undefined | boolean)[]
): string {
	return classes.filter(Boolean).join(" ");
}
// Define general type for useWindowSize hook, which includes width and height
interface Size {
	width: number | undefined;
	height: number | undefined;
}

// Hook
export function useWindowSize(): Size {
	// Initialize state with undefined width/height so server and client renders match
	// Learn more here: https://joshwcomeau.com/react/the-perils-of-rehydration/
	const [windowSize, setWindowSize] = useState<Size>({
		width: undefined,
		height: undefined,
	});
	useEffect(() => {
		// Handler to call on window resize
		function handleResize() {
			// Set window width/height to state
			setWindowSize({ width: window.innerWidth, height: window.innerHeight });
		}
		// Add event listener
		window.addEventListener("resize", handleResize);
		// Call handler right away so state gets updated with initial window size
		handleResize();
		// Remove event listener on cleanup
		return () => window.removeEventListener("resize", handleResize);
	}, []); // Empty array ensures that effect is only run on mount
	return windowSize;
}

export function usePlaygroundState(
	defaultRomeConfig: RomeConfiguration,
): [PlaygroundState, Dispatch<SetStateAction<PlaygroundState>>] {
	const searchParams = new URLSearchParams(window.location.search);
	const initState = () => ({
		code:
			window.location.hash !== "#"
				? decodeCode(window.location.hash.substring(1))
				: "",
		lineWidth: parseInt(
			searchParams.get("lineWidth") ?? defaultRomeConfig.lineWidth,
		),
		indentStyle:
			(searchParams.get("indentStyle") as IndentStyle) ??
			defaultRomeConfig.indentStyle,
		quoteStyle:
			(searchParams.get("quoteStyle") as QuoteStyle) ??
			defaultRomeConfig.quoteStyle,
		quoteProperties:
			(searchParams.get("quoteProperties") as QuoteProperties) ??
			defaultRomeConfig.quoteProperties,
		indentWidth: parseInt(
			searchParams.get("indentWidth") ?? defaultRomeConfig.indentWidth,
		),
		isTypeScript:
			searchParams.get("typescript") === "true" ||
			defaultRomeConfig.isTypeScript,
		isJsx: searchParams.get("jsx") === "true" || defaultRomeConfig.isJsx,
		sourceType:
			(searchParams.get("sourceType") as SourceType) ??
			defaultRomeConfig.sourceType,
		cursorPosition: 0,
	});
	const [playgroundState, setPlaygroundState] = useState(initState());

	useEffect(() => {
		setPlaygroundState(initState());
	}, [defaultRomeConfig]);

	useEffect(() => {
		const { code, isTypeScript, isJsx } = playgroundState;
		const queryString = new URLSearchParams({
			...crateObjectExcludeKeys(playgroundState, [
				"isTypeScript",
				"isJsx",
				"cursorPosition",
			]),
			typescript: isTypeScript.toString(),
			jsx: isJsx.toString(),
		}).toString();
		const url = `${window.location.protocol}//${window.location.host}${
			window.location.pathname
		}?${queryString}#${encodeCode(code)}`;

		window.history.replaceState({ path: url }, "", url);
	}, [playgroundState]);

	return [playgroundState, setPlaygroundState];
}

export function createSetter(
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>,
	field: keyof PlaygroundState,
) {
	return function (param: PlaygroundState[typeof field]) {
		setPlaygroundState((state) => {
			let nextState = { ...state, [field]: param };
			return nextState;
		});
	};
}

export function formatWithPrettier(
	code: string,
	options: {
		lineWidth: number;
		indentStyle: IndentStyle;
		indentWidth: number;
		language: "js" | "ts";
		quoteStyle: QuoteStyle;
		quoteProperties: QuoteProperties;
	},
): { code: string; ir: string } {
	try {
		const prettierOptions: Options = {
			useTabs: options.indentStyle === IndentStyle.Tab,
			tabWidth: options.indentWidth,
			printWidth: options.lineWidth,
			parser: getPrettierParser(options.language),
			plugins: [parserBabel],
			singleQuote: options.quoteStyle === QuoteStyle.Single,
			quoteProps: options.quoteProperties,
		};

		// @ts-ignore
		let debug = prettier.__debug;
		const document = debug.printToDoc(code, prettierOptions);
		const formattedCode = debug.printDocToString(
			document,
			prettierOptions,
		).formatted;
		const ir = debug.formatDoc(document, {
			parser: "babel",
			plugins: [parserBabel],
		});
		return { code: formattedCode, ir };
	} catch (err: any) {
		console.error(err);
		const code = err.toString();
		return { code, ir: `Error: Invalid code\n${err.message}` };
	}
}

function getPrettierParser(language: "js" | "ts"): string {
	switch (language) {
		case "js":
			return "babel";
		case "ts":
			return "babel-ts";
	}
}

// See https://developer.mozilla.org/en-US/docs/Web/API/btoa#unicode_strings
export function encodeCode(code: string): string {
	return btoa(toBinary(code));
}

export function decodeCode(encoded: string): string {
	return fromBinary(atob(encoded));
}

// convert a Unicode string to a string in which
// each 16-bit unit occupies only one byte
function toBinary(input: string) {
	const codeUnits = new Uint16Array(input.length);
	for (let i = 0; i < codeUnits.length; i++) {
		codeUnits[i] = input.charCodeAt(i);
	}

	const charCodes = new Uint8Array(codeUnits.buffer);
	let result = "";
	for (let i = 0; i < charCodes.byteLength; i++) {
		result += String.fromCharCode(charCodes[i]);
	}
	return result;
}

function fromBinary(binary: string) {
	const bytes = new Uint8Array(binary.length);
	for (let i = 0; i < bytes.length; i++) {
		bytes[i] = binary.charCodeAt(i);
	}
	const charCodes = new Uint16Array(bytes.buffer);
	let result = "";
	for (let i = 0; i < charCodes.length; i++) {
		result += String.fromCharCode(charCodes[i]);
	}
	return result;
}

/**
 * Crate an object with some keys omitted of original object
 * @param obj
 * @param keys
 * @returns
 */
function crateObjectExcludeKeys<T extends object>(
	obj: T,
	keys: Array<keyof T>,
): Record<string, any> {
	return Object.keys(obj).reduce((acc, key) => {
		if (!keys.includes(key as keyof T)) {
			acc[key] = obj[key as keyof T];
		}
		return acc;
	}, Object.create(null));
}

import { Dispatch, SetStateAction, useEffect, useState } from "react";
import prettier from "prettier";
// @ts-ignore
import parserBabel from "prettier/esm/parser-babel";
import {
	defaultRomeConfig,
	IndentStyle,
	PlaygroundState,
	QuoteStyle,
	RomeConfiguration,
	SourceType,
	TreeStyle,
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
	defaultRomeConfig: RomeConfiguration
): [PlaygroundState, Dispatch<SetStateAction<PlaygroundState>>] {
	const searchParams = new URLSearchParams(window.location.search);
	const [playgroundState, setPlaygroundState] = useState(() => ({
		code:
			window.location.hash !== "#"
				? decodeCode(window.location.hash.substring(1))
				: "",
		lineWidth: parseInt(
			searchParams.get("lineWidth") ?? defaultRomeConfig.lineWidth
		),
		indentStyle:
			(searchParams.get("indentStyle") as IndentStyle) ??
			defaultRomeConfig.indentStyle,
		quoteStyle:
			(searchParams.get("quoteStyle") as QuoteStyle) ??
			defaultRomeConfig.quoteStyle,
		indentWidth: parseInt(
			searchParams.get("indentWidth") ?? defaultRomeConfig.indentWidth
		),
		isTypeScript:
			searchParams.get("typescript") === "true" ||
			defaultRomeConfig.isTypeScript,
		isJsx: searchParams.get("jsx") === "true" || defaultRomeConfig.isJsx,
		sourceType:
			(searchParams.get("sourceType") as SourceType) ?? defaultRomeConfig,
		treeStyle: TreeStyle.Json,
	}));

	useEffect(() => {
		const { code, isTypeScript, isJsx, ...options } = playgroundState;
		//@ts-ignore
		const queryString = new URLSearchParams({
			...options,
			typescript: isTypeScript.toString(),
			jsx: isJsx.toString(),
		}).toString();
		const url = `${window.location.protocol}//${window.location.host}${
			window.location.pathname
		}?${queryString}#${encodeCode(code)}`;

		window.history.replaceState({ path: url }, "", url);
	}, [playgroundState]);

	// Only configuration change we would persist, so this will not introduce too much overhead when user changes code
	useEffect(() => {
		changeLocalStorageConfig(playgroundState);
	}, [
		playgroundState.indentStyle,
		playgroundState.indentWidth,
		playgroundState.lineWidth,
		playgroundState.quoteStyle,
		playgroundState.sourceType,
		playgroundState.treeStyle,
		playgroundState.isJsx,
		playgroundState.isTypeScript,
	]);

	return [playgroundState, setPlaygroundState];
}

export function createSetter(
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>,
	field: keyof PlaygroundState
) {
	return function (param: PlaygroundState[typeof field]) {
		setPlaygroundState((state) => ({ ...state, [field]: param }));
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
	}
): { code: string; ir: string } {
	try {
		const prettierOptions = {
			useTabs: options.indentStyle === IndentStyle.Tab,
			tabWidth: options.indentWidth,
			printWidth: options.lineWidth,
			parser: getPrettierParser(options.language),
			plugins: [parserBabel],
			singleQuote: options.quoteStyle === QuoteStyle.Single,
		};

		// @ts-ignore
		let debug = prettier.__debug;
		const document = debug.printToDoc(code, prettierOptions);
		const formattedCode = debug.printDocToString(
			document,
			prettierOptions
		).formatted;
		const ir = debug.formatDoc(document, {
			parser: "babel",
			plugins: [parserBabel],
		});
		return { code: formattedCode, ir };
	} catch (err: any) {
		console.error(err);
		//@ts-ignore
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

export function getLanguage(
	isJsx: boolean,
	isTypeScript: boolean
): "jsx" | "typescript" | "js" {
	if (isTypeScript) {
		return "typescript";
	} else if (isJsx) {
		return "jsx";
	} else {
		return "js";
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

function changeLocalStorageConfig(playgroundState: PlaygroundState) {
	// Excluding `playgroundState.code` reduce the content to save, make `localStorage.setItem` which is file IO a bit faster
	let romeConfig = Object.keys(playgroundState).reduce((acc, key) => {
		if (key !== "code") {
			acc[key] = playgroundState[key as keyof PlaygroundState];
		}
		return acc;
	}, Object.create(null));
	window.localStorage.setItem("rome_config", JSON.stringify(romeConfig));
}

export function loadRomeConfigFromLocalStorage(): RomeConfiguration {
	let configString = window.localStorage.getItem("rome_config");
	console.log(configString)
	let config: Partial<RomeConfiguration> = {};
	try {
		config = JSON.parse(configString || "");
	} catch {
		config = {};
	}
	return mergeRomeConfig(config, defaultRomeConfig);
}

// Change the romeConfig in local storage seems to be meaningless, so we simplify the validation phase.
// only use default value if some key is empty
function mergeRomeConfig(
	config: Partial<RomeConfiguration>,
	defaultConfig: RomeConfiguration
): RomeConfiguration {
	return Object.keys(defaultConfig).reduce((acc, key) => {
		if (config[key as keyof RomeConfiguration]) {
			acc[key] = config[key as keyof RomeConfiguration];
		} else {
			acc[key] = defaultConfig[key as keyof RomeConfiguration];
		}
		return acc;
	},
	Object.create(null)) as RomeConfiguration;
}

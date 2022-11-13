import { Dispatch, SetStateAction, useEffect, useState } from "react";
import prettier, { Options } from "prettier";
import type { ThemeName } from "../frontend-scripts/util";
// @ts-ignore
import parserBabel from "prettier/esm/parser-babel";
import {
	IndentStyle,
	PlaygroundState,
	QuoteStyle,
	QuoteProperties,
	RomeConfiguration,
	SourceType,
	TrailingComma,
} from "./types";
import { getCurrentTheme } from "../frontend-scripts/util";

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

export function useTheme(): ThemeName {
	const [theme, setTheme] = useState(getCurrentTheme());

	useEffect(() => {
		function onColorSchemeChange() {
			setTheme(getCurrentTheme());
		}

		window.addEventListener("colorschemechange", onColorSchemeChange);

		return function cleanup() {
			window.removeEventListener("colorschemechange", onColorSchemeChange);
		};
	});

	return theme;
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

export function createLocalStorage(name: string): {
	set: (value: string | boolean | number) => void;
	get: () => undefined | string;
	getNumber: () => undefined | number;
	getBoolean: () => boolean;
	clear: () => void;
} {
	const key = `playground:${name}`;
	return {
		set: (value) => {
			localStorage.setItem(key, String(value));
		},
		getNumber: () => {
			const elem = localStorage.getItem(key);
			if (elem == null) {
				return undefined;
			} else {
				return Number(elem);
			}
		},
		getBoolean: () => {
			return localStorage.getItem(key) === "true";
		},
		get: () => {
			return localStorage.getItem(key) || undefined;
		},
		clear: () => {
			localStorage.removeItem(key);
		},
	};
}

const lastSearchStore = createLocalStorage("last-search");

export function usePlaygroundState(
	defaultRomeConfig: RomeConfiguration,
): [PlaygroundState, Dispatch<SetStateAction<PlaygroundState>>, () => void] {
	const searchQuery =
		window.location.search === ""
			? lastSearchStore.get() ?? ""
			: window.location.search;
	const initialSearchParams = new URLSearchParams(searchQuery);

	const initState = (searchParams: URLSearchParams) => ({
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
		trailingComma:
			(searchParams.get("trailingComma") as TrailingComma) ??
			defaultRomeConfig.trailingComma,
		indentWidth: parseInt(
			searchParams.get("indentWidth") ?? defaultRomeConfig.indentWidth,
		),
		typescript:
			searchParams.get("typescript") === "true" || defaultRomeConfig.typescript,
		jsx: searchParams.get("jsx") === "true" || defaultRomeConfig.jsx,
		sourceType:
			(searchParams.get("sourceType") as SourceType) ??
			defaultRomeConfig.sourceType,
		cursorPosition: 0,
		enabledNurseryRules:
			searchParams.get("enabledNurseryRules") === "true" ||
			defaultRomeConfig.enabledNurseryRules,
	});
	const [playgroundState, setPlaygroundState] = useState(
		initState(initialSearchParams),
	);

	function resetPlaygroundState() {
		setPlaygroundState(initState(new URLSearchParams("")));
	}

	useEffect(() => {
		setPlaygroundState(initState(initialSearchParams));
	}, [defaultRomeConfig]);

	useEffect(() => {
		const { code } = playgroundState;

		const rawQueryParams: Record<string, unknown> = {
			...playgroundState,
			cursorPosition: undefined,
		};

		if (rawQueryParams.code === "") {
			rawQueryParams.code = undefined;
		}

		// Eliminate default values
		const queryStringObj: Record<string, string> = {};
		for (const key in rawQueryParams) {
			const defaultValue = String(
				defaultRomeConfig[key as keyof typeof defaultRomeConfig],
			);
			const rawValue = rawQueryParams[key];
			const value = String(rawValue);

			if (rawValue !== undefined && value !== defaultValue) {
				queryStringObj[key] = value;
			}
		}

		const queryString = new URLSearchParams(queryStringObj).toString();
		lastSearchStore.set(queryString);

		let url = `${window.location.protocol}//${window.location.host}${window.location.pathname}`;
		if (queryString !== "") {
			url += `?${queryString}#${encodeCode(code)}`;
		}

		window.history.replaceState({ path: url }, "", url);
	}, [playgroundState]);

	return [playgroundState, setPlaygroundState, resetPlaygroundState];
}

export function createSetter<Key extends keyof PlaygroundState>(
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>,
	field: Key,
): (value: PlaygroundState[Key]) => void {
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
		trailingComma: TrailingComma;
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
			trailingComma: options.trailingComma,
		};

		// @ts-ignore
		let debug = prettier.__debug;
		const document = debug.printToDoc(code, prettierOptions);

		// formatDoc must be before printDocToString because printDocToString mutates the document and breaks the ir
		const ir = debug.formatDoc(document, {
			parser: "babel",
			plugins: [parserBabel],
		});
		const formattedCode = debug.printDocToString(
			document,
			prettierOptions,
		).formatted;
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
		result += String.fromCharCode(charCodes[i]!);
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
		result += String.fromCharCode(charCodes[i]!);
	}
	return result;
}

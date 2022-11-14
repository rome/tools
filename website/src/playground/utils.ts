import { Dispatch, SetStateAction, useEffect, useState } from "react";
import prettier, { Options as PrettierOptions } from "prettier";
import type { ThemeName } from "../frontend-scripts/util";
// @ts-ignore
import parserBabel from "prettier/esm/parser-babel";
import {
	IndentStyle,
	PlaygroundState,
	QuoteStyle,
	QuoteProperties,
	TrailingComma,
	defaultPlaygroundState,
	PrettierOutput,
	PlaygroundSettings,
	emptyPrettierOutput,
	emptyRomeOutput,
	PlaygroundFileState,
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

const FILE_QUERY_KEY_REGEX = /^files\.(.*?)$/;

export function usePlaygroundState(): [
	PlaygroundState,
	Dispatch<SetStateAction<PlaygroundState>>,
	() => void,
] {
	const searchQuery =
		window.location.search === ""
			? lastSearchStore.get() ?? ""
			: window.location.search;

	function initState(
		searchParams: URLSearchParams,
		ignoreFiles: boolean,
	): PlaygroundState {
		let singleFileMode = true;
		let hasFiles = false;
		let files: PlaygroundState["files"] = {};

		if (!ignoreFiles) {
			// Populate files
			for (const [key, value] of searchParams) {
				const match = key.match(FILE_QUERY_KEY_REGEX);
				if (match != null) {
					const filename = match[1]!;
					files[filename] = {
						content: decodeCode(value),
						rome: emptyRomeOutput,
						prettier: emptyPrettierOutput,
					};
					singleFileMode = false;
					hasFiles = true;
				}
			}

			// Single file mode
			if (searchParams.get("code")) {
				const ext = getExtension({
					typescript: searchParams.get("typescript") === "true",
					jsx: searchParams.get("jsx") === "true",
					script: searchParams.get("script") === "true",
				});
				files[`main.${ext}`] = {
					content: decodeCode(searchParams.get("code") ?? ""),
					rome: emptyRomeOutput,
					prettier: emptyPrettierOutput,
				};
				hasFiles = true;
			}
		}

		if (!hasFiles) {
			files = defaultPlaygroundState.files;
		}

		return {
			cursorPosition: 0,
			tab: searchParams.get("tab") ?? "formatter",
			singleFileMode,
			currentFile: Object.keys(files)[0] ?? "main.js",
			files,
			settings: {
				lineWidth: parseInt(
					searchParams.get("lineWidth") ??
						String(defaultPlaygroundState.settings.lineWidth),
				),
				indentStyle:
					(searchParams.get("indentStyle") as IndentStyle) ??
					defaultPlaygroundState.settings.indentStyle,
				quoteStyle:
					(searchParams.get("quoteStyle") as QuoteStyle) ??
					defaultPlaygroundState.settings.quoteStyle,
				quoteProperties:
					(searchParams.get("quoteProperties") as QuoteProperties) ??
					defaultPlaygroundState.settings.quoteProperties,
				trailingComma:
					(searchParams.get("trailingComma") as TrailingComma) ??
					defaultPlaygroundState.settings.trailingComma,
				indentWidth: parseInt(
					searchParams.get("indentWidth") ??
						String(defaultPlaygroundState.settings.indentWidth),
				),
				enabledNurseryRules:
					searchParams.get("enabledNurseryRules") === "true" ||
					defaultPlaygroundState.settings.enabledNurseryRules,
				enabledLinting:
					searchParams.get("enabledLinting") === "true" ||
					defaultPlaygroundState.settings.enabledLinting,
			},
		};
	}

	const [playgroundState, setPlaygroundState] = useState(() =>
		initState(new URLSearchParams(searchQuery), window.location.search === ""),
	);

	function resetPlaygroundState() {
		setPlaygroundState(initState(new URLSearchParams(""), false));
	}

	useEffect(() => {
		const rawQueryParams: Record<string, unknown> = {
			...playgroundState.settings,
		};

		// Eliminate default values
		const queryStringObj: Record<string, string> = {};
		for (const key in rawQueryParams) {
			const defaultValue = String(
				defaultPlaygroundState.settings[key as keyof PlaygroundSettings],
			);
			const rawValue = rawQueryParams[key];
			const value = String(rawValue);

			if (rawValue !== undefined && value !== defaultValue) {
				queryStringObj[key] = value;
			}
		}

		if (
			playgroundState.singleFileMode &&
			Object.keys(playgroundState.files).length === 1
		) {
			// Single file mode
			const code = getCurrentCode(playgroundState);
			if (code) {
				queryStringObj.code = encodeCode(code);
			}

			if (!isTypeScriptFilename(playgroundState.currentFile)) {
				queryStringObj.typescript = "false";
			}

			if (!isJSXFilename(playgroundState.currentFile)) {
				queryStringObj.jsx = "false";
			}

			if (isScriptFilename(playgroundState.currentFile)) {
				queryStringObj.script = "true";
			}
		} else {
			// Populate files
			for (const filename in playgroundState.files) {
				const content = playgroundState.files[filename]?.content ?? "";
				queryStringObj[`files.${filename}`] = encodeCode(content);
			}
		}

		const queryString = new URLSearchParams(queryStringObj).toString();
		lastSearchStore.set(queryString);

		let url = `${window.location.protocol}//${window.location.host}${window.location.pathname}`;
		if (queryString !== "") {
			url += `?${queryString}`;
		}

		window.history.replaceState({ path: url }, "", url);
	}, [playgroundState]);

	return [playgroundState, setPlaygroundState, resetPlaygroundState];
}

export function getCurrentCode(state: PlaygroundState): string {
	return state.files[state.currentFile]?.content ?? "";
}

export function getFileState(
	state: Pick<PlaygroundState, "files">,
	filename: string,
): PlaygroundFileState {
	return (
		state.files[filename] ?? {
			content: "",
			rome: emptyRomeOutput,
			prettier: emptyPrettierOutput,
		}
	);
}

export function createPlaygroundSettingsSetter<
	Key extends keyof PlaygroundSettings,
>(
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>,
	field: Key,
): (value: PlaygroundSettings[Key]) => void {
	return function (param: PlaygroundSettings[typeof field]) {
		setPlaygroundState((state) => {
			return {
				...state,
				settings: {
					...state.settings,
					[field]: param,
				},
			};
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
): PrettierOutput {
	try {
		const prettierOptions: PrettierOptions = {
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
		const debug = prettier.__debug;
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

		return {
			type: "SUCCESS",
			code: formattedCode,
			ir,
		};
	} catch (err: any) {
		if (err instanceof SyntaxError) {
			return {
				type: "ERROR",
				stack: err.message,
			};
		} else {
			return {
				type: "ERROR",
				stack: err.stack,
			};
		}
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
	try {
		return fromBinary(atob(encoded));
	} catch {
		return "";
	}
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

export function classnames(...names: (undefined | boolean | string)[]): string {
	let out = "";
	for (const name of names) {
		if (name === undefined || typeof name === "boolean") {
			continue;
		}

		if (out !== "") {
			out += " ";
		}
		out += name;
	}
	return out;
}

export function isTypeScriptFilename(filename: string): boolean {
	return filename.endsWith(".ts") || filename.endsWith(".tsx");
}

export function isJSXFilename(filename: string): boolean {
	return filename.endsWith(".tsx") || filename.endsWith(".jsx");
}

export function isScriptFilename(filename: string): boolean {
	return filename.endsWith(".js");
}

export function modifyFilename(
	filename: string,
	opts: ExtensionOptions,
): string {
	const ext = getExtension(opts);
	const parts = filename.split(".");
	parts.pop();
	parts.push(ext);
	return parts.join(".");
}

type ExtensionOptions = {
	jsx: boolean;
	typescript: boolean;
	script: boolean;
};

export function getExtension(opts: ExtensionOptions): string {
	let ext = "";

	if (opts.script) {
		ext = "js";
	} else {
		ext = "mjs";
	}

	if (opts.typescript) {
		if (opts.jsx) {
			ext = "tsx";
		} else {
			ext = "ts";
		}
	} else if (opts.jsx) {
		ext = "jsx";
	}

	return ext;
}

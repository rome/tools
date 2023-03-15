import type { ThemeName } from "../frontend-scripts/util";
import { getCurrentTheme } from "../frontend-scripts/util";
import {
	PlaygroundFileState,
	PlaygroundSettings,
	PlaygroundState,
	emptyPrettierOutput,
	emptyRomeOutput,
} from "./types";
import { Dispatch, SetStateAction, useEffect, useState } from "react";

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
		return () => {
			window.removeEventListener("resize", handleResize);
		};
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

// See https://developer.mozilla.org/en-US/docs/Web/API/btoa#unicode_strings
export function encodeCode(code: string): string {
	return btoa(toBinary(code));
}

export function decodeCode(encoded: string): string {
	try {
		return fromBinary(atob(encoded));
	} catch {
		return encoded;
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
	return (
		filename.endsWith(".ts") ||
		filename.endsWith(".tsx") ||
		filename.endsWith(".mts") ||
		filename.endsWith(".cts")
	);
}

export function isJSXFilename(filename: string): boolean {
	return filename.endsWith(".tsx") || filename.endsWith(".jsx");
}

export function isScriptFilename(filename: string): boolean {
	return filename.endsWith(".cjs") || filename.endsWith(".cts");
}

export function isModuleFilename(filename: string): boolean {
	return (
		filename.endsWith(".mjs") ||
		filename.endsWith(".mts") ||
		filename.endsWith(".js") ||
		filename.endsWith(".ts")
	);
}

export function isJSONFilename(filename: string): boolean {
	return filename.endsWith(".json");
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
		ext = "cjs";
	} else {
		ext = "js";
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

export function isValidExtension(filename: string): boolean {
	return (
		isScriptFilename(filename) ||
		isModuleFilename(filename) ||
		isTypeScriptFilename(filename) ||
		isJSXFilename(filename) ||
		isJSONFilename(filename)
	);
}

export function normalizeFilename(filename: string): string {
	return isValidExtension(filename) ? filename : `${filename}.js`;
}

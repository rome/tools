import { useEffect, useState, useRef, SetStateAction, Dispatch } from "react";
import {
	defaultPlaygroundState,
	emptyPrettierOutput,
	emptyRomeOutput,
	IndentStyle,
	LoadingState,
	PlaygroundSettings,
	PlaygroundState,
	QuoteProperties,
	QuoteStyle,
	TrailingComma,
	Semicolons,
	LintRules,
} from "./types";
import {
	createLocalStorage,
	decodeCode,
	encodeCode,
	getCurrentCode,
	getExtension,
	getFileState,
	isJSXFilename,
	isScriptFilename,
	isTypeScriptFilename,
	normalizeFilename,
} from "./utils";
import Playground from "./Playground";
import LoadingScreen from "./components/LoadingScreen";

function throttle(callback: () => void): () => void {
	const timeout = setTimeout(callback, 100);

	return () => {
		clearTimeout(timeout);
	};
}

function PlaygroundLoader() {
	const [loadingState, setLoadingState] = useState(LoadingState.Loading);
	const [state, setPlaygroundState, resetPlaygroundState] =
		usePlaygroundState();
	const romeWorkerRef = useRef<Worker | null>(null);
	const prettierWorkerRef = useRef<Worker | null>(null);

	// rome-ignore lint/nursery/useExhaustiveDependencies: dependencies mismatch
	useEffect(() => {
		romeWorkerRef.current = new Worker(
			new URL("./workers/romeWorker", import.meta.url),
			{ type: "module" },
		);
		prettierWorkerRef.current = new Worker(
			new URL("./workers/prettierWorker", import.meta.url),
			{ type: "module" },
		);

		romeWorkerRef.current.addEventListener("message", (event) => {
			switch (event.data.type) {
				case "init": {
					const loadingState = event.data.loadingState as LoadingState;
					setLoadingState(loadingState);
					break;
				}

				case "updated": {
					const { filename, romeOutput } = event.data;
					setPlaygroundState((state) => ({
						...state,
						files: {
							...state.files,
							[filename]: {
								...getFileState(state, filename),
								rome: romeOutput,
							},
						},
					}));
					break;
				}

				default:
					console.error(`Unknown message ${event.data.type}`);
			}
		});

		prettierWorkerRef.current.addEventListener("message", (event) => {
			switch (event.data.type) {
				case "formatted": {
					const { filename, prettierOutput } = event.data;
					setPlaygroundState((state) => ({
						...state,
						files: {
							...state.files,
							[filename]: {
								...getFileState(state, filename),
								prettier: prettierOutput,
							},
						},
					}));
					break;
				}

				default:
					console.error(`Unknown message ${event.data.type}`);
			}
		});

		romeWorkerRef.current?.postMessage({
			type: "init",
		});

		return () => {
			romeWorkerRef.current?.terminate();
			prettierWorkerRef.current?.terminate();
		};
	}, []);

	// Dispatch updated settings
	// rome-ignore lint/nursery/useExhaustiveDependencies: dependencies mismatch
	useEffect(() => {
		if (loadingState !== LoadingState.Success) {
			return;
		}

		return throttle(() => {
			romeWorkerRef.current?.postMessage({
				type: "updateSettings",
				settings: state.settings,
			});

			romeWorkerRef.current?.postMessage({
				type: "update",
				cursorPosition: state.cursorPosition,
				filename: state.currentFile,
				code: getCurrentCode(state),
			});

			prettierWorkerRef.current?.postMessage({
				type: "updateSettings",
				settings: state.settings,
			});

			prettierWorkerRef.current?.postMessage({
				type: "format",
				filename: state.currentFile,
				code: getCurrentCode(state),
			});
		});
	}, [loadingState, state.settings]);

	// Dispatch updated code to Prettier
	// rome-ignore lint/nursery/useExhaustiveDependencies: dependencies mismatch
	useEffect(() => {
		if (loadingState !== LoadingState.Success) {
			return;
		}

		return throttle(() => {
			prettierWorkerRef.current?.postMessage({
				type: "format",
				filename: state.currentFile,
				code: getCurrentCode(state),
			});

			romeWorkerRef.current?.postMessage({
				type: "update",
				cursorPosition: state.cursorPosition,
				filename: state.currentFile,
				code: getCurrentCode(state),
			});
		});
	}, [
		loadingState,
		state.currentFile,
		state.cursorPosition,
		getCurrentCode(state),
	]);

	switch (loadingState) {
		case LoadingState.Error:
			return <div>Error loading. Please refresh</div>;

		case LoadingState.Loading:
			return <LoadingScreen />;

		default:
			return (
				<Playground
					resetPlaygroundState={resetPlaygroundState}
					setPlaygroundState={setPlaygroundState}
					playgroundState={state}
				/>
			);
	}
}

function buildLocation(state: PlaygroundState): string {
	const rawQueryParams: Record<string, unknown> = {
		...state.settings,
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

	if (state.singleFileMode && Object.keys(state.files).length === 1) {
		// Single file mode
		const code = getCurrentCode(state);
		if (code) {
			queryStringObj.code = encodeCode(code);
		}

		if (!isTypeScriptFilename(state.currentFile)) {
			queryStringObj.typescript = "false";
		}

		if (!isJSXFilename(state.currentFile)) {
			queryStringObj.jsx = "false";
		}

		if (isScriptFilename(state.currentFile)) {
			queryStringObj.script = "true";
		}
	} else {
		// Populate files
		for (const filename in state.files) {
			const content = state.files[filename]?.content ?? "";
			queryStringObj[`files.${filename}`] = encodeCode(content);
		}
	}

	const queryString = new URLSearchParams(queryStringObj).toString();
	lastSearchStore.set(queryString);

	let url = `${window.location.protocol}//${window.location.host}${window.location.pathname}`;
	if (queryString !== "") {
		url += `?${queryString}`;
	}
	return url;
}

function initState(
	searchParams: URLSearchParams,
	includeFiles: boolean,
): PlaygroundState {
	let singleFileMode = true;
	let hasFiles = false;
	let files: PlaygroundState["files"] = {};

	if (includeFiles) {
		// Populate files
		for (const [key, value] of searchParams) {
			const match = key.match(FILE_QUERY_KEY_REGEX);
			if (match != null) {
				const filename = normalizeFilename(match[1]!);
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
				typescript: searchParams.get("typescript") !== "false",
				jsx: searchParams.get("jsx") !== "false",
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
			semicolons:
				(searchParams.get("semicolons") as Semicolons) ??
				defaultPlaygroundState.settings.semicolons,
			lintRules:
				(searchParams.get("lintRules") as LintRules) ??
				defaultPlaygroundState.settings.lintRules,
			enabledLinting:
				searchParams.get("enabledLinting") === "true" ||
				defaultPlaygroundState.settings.enabledLinting,
		},
	};
}

const lastSearchStore = createLocalStorage("last-search");

const FILE_QUERY_KEY_REGEX = /^files\.(.*?)$/;

// Safari/Webkit/JSC/whatever only allows setting a URL 50 times within 30 seconds
// set our maximum update frequency just under that to avoid any chance of hitting it
const URL_UPDATE_THROTTLE = 30000 / 40;

export function usePlaygroundState(): [
	PlaygroundState,
	Dispatch<SetStateAction<PlaygroundState>>,
	() => void,
] {
	const [url, setURL] = useState(window.location.toString());

	const [playgroundState, setPlaygroundState] = useState(() => {
		let searchQuery = window.location.search;
		let includeSearchQueryFiles = true;

		// Default to query of last session to load settings
		if (searchQuery === "") {
			searchQuery = lastSearchStore.get() ?? "";
			includeSearchQueryFiles = false;
		}

		return initState(new URLSearchParams(searchQuery), includeSearchQueryFiles);
	});

	function resetPlaygroundState() {
		setPlaygroundState(initState(new URLSearchParams(""), false));
	}

	useEffect(() => {
		setURL(buildLocation(playgroundState));
	}, [playgroundState]);

	// Throttle updating of URL
	useEffect(() => {
		const timeout = setTimeout(() => {
			window.history.replaceState({ path: url }, "", url);
		}, URL_UPDATE_THROTTLE);

		return () => {
			clearTimeout(timeout);
		};
	}, [url]);

	return [playgroundState, setPlaygroundState, resetPlaygroundState];
}

export default PlaygroundLoader;

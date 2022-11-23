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

function App() {
	const [loadingState, setLoadingState] = useState(LoadingState.Loading);
	const [state, setPlaygroundState, resetPlaygroundState] =
		usePlaygroundState();
	const romeWorkerRef = useRef<Worker | null>(null);
	const prettierWorkerRef = useRef<Worker | null>(null);

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

export default App;

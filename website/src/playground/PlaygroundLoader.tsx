import { useEffect, useState, useRef } from "react";
import { LoadingState } from "./types";
import { getCurrentCode, getFileState, usePlaygroundState } from "./utils";
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

			prettierWorkerRef.current?.postMessage({
				type: "updateSettings",
				settings: state.settings,
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

export default App;

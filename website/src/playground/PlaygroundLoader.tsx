import { useEffect, useState, useRef } from "react";
import { LoadingState, RomeOutput } from "./types";
import { defaultRomeConfig } from "./types";
import { usePlaygroundState } from "./utils";
import Playground from "./Playground";
import LoadingScreen from "./components/LoadingScreen";

function App() {
	const [loadingState, setLoadingState] = useState(LoadingState.Loading);
	const [romeConfig, setRomeConfig] = useState(defaultRomeConfig);
	const [playgroundState, setPlaygroundState, resetPlaygroundState] = usePlaygroundState(romeConfig);
	const romeWorkerRef = useRef<Worker | null>(null);
	const prettierWorkerRef = useRef<Worker | null>(null);

	const [romeOutput, setRomeOutput] = useState<RomeOutput>({
		ast: "",
		cst: "",
		errors: "",
		formatted_code: "",
		formatter_ir: "",
		control_flow_graph: "",
	});
	const [prettierOutput, setPrettierOutput] = useState({ code: "", ir: "" });

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
					if (loadingState === LoadingState.Success) {
						setRomeConfig({ ...romeConfig });
					}
					break;
				}

				case "formatted": {
					setRomeOutput(event.data.romeOutput);
					break;
				}

				default:
					console.error(`Unknown message ${event.data.type}`);
			}
		});

		prettierWorkerRef.current.addEventListener("message", (event) => {
			switch (event.data.type) {
				case "formatted": {
					setPrettierOutput(event.data.prettierOutput);
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

	useEffect(() => {
		if (loadingState !== LoadingState.Success) {
			return;
		}

		// Throttle the formatting so that it doesn't run on every keystroke to prevent that the
		// workers are busy formatting outdated code.
		let timeout = setTimeout(() => {
			romeWorkerRef.current?.postMessage({ type: "format", playgroundState });
			prettierWorkerRef.current?.postMessage({
				type: "format",
				playgroundState,
			});
		}, 100);

		return () => {
			clearTimeout(timeout);
		};
	}, [loadingState, playgroundState]);

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
					playgroundState={playgroundState}
					prettierOutput={prettierOutput}
					romeOutput={romeOutput}
				/>
			);
	}
}

export default App;

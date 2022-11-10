import "react-tabs/style/react-tabs.css";
import { useEffect, useState, useRef } from "react";
import { LoadingState, RomeOutput } from "./types";
import { defaultRomeConfig } from "./types";
import { usePlaygroundState, useWindowSize } from "./utils";
import Playground from "./Playground";

function App() {
	const [loadingState, setLoadingState] = useState(LoadingState.Loading);
	const [romeConfig, setRomeConfig] = useState(defaultRomeConfig);
	const [playgroundState, setPlaygroundState] = usePlaygroundState(romeConfig);
	const { width } = useWindowSize();
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
			new URL("./romeWorker", import.meta.url),
			{ type: "module" },
		);
		prettierWorkerRef.current = new Worker(
			new URL("./prettierWorker", import.meta.url),
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

		return () => clearTimeout(timeout);
	}, [loadingState, playgroundState]);

	switch (loadingState) {
		case LoadingState.Error:
			return <div>Error loading. Please refresh</div>;
		case LoadingState.Loading:
			return (
				<div className="h-screen w-screen flex align-center justify-center">
					Loading...
				</div>
			);
		default:
			return (
				<Playground
					setPlaygroundState={setPlaygroundState}
					playgroundState={playgroundState}
					prettierOutput={prettierOutput}
					romeOutput={romeOutput}
				/>
			);
	}
}

export default App;

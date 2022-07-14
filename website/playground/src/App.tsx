import "react-tabs/style/react-tabs.css";
import { useEffect, useState, useRef } from "react";
import { LoadingState, RomeOutput } from "./types";
import { defaultRomeConfig } from "./types";
import {
	loadRomeConfigFromLocalStorage,
	usePlaygroundState,
	useWindowSize,
} from "./utils";
import DesktopPlayground from "./DesktopPlayground";
import { MobilePlayground } from "./MobilePlayground";
import RomeWorker from "./romeWorker?worker";
import PrettierWorker from "./prettierWorker?worker";

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
	});
	const [prettierOutput, setPrettierOutput] = useState({ code: "", ir: "" });

	useEffect(() => {
		romeWorkerRef.current = new RomeWorker();
		prettierWorkerRef.current = new PrettierWorker();

		romeWorkerRef.current.addEventListener("message", (event) => {
			if (event.data.type === "init") {
				const loadingState = event.data.loadingState as LoadingState;
				setLoadingState(loadingState);
				if (loadingState === LoadingState.Success) {
					// We only load the config from local storage once when app is loaded.
					const localStorageRomeConfig = loadRomeConfigFromLocalStorage();
					setRomeConfig({ ...romeConfig, ...localStorageRomeConfig });
				}
			}
			if (event.data.type === "formatted") {
				setRomeOutput(event.data.romeOutput);
			}
		});
		prettierWorkerRef.current.addEventListener("message", (event) => {
			if (event.data.type === "init") {
				setLoadingState(event.data.loadingState as LoadingState);
			}
			if (event.data.type === "formatted") {
				setPrettierOutput(event.data.prettierOutput);
			}
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
		romeWorkerRef.current?.postMessage({ type: "format", playgroundState });
		prettierWorkerRef.current?.postMessage({ type: "format", playgroundState });
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
			if (width && width < 480) {
				return (
					<MobilePlayground
						setPlaygroundState={setPlaygroundState}
						playgroundState={playgroundState}
						prettierOutput={prettierOutput}
						romeOutput={romeOutput}
					/>
				);
			}
			return (
				<DesktopPlayground
					setPlaygroundState={setPlaygroundState}
					playgroundState={playgroundState}
					prettierOutput={prettierOutput}
					romeOutput={romeOutput}
				/>
			);
	}
}

export default App;

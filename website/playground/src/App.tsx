import "react-tabs/style/react-tabs.css";
import { useEffect, useState, useRef } from "react";
import { usePlaygroundState, useWindowSize } from "./utils";
import { LoadingState, RomeOutput } from "./types";
import DesktopPlayground from "./DesktopPlayground";
import { MobilePlayground } from "./MobilePlayground";

function App() {
	const [loadingState, setLoadingState] = useState(LoadingState.Loading);
	const [playgroundState, setPlaygroundState] = usePlaygroundState();
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
		romeWorkerRef.current = new Worker(new URL(
			"./romeWorker.ts",
			import.meta.url,
		), { type: "module" });
		prettierWorkerRef.current = new Worker(new URL(
			"./prettierWorker.ts",
			import.meta.url,
		), { type: "module" });

		romeWorkerRef.current.addEventListener("message", (event) => {
			if (event.data.type === "init") {
				setLoadingState(event.data.loadingState as LoadingState);
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

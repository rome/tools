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
	const workerRef = useRef<Worker | null>(null);

	const [romeOutput, setRomeOutput] = useState<RomeOutput>({
		ast: "",
		cst: "",
		errors: "",
		formatted_code: "",
		formatter_ir: "",
	});
	const [prettierOutput, setPrettierOutput] = useState({ code: "", ir: "" });

	useEffect(() => {
		workerRef.current = new Worker(new URL("./worker.ts", import.meta.url), {
			type: "module",
		});

		workerRef.current.addEventListener("message", (event) => {
			if (event.data.type === "init") {
				setLoadingState(event.data.loadingState as LoadingState);
			}
			if (event.data.type === "formatted") {
				setRomeOutput(event.data.romeOutput);
				setPrettierOutput(event.data.prettierOutput);
			}
		});

		return () => {
			workerRef.current?.terminate();
		};
	}, []);

	useEffect(() => {
		if (loadingState !== LoadingState.Success) {
			return;
		}
		let timeout = setTimeout(() => {
			if (workerRef.current) {
				workerRef.current.postMessage({ type: "format", playgroundState });
			}
		}, 500);
		return () => {
			clearTimeout(timeout);
		};
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

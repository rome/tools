import "react-tabs/style/react-tabs.css";
import init, { run } from "../pkg/rome_playground";
import { useEffect, useState } from "react";
import { IndentStyle, QuoteStyle, SourceType } from "./types";
import { decodeCode, encodeCode, useWindowSize } from "./utils";
import DesktopPlayground from "./DesktopPlayground";
import { MobilePlayground } from "./MobilePlayground";

enum LoadingState {
	Loading,
	Success,
	Error,
}

function App() {
	useEffect(() => {
		init()
			.then(() => {
				setLoadingState(LoadingState.Success);
			})
			.catch(() => {
				setLoadingState(LoadingState.Error);
			});
	}, []);

	const searchParams = new URLSearchParams(window.location.search);
	const [loadingState, setLoadingState] = useState(LoadingState.Loading);
	const [code, setCode] = useState(() =>
		window.location.hash !== "#"
			? decodeCode(window.location.hash.substring(1))
			: ""
	);
	const [lineWidth, setLineWidth] = useState(
		parseInt(searchParams.get("lineWidth") ?? "80")
	);
	const [indentStyle, setIndentStyle] = useState(
		(searchParams.get("indentStyle") as IndentStyle) ?? IndentStyle.Tab
	);
	const [quoteStyle, setQuoteStyle] = useState(
		(searchParams.get("quoteStyle") as QuoteStyle) ?? QuoteStyle.Double
	);
	const [indentWidth, setIndentWidth] = useState(
		parseInt(searchParams.get("indentWidth") ?? "2")
	);
	const [isTypeScript, setIsTypeScript] = useState(
		searchParams.get("typescript") === "true"
	);
	const [isJsx, setIsJsx] = useState(searchParams.get("jsx") === "true");
	const [sourceType, setSourceType] = useState(
		(searchParams.get("sourceType") as SourceType) ?? SourceType.Module
	);

	useEffect(() => {
		const url = `${window.location.protocol}//${window.location.host}${
			window.location.pathname
		}?lineWidth=${lineWidth}&indentStyle=${indentStyle}&quoteStyle=${quoteStyle}&indentWidth=${indentWidth}&typescript=${isTypeScript}&jsx=${isJsx}&sourceType=${sourceType}#${encodeCode(
			code
		)}`;
		window.history.pushState({ path: url }, "", url);
	}, [
		lineWidth,
		indentStyle,
		quoteStyle,
		indentWidth,
		code,
		isTypeScript,
		isJsx,
		sourceType,
	]);
	const { width, height } = useWindowSize();

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
			const romeOutput = run(
				code,
				lineWidth,
				indentStyle === IndentStyle.Space ? indentWidth : undefined,
				quoteStyle,
				isTypeScript,
				isJsx,
				sourceType
			);
			if (width && width < 480) {
				return (
					<MobilePlayground
						isTypeScript={isTypeScript}
						setIsTypeScript={setIsTypeScript}
						isJsx={isJsx}
						setIsJsx={setIsJsx}
						sourceType={sourceType}
						setSourceType={setSourceType}
						indentWidth={indentWidth}
						setIndentWidth={setIndentWidth}
						indentStyle={indentStyle}
						setIndentStyle={setIndentStyle}
						quoteStyle={quoteStyle}
						setQuoteStyle={setQuoteStyle}
						lineWidth={lineWidth}
						setLineWidth={setLineWidth}
						code={code}
						setCode={setCode}
						romeOutput={romeOutput}
					/>
				);
			}
			return (
				<DesktopPlayground
					isTypeScript={isTypeScript}
					setIsTypeScript={setIsTypeScript}
					isJsx={isJsx}
					setIsJsx={setIsJsx}
					sourceType={sourceType}
					setSourceType={setSourceType}
					indentWidth={indentWidth}
					setIndentWidth={setIndentWidth}
					indentStyle={indentStyle}
					setIndentStyle={setIndentStyle}
					quoteStyle={quoteStyle}
					setQuoteStyle={setQuoteStyle}
					lineWidth={lineWidth}
					setLineWidth={setLineWidth}
					code={code}
					setCode={setCode}
					romeOutput={romeOutput}
				/>
			);
	}
}

export default App;

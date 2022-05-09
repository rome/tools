import "react-tabs/style/react-tabs.css";
import init, { run } from "../pkg/rome_playground";
import { useEffect, useState } from "react";
import { IndentStyle, SyntaxTreeRepresentation } from "./types";
import {
	formatWithPrettier,
	usePlaygroundState,
	useSyntaxTreeRepresentationState,
	useWindowSize,
} from "./utils";
import DesktopPlayground from "./DesktopPlayground";
import { MobilePlayground } from "./MobilePlayground";

enum LoadingState { Loading, Success, Error }

function App() {
	useEffect(
		() => {
			init()
				.then(() => {
					setLoadingState(LoadingState.Success);
				})
				.catch(() => {
					setLoadingState(LoadingState.Error);
				});
		},
		[],
	);

	const [loadingState, setLoadingState] = useState(LoadingState.Loading);
	const playgroundState = usePlaygroundState();
	const syntaxTreeRepresentationState = useSyntaxTreeRepresentationState();
	const { width } = useWindowSize();

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
			const {
				code,
				lineWidth,
				indentStyle,
				indentWidth,
				quoteStyle,
				isTypeScript,
				isJsx,
				sourceType,
			} = playgroundState;

			const romeOutput = run(
				code,
				lineWidth,
				indentStyle === IndentStyle.Space ? indentWidth : undefined,
				quoteStyle,
				isTypeScript,
				isJsx,
				sourceType,
				// For now, mobile rawRepresentation will be always false, which will always use json tree representation until we find some better ui switch
				Boolean(
					!(width && width < 480) && syntaxTreeRepresentationState.rawCstRepresentation === SyntaxTreeRepresentation.Raw,
				),
			);
			const prettierOutput = formatWithPrettier(
				code,
				{
					lineWidth,
					indentStyle,
					indentWidth,
					language: isTypeScript ? "ts" : "js",
					quoteStyle,
				},
			);

			if (width && width < 480) {
				return (
					<MobilePlayground
						syntaxTreeRepresentationState={syntaxTreeRepresentationState}
						playgroundState={playgroundState}
						prettierOutput={prettierOutput}
						romeOutput={romeOutput}
					/>
				);
			}
			return (
				<DesktopPlayground
					syntaxTreeRepresentationState={syntaxTreeRepresentationState}
					playgroundState={playgroundState}
					prettierOutput={prettierOutput}
					romeOutput={romeOutput}
				/>
			);
	}
}

export default App;

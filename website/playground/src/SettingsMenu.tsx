import LineWidthInput from "./LineWidthInput";
import IndentStyleSelect from "./IndentStyleSelect";
import QuoteStyleSelect from "./QuoteStyleSelect";
import SourceTypeSelect from "./SourceTypeSelect";
import { PlaygroundSettings, PlaygroundState } from "./types";
import { Dispatch, SetStateAction } from "react";
import { createSetter } from "./utils";
import QuotePropertiesSelect from "./QuotePropertiesSelect";
import NurseryRules from "./NurseryRules";

interface Props {
	settings: PlaygroundSettings;
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>;
}

export function SettingsMenu({
	setPlaygroundState,
	settings: {
		lineWidth,
		indentWidth,
		indentStyle,
		quoteStyle,
		quoteProperties,
		sourceType,
		isTypeScript,
		isJsx,
		enabledNurseryRules
	},
}: Props) {
	return (
		<div>
			<div className="flex flex-col sm:flex-row">
				<LineWidthInput
					lineWidth={lineWidth}
					setLineWidth={createSetter(setPlaygroundState, "lineWidth")}
				/>
				<IndentStyleSelect
					indentWidth={indentWidth}
					setIndentWidth={createSetter(setPlaygroundState, "indentWidth")}
					indentStyle={indentStyle}
					setIndentStyle={createSetter(setPlaygroundState, "indentStyle")}
				/>
			</div>
			<div className="flex flex-col sm:flex-row">
				<QuoteStyleSelect
					quoteStyle={quoteStyle}
					setQuoteStyle={createSetter(setPlaygroundState, "quoteStyle")}
				/>
				<QuotePropertiesSelect
					quoteProperties={quoteProperties}
					setQuoteProperties={createSetter(
						setPlaygroundState,
						"quoteProperties",
					)}
				/>
				<SourceTypeSelect
					isTypeScript={isTypeScript}
					setIsTypeScript={createSetter(setPlaygroundState, "isTypeScript")}
					isJsx={isJsx}
					setIsJsx={createSetter(setPlaygroundState, "isJsx")}
					sourceType={sourceType}
					setSourceType={createSetter(setPlaygroundState, "sourceType")}
				/>
				<NurseryRules setEnabledNurseryRules={createSetter(setPlaygroundState, "enabledNurseryRules")} enabledNurseryRules={enabledNurseryRules} />
			</div>
		</div>
	);
}

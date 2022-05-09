import LineWidthInput from "./LineWidthInput";
import IndentStyleSelect from "./IndentStyleSelect";
import QuoteStyleSelect from "./QuoteStyleSelect";
import SourceTypeSelect from "./SourceTypeSelect";
import { PlaygroundSettings } from "./types";

interface Props {
	settings: PlaygroundSettings;
}

export function SettingsMenu(
	{
		settings: {
			lineWidth,
			setLineWidth,
			indentWidth,
			setIndentWidth,
			indentStyle,
			setIndentStyle,
			quoteStyle,
			setQuoteStyle,
			sourceType,
			setSourceType,
			isTypeScript,
			setIsTypeScript,
			isJsx,
			setIsJsx,
		},
	}: Props,
) {
	return (
		<div>
			<div className="flex flex-col sm:flex-row">
				<LineWidthInput lineWidth={lineWidth} setLineWidth={setLineWidth} />
				<IndentStyleSelect
					indentWidth={indentWidth}
					setIndentWidth={setIndentWidth}
					indentStyle={indentStyle}
					setIndentStyle={setIndentStyle}
				/>
			</div>
			<div className="flex flex-col sm:flex-row">
				<QuoteStyleSelect
					quoteStyle={quoteStyle}
					setQuoteStyle={setQuoteStyle}
				/>
				<SourceTypeSelect
					isTypeScript={isTypeScript}
					setIsTypeScript={setIsTypeScript}
					isJsx={isJsx}
					setIsJsx={setIsJsx}
					sourceType={sourceType}
					setSourceType={setSourceType}
				/>
			</div>
		</div>
	);
}

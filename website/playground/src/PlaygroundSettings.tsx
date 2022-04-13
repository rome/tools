import LineWidthInput from "./LineWidthInput";
import IndentStyleSelect from "./IndentStyleSelect";
import QuoteStyleSelect from "./QuoteStyleSelect";
import SourceTypeSelect from "./SourceTypeSelect";
import { PlaygroundProps } from "./types";

type Props = Pick<
	PlaygroundProps,
	| "lineWidth"
	| "setLineWidth"
	| "indentWidth"
	| "setIndentWidth"
	| "indentStyle"
	| "setIndentStyle"
	| "quoteStyle"
	| "setQuoteStyle"
	| "sourceType"
	| "setSourceType"
	| "isTypeScript"
	| "setIsTypeScript"
	| "isJsx"
	| "setIsJsx"
>;

export function PlaygroundSettings({
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
}: Props) {
	return (
		<div className="flex items-baseline">
			<LineWidthInput lineWidth={lineWidth} setLineWidth={setLineWidth} />
			<IndentStyleSelect
				indentWidth={indentWidth}
				setIndentWidth={setIndentWidth}
				indentStyle={indentStyle}
				setIndentStyle={setIndentStyle}
			/>
			<QuoteStyleSelect quoteStyle={quoteStyle} setQuoteStyle={setQuoteStyle} />
			<SourceTypeSelect
				isTypeScript={isTypeScript}
				setIsTypeScript={setIsTypeScript}
				isJsx={isJsx}
				setIsJsx={setIsJsx}
				sourceType={sourceType}
				setSourceType={setSourceType}
			/>
		</div>
	);
}

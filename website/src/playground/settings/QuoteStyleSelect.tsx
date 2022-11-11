import { QuoteStyle } from "../types";

interface Props {
	setQuoteStyle: (v: QuoteStyle) => void;
	quoteStyle: QuoteStyle;
}

export default function QuoteStyleSelect({ setQuoteStyle, quoteStyle }: Props) {
	return (
		<div>
			<fieldset>
				<legend className="sr-only">Quote Style</legend>
				<div className="relative flex items-start">
					<div className="">
						<label
							htmlFor="quoteStyle"
							className="block"
						>
							Quote Style
						</label>
						<span id="quote-style-description">
							<span className="sr-only">Quote style</span>
						</span>
						<select
							id="quoteStyle"
							aria-describedby="quote-style-description"
							name="quoteStyle"
							value={quoteStyle ?? ""}
							onChange={(e) => setQuoteStyle(e.target.value as QuoteStyle)}
						>
							<option value={QuoteStyle.Double}>Double</option>
							<option value={QuoteStyle.Single}>Single</option>
						</select>
					</div>
				</div>
			</fieldset>
		</div>
	);
}

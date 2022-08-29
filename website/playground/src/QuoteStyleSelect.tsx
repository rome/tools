import { QuoteStyle } from "./types";

interface Props {
	setQuoteStyle: (v: QuoteStyle) => void;
	quoteStyle: QuoteStyle;
}

export default function QuoteStyleSelect({ setQuoteStyle, quoteStyle }: Props) {
	return (
		<div className="pl-5 pb-5">
			<fieldset>
				<legend className="sr-only">Quote Style</legend>
				<div className="relative flex items-start">
					<div className="">
						<label
							htmlFor="quoteStyle"
							className="block text-sm font-medium text-gray-700"
						>
							Quote Style
						</label>
						<span id="quote-style-description" className="text-gray-500">
							<span className="sr-only">Quote style</span>
						</span>
						<select
							id="quoteStyle"
							aria-describedby="quote-style-description"
							name="quoteStyle"
							value={quoteStyle ?? ""}
							onChange={(e) => setQuoteStyle(e.target.value as QuoteStyle)}
							className="w-[100px] mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
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

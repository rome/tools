import { QuoteProperties } from "./types";

interface Props {
	setQuoteProperties: (v: QuoteProperties) => void;
	quoteProperties: QuoteProperties;
}

export default function QuotePropertiesSelect({
	setQuoteProperties,
	quoteProperties,
}: Props) {
	return (
		<div className="pl-5 pb-5">
			<fieldset>
				<legend className="sr-only">Quote Properties</legend>
				<div className="relative flex items-start">
					<div className="">
						<label
							htmlFor="quoteProperties"
							className="block text-sm font-medium text-gray-700"
						>
							Quote Properties
						</label>
						<span id="quote-properties-description" className="text-gray-500">
							<span className="sr-only">Quote Properties</span>
						</span>
						<select
							id="quoteProperties"
							aria-describedby="quote-properties-description"
							name="quoteProperties"
							value={quoteProperties ?? ""}
							onChange={(e) =>
								setQuoteProperties(e.target.value as QuoteProperties)
							}
							className="w-[100px] mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
						>
							<option value={QuoteProperties.AsNeeded}>As needed</option>
							<option value={QuoteProperties.Preserve}>Preserve</option>
						</select>
					</div>
				</div>
			</fieldset>
		</div>
	);
}

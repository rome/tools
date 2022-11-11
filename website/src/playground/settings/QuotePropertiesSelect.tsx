import { QuoteProperties } from "../types";

interface Props {
	setQuoteProperties: (v: QuoteProperties) => void;
	quoteProperties: QuoteProperties;
}

export default function QuotePropertiesSelect({
	setQuoteProperties,
	quoteProperties,
}: Props) {
	return (
		<div>
			<fieldset>
				<legend className="sr-only">Quote Properties</legend>
				<div>
					<div>
						<label
							htmlFor="quoteProperties"
							className="block"
						>
							Quote Properties
						</label>
						<span id="quote-properties-description">
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

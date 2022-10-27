import { TrailingComma } from "./types";

interface Props {
	setTrailingComma: (v: TrailingComma) => void;
	trailingComma: TrailingComma;
}

export default function TrailingCommaSelect({
	setTrailingComma,
	trailingComma,
}: Props) {
	return (
		<div className="pl-5 pb-5">
			<fieldset>
				<legend className="sr-only">Trailing Comma</legend>
				<div className="relative flex items-start">
					<div className="">
						<label
							htmlFor="trailingComma"
							className="block text-sm font-medium text-gray-700"
						>
							Trailing Comma
						</label>
						<span id="quote-style-description" className="text-gray-500">
							<span className="sr-only">Trailing Comma</span>
						</span>
						<select
							id="trailingComma"
							aria-describedby="quote-style-description"
							name="trailingComma"
							value={trailingComma ?? "all"}
							onChange={(e) =>
								setTrailingComma(e.target.value as TrailingComma)
							}
							className="w-[100px] mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
						>
							<option value={TrailingComma.All}>All</option>
							<option value={TrailingComma.ES5}>ES5</option>
							<option value={TrailingComma.None}>None</option>
						</select>
					</div>
				</div>
			</fieldset>
		</div>
	);
}

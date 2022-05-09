import { SyntaxTreeRepresentation } from "./types";

interface Props {
	setSyntaxTreeRepresentation: (v: SyntaxTreeRepresentation) => void;
	representation: SyntaxTreeRepresentation;
}

export default function SyntaxTreeRepresentationSelect(
	{ setSyntaxTreeRepresentation, representation }: Props,
) {
	return (
		<div >
			<fieldset>
				<legend className="sr-only">SyntaxTreeRepresentation</legend>
				<div className="relative flex items-start">
					<div className="flex items-center">
						<label
							htmlFor="quoteStyle"
							className="block text-sm font-medium text-gray-700 mr-3"
						>
							SyntaxTreeRepresentation
						</label>
						<span id="quote-type-description" className="text-gray-500">
							<span className="sr-only">Quote type</span>
						</span>
						<select
							id="quoteStyle"
							aria-describedby="quote-type-description"
							name="quoteStyle"
							value={representation ?? ""}
							onChange={(e) => {
								setSyntaxTreeRepresentation(e.target.value as SyntaxTreeRepresentation);
							}}
							className="w-[100px] mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
						>
							<option value={SyntaxTreeRepresentation.Raw}>raw</option>
							<option value={SyntaxTreeRepresentation.JsonTree}>json-tree</option>
						</select>
					</div>
				</div>
			</fieldset>
		</div>
	);
}

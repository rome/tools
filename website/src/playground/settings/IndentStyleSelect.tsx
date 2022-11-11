import { IndentStyle } from "../types";

interface Props {
	setIndentStyle: (indentStyle: IndentStyle) => void;
	indentStyle: IndentStyle;
	indentWidth: number;
	setIndentWidth: (indentWidth: number) => void;
}

export default function IndentStyleSelect({
	indentStyle,
	setIndentStyle,
	indentWidth,
	setIndentWidth,
}: Props) {
	return (
		<div className="pl-5 pb-5 flex">
			<fieldset className="space-y-5">
				<div>
					<label
						htmlFor="indentStyle"
						className="block"
					>
						Indent Style
					</label>
					<select
						id="location"
						name="location"
						className="w-[100px] mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
						value={indentStyle}
						onChange={(e) => {
							setIndentStyle(e.target.value as IndentStyle);
						}}
					>
						<option value={IndentStyle.Tab}>Tabs</option>
						<option value={IndentStyle.Space}>Spaces</option>
					</select>
				</div>
			</fieldset>
			{indentStyle === IndentStyle.Space && (
				<div className="pl-4">
					<label
						htmlFor="indentWidth"
						className="block"
					>
						Indent Width
					</label>
					<input
						type="number"
						name="indentWidth"
						id="indentWidth"
						className="w-[65px] mt-1 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full border-gray-300 rounded-md"
						value={indentWidth}
						onChange={(e) => {
							setIndentWidth(parseInt(e.target.value));
						}}
					/>
				</div>
			)}
		</div>
	);
}

interface Props {
	lineWidth: number;
	setLineWidth: (lineWidth: number) => void;
}
export default function LineWidthInput({ lineWidth, setLineWidth }: Props) {
	return (
		<div>
			<div>
				<label
					htmlFor="lineWidth"
				>
					Line Width
				</label>
				<div>
					<input
						type="number"
						name="lineWidth"
						id="lineWidth"
						value={lineWidth}
						onChange={(e) => {
							setLineWidth(parseInt(e.target.value));
						}}
					/>
				</div>
			</div>
			<button
				aria-label="Set line width to 80 characters"
				onClick={() => setLineWidth(80)}
				onKeyDown={() => setLineWidth(80)}
				disabled={lineWidth === 80}
			>
				80
			</button>
			<button
				aria-label="Set line width to 120 characters"
				onClick={() => setLineWidth(120)}
				onKeyDown={() => setLineWidth(120)}
				disabled={lineWidth === 120}
			>
				120
			</button>
		</div>
	);
}

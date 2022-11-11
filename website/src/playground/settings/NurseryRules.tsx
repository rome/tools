interface Props {
	setEnabledNurseryRules: (b: boolean) => void;
	enabledNurseryRules: boolean;
}

export default function NurseryRules({
	setEnabledNurseryRules,
	enabledNurseryRules,
}: Props) {
	return (
		<div>
			<fieldset>
				<legend className="sr-only">Linter</legend>
				<div>
					<div>
						<input
							id="nursery-rules"
							aria-describedby="nursery-rules-description"
							name="nursery-rules"
							type="checkbox"
							checked={enabledNurseryRules}
							onChange={(e) => setEnabledNurseryRules(e.target.checked)}
						/>
					</div>
					<div>
						<label
							htmlFor="nursery-rules"
						>
							Nursery lint rules
						</label>
						<span id="nursery-rules-description">
							<span className="sr-only">Nursery lint rules</span>
						</span>
					</div>
				</div>
			</fieldset>
		</div>
	);
}

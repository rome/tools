import { SourceType } from "./types";

interface Props {
	setIsTypeScript: (b: boolean) => void;
	isTypeScript: boolean;
	setIsJsx: (b: boolean) => void;
	isJsx: boolean;
	setSourceType: (v: SourceType) => void;
	sourceType: SourceType;
}

export default function SourceTypeSelect({
	setIsTypeScript,
	isTypeScript,
	setIsJsx,
	isJsx,
	setSourceType,
	sourceType,
}: Props) {
	return (
		<div className="pl-5 pb-5">
			<fieldset className="space-y-5">
				<legend className="sr-only">File Type</legend>

				<div className="relative flex items-start">
					<div className="">
						<label
							htmlFor="sourceType"
							className="block text-sm font-medium text-gray-700"
						>
							Source type:
						</label>
						<span id="source-type-description" className="text-gray-500">
							<span className="sr-only">Source type</span>
						</span>
						<select
							id="sourceType"
							aria-describedby="source-type-description"
							name="sourceType"
							value={sourceType ?? ""}
							onChange={(e) => setSourceType(e.target.value as SourceType)}
							className="w-[100px] mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
						>
							<option value={SourceType.Module}>Module</option>
							<option value={SourceType.Script}>Script</option>
						</select>
					</div>
				</div>

				<div className="relative flex items-start">
					<div className="flex items-center h-5">
						<input
							id="typescript"
							aria-describedby="typescript-description"
							name="typescript"
							type="checkbox"
							checked={isTypeScript}
							onChange={(e) => {
								setIsTypeScript(false);
								setIsJsx(false);
								setIsTypeScript(e.target.checked);
							}}
							className="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded disabled:opacity-30"
							disabled={sourceType == SourceType.Script}
						/>
					</div>
					<div className="ml-3 text-sm">
						<label htmlFor="typescript" className="font-medium text-gray-700">
							TypeScript
						</label>
						<span id="typescript-description" className="text-gray-500">
							<span className="sr-only">TypeScript</span>
						</span>
					</div>
				</div>
				<div className="relative flex items-start">
					<div className="flex items-center h-5">
						<input
							id="jsx"
							aria-describedby="jsx-description"
							name="jsx"
							type="checkbox"
							checked={isJsx}
							onChange={(e) => setIsJsx(e.target.checked)}
							className="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded disabled:opacity-30"
							disabled={sourceType == SourceType.Script}
						/>
					</div>
					<div className="ml-3 text-sm">
						<label htmlFor="jsx" className="font-medium text-gray-700">
							JSX
						</label>
						<span id="jsx-description" className="text-gray-500">
							<span className="sr-only">JSX</span>
						</span>
					</div>
				</div>
			</fieldset>
		</div>
	);
}

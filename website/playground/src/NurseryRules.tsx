import {SourceType} from "./types";

interface Props {
    setEnabledNurseryRules: (b: boolean) => void,
    enabledNurseryRules: boolean
}

export default function NurseryRules({
    setEnabledNurseryRules,
    enabledNurseryRules

                                         }: Props) {
    return (
        <div className="pl-5 pb-5">
            <fieldset className="flex items-center">
                <legend className="sr-only">Linter</legend>
                <div className="relative flex p-5 pl-1 pt-7 pb-0">
                    <div className="flex items-center h-5">
                        <input
                            id="nursery-rules"
                            aria-describedby="jsx-description"
                            name="jsx"
                            type="checkbox"
                            checked={enabledNurseryRules}
                            onChange={(e) => setEnabledNurseryRules(e.target.checked)}
                            className="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded disabled:opacity-30"
                        />
                    </div>
                    <div className="ml-1 text-sm">

                        <label htmlFor="jsx" className="font-medium text-gray-700">
                            Nursery rules enabled
                        </label>
                        <span id="nursery-rules-description" className="text-gray-500">
							<span className="sr-only">Nursery rules enabled</span>
						</span>
                    </div>
                </div>
            </fieldset>
        </div>
    );
}

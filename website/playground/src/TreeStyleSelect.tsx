import { classNames } from "./utils";
import { TreeStyle } from "./types";
import { Dispatch, SetStateAction } from "react";

interface Props {
	treeStyle: TreeStyle;
	setTreeStyle: (treeStyle: TreeStyle) => void;
}

export default function TreeStyleSelect({ treeStyle, setTreeStyle }: Props) {
	return (
		<div className="group p-0.5 rounded-lg flex bg-gray-200 mb-4 m-2 w-fit">
			<button
				type="button"
				onClick={() => setTreeStyle(TreeStyle.Json)}
				className={classNames(
					"flex focus-visible:ring-2 focus-visible:ring-teal-500 focus-visible:ring-offset-2 rounded-md focus:outline-none focus-visible:ring-offset-gray-100",
					treeStyle === TreeStyle.Json &&
						"bg-white shadow-sm ring-1 ring-black ring-opacity-5"
				)}
			>
				<span
					className={classNames(
						"p-1.5 lg:pl-2.5 lg:pr-3.5 rounded-md flex items-center text-sm font-medium",
						treeStyle === TreeStyle.Json &&
							"bg-white shadow-sm ring-1 ring-black ring-opacity-5"
					)}
				>
					<span
						className={classNames(
							"text-gray-900",
							treeStyle === TreeStyle.Json
								? "text-gray-900"
								: "text-gray-600 group-hover:text-gray-900"
						)}
					>
						JSON
					</span>
				</span>
			</button>
			<button
				type="button"
				onClick={() => setTreeStyle(TreeStyle.Text)}
				className={classNames(
					"ml-0.5 p-1.5 lg:pl-2.5 lg:pr-3.5 rounded-md flex items-center text-sm text-gray-600 font-medium focus-visible:ring-2 focus-visible:ring-teal-500 focus-visible:ring-offset-2 focus:outline-none focus-visible:ring-offset-gray-100",
					treeStyle === TreeStyle.Text &&
						"bg-white shadow-sm ring-1 ring-black ring-opacity-5"
				)}
			>
				<span className="text-gray-900">Text</span>
			</button>
		</div>
	);
}

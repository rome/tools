import { SetStateAction } from "react";
import { Dispatch } from "react";
import ReactJson from "react-json-view";
import TreeStyleSelect from "./TreeStyleSelect";
import { PlaygroundState, TreeStyle } from "./types";
import { createSetter } from "./utils";

interface Props {
	tree: string;
	treeStyle: TreeStyle;
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>;
}

export default function TreeView(
	{ tree, treeStyle, setPlaygroundState }: Props,
) {
	return (
		<div className="overflow-scroll">
			<TreeStyleSelect
				treeStyle={treeStyle}
				setTreeStyle={createSetter(setPlaygroundState, "treeStyle")}
			/>
			{treeStyle === TreeStyle.Json ? (
				<ReactJson src={JSON.parse(tree)} />
			) : (
				<pre>{tree}</pre>
			)}
		</div>
	);
}

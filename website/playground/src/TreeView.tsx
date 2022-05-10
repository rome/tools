//@ts-ignore
import ReactJson from "react-json-view";
import TreeStyleSelect from "./TreeStyleSelect";
import { TreeStyle } from "./types";

interface Props {
	tree: object;
	treeStyle: TreeStyle;
	setTreeStyle: (treeStyle: TreeStyle) => void;
}

export default function TreeView({ tree, treeStyle, setTreeStyle }: Props) {
	return (
		<div className="overflow-scroll">
			<TreeStyleSelect treeStyle={treeStyle} setTreeStyle={setTreeStyle} />
			{treeStyle === TreeStyle.Json ? (
				<ReactJson src={tree} />
			) : (
				<pre>{tree}</pre>
			)}
		</div>
	);
}

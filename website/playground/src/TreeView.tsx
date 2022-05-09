//@ts-ignore
import ReactJson from "react-json-view";
import TreeStyleSelect from "./TreeStyleSelect";
import { Dispatch, SetStateAction } from "react";
import { TreeStyle } from "./types";

interface Props {
	tree: object;
	treeStyle: TreeStyle;
	setTreeStyle: Dispatch<SetStateAction<TreeStyle>>;
}

export default function TreeView({ tree, treeStyle, setTreeStyle }: Props) {
	return (
		<div className="overflow-scroll">
			<TreeStyleSelect treeStyle={treeStyle} setTreeStyle={setTreeStyle} />
			{treeStyle === TreeStyle.Json ? (
				<ReactJson style={{ zIndex: "10 " }} src={tree} />
			) : (
				<pre>{tree}</pre>
			)}
		</div>
	);
}

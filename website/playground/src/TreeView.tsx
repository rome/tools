//@ts-ignore
import ReactJson from "react-json-view";
import TreeStyleSelect from "./TreeStyleSelect";

interface Props { tree: object }

export default function TreeView({ tree }: Props) {
	return (
		<div className="overflow-scroll">
			<TreeStyleSelect />
			<ReactJson style={{ zIndex: "10 " }} src={tree} />
		</div>
	);
}

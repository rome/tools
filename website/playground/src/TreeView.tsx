//@ts-ignore
import ReactJson from "react-json-view";

interface Props { tree: object }

export default function TreeView({ tree }: Props) {
	return (
		<div className="overflow-scroll">
			<ReactJson src={tree} />
		</div>
	);
}

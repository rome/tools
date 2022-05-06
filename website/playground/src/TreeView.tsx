//@ts-ignore
import ReactJson from "react-json-view";

interface Props { tree: string }

export default function TreeView({ tree }: Props) {
	return (
		<div className="overflow-scroll">
			<ReactJson src={JSON.parse(tree)} />
		</div>
	);
}

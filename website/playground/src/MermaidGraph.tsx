import { memo } from "react";

interface MermaidGraphProps {
	graph: string;
}

export default memo(function MermaidGraph({ graph }: MermaidGraphProps) {
	if (graph === "") {
		return null;
	}

	const encodedGraph = encodeURIComponent(btoa(graph));

	return (
		<iframe
			className="h-screen w-full"
			src={`/mermaid.html?graph=${encodedGraph}`}
		/>
	);
});

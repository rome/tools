import { memo } from "react";

interface Props {
	graph: string;
}

const MermaidGraph = memo(function MermaidGraph({ graph }: Props) {
	const encodedGraph = encodeURIComponent(btoa(graph));

	return <iframe src={`/playground/mermaid?graph=${encodedGraph}`} />;
});

export default function ControlFlowTab({graph}: Props) {
  if (graph === "") {
    return <span>No control flow graph present</span>;
  }
  
  return <MermaidGraph graph={graph} />;
}
